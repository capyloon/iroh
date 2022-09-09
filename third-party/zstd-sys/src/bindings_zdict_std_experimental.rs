/*
This file is auto-generated from the public API of the zstd library.
It is released under the same BSD license.

BSD License

For Zstandard software

Copyright (c) 2016-present, Facebook, Inc. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

 * Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

 * Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

 * Neither the name Facebook nor the names of its contributors may be used to
   endorse or promote products derived from this software without specific
   prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* automatically generated by rust-bindgen 0.59.2 */

pub const ZDICT_DICTSIZE_MIN: u32 = 256;
pub const ZDICT_CONTENTSIZE_MIN: u32 = 128;
extern "C" {
    #[doc = " ZDICT_trainFromBuffer():"]
    #[doc = "  Train a dictionary from an array of samples."]
    #[doc = "  Redirect towards ZDICT_optimizeTrainFromBuffer_fastCover() single-threaded, with d=8, steps=4,"]
    #[doc = "  f=20, and accel=1."]
    #[doc = "  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,"]
    #[doc = "  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order."]
    #[doc = "  The resulting dictionary will be saved into `dictBuffer`."]
    #[doc = " @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)"]
    #[doc = "          or an error code, which can be tested with ZDICT_isError()."]
    #[doc = "  Note:  Dictionary training will fail if there are not enough samples to construct a"]
    #[doc = "         dictionary, or if most of the samples are too small (< 8 bytes being the lower limit)."]
    #[doc = "         If dictionary training fails, you should use zstd without a dictionary, as the dictionary"]
    #[doc = "         would've been ineffective anyways. If you believe your samples would benefit from a dictionary"]
    #[doc = "         please open an issue with details, and we can look into it."]
    #[doc = "  Note: ZDICT_trainFromBuffer()'s memory usage is about 6 MB."]
    #[doc = "  Tips: In general, a reasonable dictionary has a size of ~ 100 KB."]
    #[doc = "        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`."]
    #[doc = "        In general, it's recommended to provide a few thousands samples, though this can vary a lot."]
    #[doc = "        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    pub fn ZDICT_trainFromBuffer(
        dictBuffer: *mut ::core::ffi::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_params_t {
    pub compressionLevel: ::std::os::raw::c_int,
    pub notificationLevel: ::std::os::raw::c_uint,
    pub dictID: ::std::os::raw::c_uint,
}
extern "C" {
    #[doc = " ZDICT_finalizeDictionary():"]
    #[doc = " Given a custom content as a basis for dictionary, and a set of samples,"]
    #[doc = " finalize dictionary by adding headers and statistics according to the zstd"]
    #[doc = " dictionary format."]
    #[doc = ""]
    #[doc = " Samples must be stored concatenated in a flat buffer `samplesBuffer`,"]
    #[doc = " supplied with an array of sizes `samplesSizes`, providing the size of each"]
    #[doc = " sample in order. The samples are used to construct the statistics, so they"]
    #[doc = " should be representative of what you will compress with this dictionary."]
    #[doc = ""]
    #[doc = " The compression level can be set in `parameters`. You should pass the"]
    #[doc = " compression level you expect to use in production. The statistics for each"]
    #[doc = " compression level differ, so tuning the dictionary for the compression level"]
    #[doc = " can help quite a bit."]
    #[doc = ""]
    #[doc = " You can set an explicit dictionary ID in `parameters`, or allow us to pick"]
    #[doc = " a random dictionary ID for you, but we can't guarantee no collisions."]
    #[doc = ""]
    #[doc = " The dstDictBuffer and the dictContent may overlap, and the content will be"]
    #[doc = " appended to the end of the header. If the header + the content doesn't fit in"]
    #[doc = " maxDictSize the beginning of the content is truncated to make room, since it"]
    #[doc = " is presumed that the most profitable content is at the end of the dictionary,"]
    #[doc = " since that is the cheapest to reference."]
    #[doc = ""]
    #[doc = " `maxDictSize` must be >= max(dictContentSize, ZSTD_DICTSIZE_MIN)."]
    #[doc = ""]
    #[doc = " @return: size of dictionary stored into `dstDictBuffer` (<= `maxDictSize`),"]
    #[doc = "          or an error code, which can be tested by ZDICT_isError()."]
    #[doc = " Note: ZDICT_finalizeDictionary() will push notifications into stderr if"]
    #[doc = "       instructed to, using notificationLevel>0."]
    #[doc = " NOTE: This function currently may fail in several edge cases including:"]
    #[doc = "         * Not enough samples"]
    #[doc = "         * Samples are uncompressible"]
    #[doc = "         * Samples are all exactly the same"]
    pub fn ZDICT_finalizeDictionary(
        dstDictBuffer: *mut ::core::ffi::c_void,
        maxDictSize: usize,
        dictContent: *const ::core::ffi::c_void,
        dictContentSize: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
        parameters: ZDICT_params_t,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_getDictID(
        dictBuffer: *const ::core::ffi::c_void,
        dictSize: usize,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ZDICT_getDictHeaderSize(
        dictBuffer: *const ::core::ffi::c_void,
        dictSize: usize,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_isError(errorCode: usize) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn ZDICT_getErrorName(
        errorCode: usize,
    ) -> *const ::std::os::raw::c_char;
}
#[doc = " ZDICT_cover_params_t:"]
#[doc = "  k and d are the only required parameters."]
#[doc = "  For others, value 0 means default."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_cover_params_t {
    pub k: ::std::os::raw::c_uint,
    pub d: ::std::os::raw::c_uint,
    pub steps: ::std::os::raw::c_uint,
    pub nbThreads: ::std::os::raw::c_uint,
    pub splitPoint: f64,
    pub shrinkDict: ::std::os::raw::c_uint,
    pub shrinkDictMaxRegression: ::std::os::raw::c_uint,
    pub zParams: ZDICT_params_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_fastCover_params_t {
    pub k: ::std::os::raw::c_uint,
    pub d: ::std::os::raw::c_uint,
    pub f: ::std::os::raw::c_uint,
    pub steps: ::std::os::raw::c_uint,
    pub nbThreads: ::std::os::raw::c_uint,
    pub splitPoint: f64,
    pub accel: ::std::os::raw::c_uint,
    pub shrinkDict: ::std::os::raw::c_uint,
    pub shrinkDictMaxRegression: ::std::os::raw::c_uint,
    pub zParams: ZDICT_params_t,
}
extern "C" {
    #[doc = " ZDICT_trainFromBuffer_cover():"]
    #[doc = "  Train a dictionary from an array of samples using the COVER algorithm."]
    #[doc = "  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,"]
    #[doc = "  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order."]
    #[doc = "  The resulting dictionary will be saved into `dictBuffer`."]
    #[doc = " @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)"]
    #[doc = "          or an error code, which can be tested with ZDICT_isError()."]
    #[doc = "          See ZDICT_trainFromBuffer() for details on failure modes."]
    #[doc = "  Note: ZDICT_trainFromBuffer_cover() requires about 9 bytes of memory for each input byte."]
    #[doc = "  Tips: In general, a reasonable dictionary has a size of ~ 100 KB."]
    #[doc = "        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`."]
    #[doc = "        In general, it's recommended to provide a few thousands samples, though this can vary a lot."]
    #[doc = "        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    pub fn ZDICT_trainFromBuffer_cover(
        dictBuffer: *mut ::core::ffi::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
        parameters: ZDICT_cover_params_t,
    ) -> usize;
}
extern "C" {
    #[doc = " ZDICT_optimizeTrainFromBuffer_cover():"]
    #[doc = " The same requirements as above hold for all the parameters except `parameters`."]
    #[doc = " This function tries many parameter combinations and picks the best parameters."]
    #[doc = " `*parameters` is filled with the best parameters found,"]
    #[doc = " dictionary constructed with those parameters is stored in `dictBuffer`."]
    #[doc = ""]
    #[doc = " All of the parameters d, k, steps are optional."]
    #[doc = " If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}."]
    #[doc = " if steps is zero it defaults to its default value."]
    #[doc = " If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000]."]
    #[doc = ""]
    #[doc = " @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)"]
    #[doc = "          or an error code, which can be tested with ZDICT_isError()."]
    #[doc = "          On success `*parameters` contains the parameters selected."]
    #[doc = "          See ZDICT_trainFromBuffer() for details on failure modes."]
    #[doc = " Note: ZDICT_optimizeTrainFromBuffer_cover() requires about 8 bytes of memory for each input byte and additionally another 5 bytes of memory for each byte of memory for each thread."]
    pub fn ZDICT_optimizeTrainFromBuffer_cover(
        dictBuffer: *mut ::core::ffi::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
        parameters: *mut ZDICT_cover_params_t,
    ) -> usize;
}
extern "C" {
    #[doc = " ZDICT_trainFromBuffer_fastCover():"]
    #[doc = "  Train a dictionary from an array of samples using a modified version of COVER algorithm."]
    #[doc = "  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,"]
    #[doc = "  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order."]
    #[doc = "  d and k are required."]
    #[doc = "  All other parameters are optional, will use default values if not provided"]
    #[doc = "  The resulting dictionary will be saved into `dictBuffer`."]
    #[doc = " @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)"]
    #[doc = "          or an error code, which can be tested with ZDICT_isError()."]
    #[doc = "          See ZDICT_trainFromBuffer() for details on failure modes."]
    #[doc = "  Note: ZDICT_trainFromBuffer_fastCover() requires 6 * 2^f bytes of memory."]
    #[doc = "  Tips: In general, a reasonable dictionary has a size of ~ 100 KB."]
    #[doc = "        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`."]
    #[doc = "        In general, it's recommended to provide a few thousands samples, though this can vary a lot."]
    #[doc = "        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    pub fn ZDICT_trainFromBuffer_fastCover(
        dictBuffer: *mut ::core::ffi::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
        parameters: ZDICT_fastCover_params_t,
    ) -> usize;
}
extern "C" {
    #[doc = " ZDICT_optimizeTrainFromBuffer_fastCover():"]
    #[doc = " The same requirements as above hold for all the parameters except `parameters`."]
    #[doc = " This function tries many parameter combinations (specifically, k and d combinations)"]
    #[doc = " and picks the best parameters. `*parameters` is filled with the best parameters found,"]
    #[doc = " dictionary constructed with those parameters is stored in `dictBuffer`."]
    #[doc = " All of the parameters d, k, steps, f, and accel are optional."]
    #[doc = " If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}."]
    #[doc = " if steps is zero it defaults to its default value."]
    #[doc = " If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000]."]
    #[doc = " If f is zero, default value of 20 is used."]
    #[doc = " If accel is zero, default value of 1 is used."]
    #[doc = ""]
    #[doc = " @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)"]
    #[doc = "          or an error code, which can be tested with ZDICT_isError()."]
    #[doc = "          On success `*parameters` contains the parameters selected."]
    #[doc = "          See ZDICT_trainFromBuffer() for details on failure modes."]
    #[doc = " Note: ZDICT_optimizeTrainFromBuffer_fastCover() requires about 6 * 2^f bytes of memory for each thread."]
    pub fn ZDICT_optimizeTrainFromBuffer_fastCover(
        dictBuffer: *mut ::core::ffi::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
        parameters: *mut ZDICT_fastCover_params_t,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: ::std::os::raw::c_uint,
    pub zParams: ZDICT_params_t,
}
extern "C" {
    #[doc = " ZDICT_trainFromBuffer_legacy():"]
    #[doc = "  Train a dictionary from an array of samples."]
    #[doc = "  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,"]
    #[doc = "  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order."]
    #[doc = "  The resulting dictionary will be saved into `dictBuffer`."]
    #[doc = " `parameters` is optional and can be provided with values set to 0 to mean \"default\"."]
    #[doc = " @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)"]
    #[doc = "          or an error code, which can be tested with ZDICT_isError()."]
    #[doc = "          See ZDICT_trainFromBuffer() for details on failure modes."]
    #[doc = "  Tips: In general, a reasonable dictionary has a size of ~ 100 KB."]
    #[doc = "        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`."]
    #[doc = "        In general, it's recommended to provide a few thousands samples, though this can vary a lot."]
    #[doc = "        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    #[doc = "  Note: ZDICT_trainFromBuffer_legacy() will send notifications into stderr if instructed to, using notificationLevel>0."]
    pub fn ZDICT_trainFromBuffer_legacy(
        dictBuffer: *mut ::core::ffi::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
        parameters: ZDICT_legacy_params_t,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_addEntropyTablesFromBuffer(
        dictBuffer: *mut ::core::ffi::c_void,
        dictContentSize: usize,
        dictBufferCapacity: usize,
        samplesBuffer: *const ::core::ffi::c_void,
        samplesSizes: *const usize,
        nbSamples: ::std::os::raw::c_uint,
    ) -> usize;
}
