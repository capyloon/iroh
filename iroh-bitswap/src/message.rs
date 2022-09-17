use core::convert::TryFrom;

use ahash::AHashMap;
use anyhow::{Context, Result};
use bytes::Bytes;
use cid::Cid;
use multihash::{Code, MultihashDigest};
use once_cell::sync::Lazy;
use prost::Message;

use crate::block::Block;
use crate::prefix::Prefix;

mod pb {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/bitswap_pb.rs"));
}

/// The maximum size a single entry inside a wantlist can have.
static MAX_ENTRY_SIZE: Lazy<usize> = Lazy::new(|| {
    let cid = Cid::new_v0(Code::Sha2_256.digest(b"cid")).unwrap();
    let entry = Entry {
        cid,
        priority: i32::MAX,
        want_type: WantType::Have,
        send_dont_have: true,
        cancel: true,
    };
    entry.encoded_len()
});

/// Represents a HAVE / DONT_HAVE for a given Cid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockPresence {
    pub cid: Cid,
    pub typ: BlockPresenceType,
}

impl BlockPresence {
    pub fn encoded_len(&self) -> usize {
        let bpm: pb::message::BlockPresence = self.clone().into();
        bpm.encoded_len()
    }

    pub fn encoded_len_for_cid(cid: Cid) -> usize {
        pb::message::BlockPresence {
            cid: cid.to_bytes(),
            r#type: BlockPresenceType::Have.into(),
        }
        .encoded_len()
    }
}

impl From<BlockPresence> for pb::message::BlockPresence {
    fn from(bp: BlockPresence) -> Self {
        pb::message::BlockPresence {
            cid: bp.cid.to_bytes(),
            r#type: bp.typ.into(),
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(i32)]
pub enum BlockPresenceType {
    Have = 0,
    DontHave = 1,
}

impl From<BlockPresenceType> for pb::message::BlockPresenceType {
    fn from(ty: BlockPresenceType) -> Self {
        match ty {
            BlockPresenceType::Have => pb::message::BlockPresenceType::Have,
            BlockPresenceType::DontHave => pb::message::BlockPresenceType::DontHave,
        }
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(i32)]
pub enum WantType {
    Block = 0,
    Have = 1,
}

impl From<WantType> for pb::message::wantlist::WantType {
    fn from(want: WantType) -> Self {
        match want {
            WantType::Block => pb::message::wantlist::WantType::Block,
            WantType::Have => pb::message::wantlist::WantType::Have,
        }
    }
}

// A wantlist entry in a Bitswap message, with flags indicating
// - whether message is a cancel
// - whether requester wants a DONT_HAVE message
// - whether requester wants a HAVE message (instead of the block)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub cid: Cid,
    pub priority: Priority,
    pub want_type: WantType,
    pub cancel: bool,
    pub send_dont_have: bool,
}

impl Entry {
    /// Returns the encoded length of this entry.
    pub fn encoded_len(&self) -> usize {
        let pb: pb::message::wantlist::Entry = self.into();
        pb.encoded_len()
    }
}

impl From<&Entry> for pb::message::wantlist::Entry {
    fn from(e: &Entry) -> Self {
        pb::message::wantlist::Entry {
            block: e.cid.to_bytes(),
            priority: e.priority,
            want_type: e.want_type.into(),
            cancel: e.cancel,
            send_dont_have: e.send_dont_have,
        }
    }
}

/// Priority of a wanted block.
pub type Priority = i32;

/// A bitswap message.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct BitswapMessage {
    full: bool,
    wantlist: AHashMap<Cid, Entry>,
    blocks: AHashMap<Cid, Block>,
    block_presences: AHashMap<Cid, BlockPresenceType>,
    pending_bytes: i32,
}

impl BitswapMessage {
    pub fn new(full: bool) -> Self {
        BitswapMessage {
            full,
            ..Default::default()
        }
    }

    /// Clears all contents of this message for it to be reused.
    pub fn clear(&mut self, full: bool) {
        self.full = full;
        self.wantlist.clear();
        self.blocks.clear();
        self.block_presences.clear();
        self.pending_bytes = 0;
    }

    pub fn full(&self) -> bool {
        self.full
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty() && self.wantlist.is_empty() && self.block_presences.is_empty()
    }

    pub fn wantlist(&self) -> impl Iterator<Item = &Entry> {
        self.wantlist.values()
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.values()
    }

    pub fn block_presences(&self) -> impl Iterator<Item = BlockPresence> + '_ {
        self.block_presences.iter().map(|(cid, typ)| BlockPresence {
            cid: *cid,
            typ: *typ,
        })
    }

    pub fn haves(&self) -> impl Iterator<Item = &Cid> {
        self.get_block_presence_by_type(BlockPresenceType::Have)
    }

    pub fn dont_haves(&self) -> impl Iterator<Item = &Cid> {
        self.get_block_presence_by_type(BlockPresenceType::DontHave)
    }

    fn get_block_presence_by_type(&self, typ: BlockPresenceType) -> impl Iterator<Item = &Cid> {
        self.block_presences
            .iter()
            .filter_map(move |(cid, t)| if *t == typ { Some(cid) } else { None })
    }

    pub fn pending_bytes(&self) -> i32 {
        self.pending_bytes
    }

    pub fn set_pending_bytes(&mut self, bytes: i32) {
        self.pending_bytes = bytes;
    }

    pub fn remove(&mut self, cid: &Cid) {
        self.wantlist.remove(cid);
    }

    pub fn cancel(&mut self, cid: Cid) {
        self.add_full_entry(cid, 0, true, WantType::Block, false);
    }

    pub fn add_entry(
        &mut self,
        cid: Cid,
        priority: Priority,
        want_type: WantType,
        send_dont_have: bool,
    ) {
        self.add_full_entry(cid, priority, false, want_type, send_dont_have);
    }

    fn add_full_entry(
        &mut self,
        cid: Cid,
        priority: Priority,
        cancel: bool,
        want_type: WantType,
        send_dont_have: bool,
    ) -> usize {
        if let Some(entry) = self.wantlist.get_mut(&cid) {
            // only change priority if want is of the same type
            if entry.want_type == want_type {
                entry.priority = priority;
            }

            // only change from dont cancel to cancel
            if cancel {
                entry.cancel = cancel;
            }

            // only change from dont send to do send DONT_HAVE
            if send_dont_have {
                entry.send_dont_have = send_dont_have;
            }

            // want block overrides existing want have
            if want_type == WantType::Block && entry.want_type == WantType::Have {
                entry.want_type = want_type;
            }

            return 0;
        }

        let entry = Entry {
            cid,
            priority,
            want_type,
            send_dont_have,
            cancel,
        };
        let size = entry.encoded_len();
        self.wantlist.insert(cid, entry);
        size
    }

    pub fn add_block(&mut self, block: Block) {
        self.block_presences.remove(block.cid());
        self.blocks.insert(*block.cid(), block);
    }

    pub fn add_block_presence(&mut self, cid: Cid, typ: BlockPresenceType) {
        if self.blocks.contains_key(&cid) {
            return;
        }
        self.block_presences.insert(cid, typ);
    }

    pub fn add_have(&mut self, cid: Cid) {
        self.add_block_presence(cid, BlockPresenceType::Have);
    }

    pub fn add_dont_have(&mut self, cid: Cid) {
        self.add_block_presence(cid, BlockPresenceType::DontHave);
    }

    pub fn encoded_len(&self) -> usize {
        let block_size: usize = self.blocks.values().map(|b| b.data.len()).sum();
        let block_presence_size: usize = self.block_presences().map(|bp| bp.encoded_len()).sum();

        let wantlist_size: usize = self.wantlist.values().map(|e| e.encoded_len()).sum();

        block_size + block_presence_size + wantlist_size
    }

    pub fn encode_as_proto_v0(&self) -> pb::Message {
        let mut message = pb::Message::default();

        // wantlist
        let mut wantlist = pb::message::Wantlist::default();
        for entry in self.wantlist.values() {
            wantlist.entries.push(entry.into());
        }
        wantlist.full = self.full;
        message.wantlist = Some(wantlist);

        // blocks
        for block in self.blocks.values() {
            message.blocks.push(block.data().clone());
        }

        message
    }

    pub fn encode_as_proto_v1(&self) -> pb::Message {
        let mut message = pb::Message::default();

        // wantlist
        let mut wantlist = pb::message::Wantlist::default();
        for entry in self.wantlist.values() {
            wantlist.entries.push(entry.into());
        }
        wantlist.full = self.full;
        message.wantlist = Some(wantlist);

        // blocks
        for block in self.blocks.values() {
            message.payload.push(pb::message::Block {
                prefix: Prefix::from(block.cid()).to_bytes(),
                data: block.data().clone(),
            });
        }

        // block presences
        for (cid, typ) in &self.block_presences {
            message.block_presences.push(pb::message::BlockPresence {
                cid: cid.to_bytes(),
                r#type: (*typ).into(),
            });
        }

        message.pending_bytes = self.pending_bytes();

        message
    }
}

impl TryFrom<pb::Message> for BitswapMessage {
    type Error = anyhow::Error;

    fn try_from(pbm: pb::Message) -> Result<Self, Self::Error> {
        let full = pbm.wantlist.as_ref().map(|w| w.full).unwrap_or_default();
        let mut message = BitswapMessage::new(full);

        if let Some(wantlist) = pbm.wantlist {
            for entry in wantlist.entries {
                let cid = Cid::try_from(entry.block).context("invalid cid")?;
                message.add_full_entry(
                    cid,
                    entry.priority,
                    entry.cancel,
                    entry.want_type.try_into()?,
                    entry.send_dont_have,
                );
            }
        }

        // deprecated
        for data in pbm.blocks {
            // CID v0, SHA26
            let block = Block::from_v0_data(data)?;
            message.add_block(block);
        }

        for block in pbm.payload {
            let prefix = Prefix::new(&block.prefix)?;
            let cid = prefix.to_cid(&block.data)?;
            let block = Block::new(block.data, cid);
            message.add_block(block);
        }

        for block_presence in pbm.block_presences {
            let cid = Cid::try_from(block_presence.cid).context("invalid cid")?;
            message.add_block_presence(cid, block_presence.r#type.try_into()?);
        }

        message.pending_bytes = pbm.pending_bytes;

        Ok(message)
    }
}

impl TryFrom<Bytes> for BitswapMessage {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let pbm = pb::Message::decode(value)?;
        pbm.try_into()
    }
}
