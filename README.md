# iroh

Iroh is a next-generation implementation of the Interplanetary File System ([IPFS](https://ipfs.io)) for Cloud & Mobile platforms.

IPFS is a networking protocol for exchanging _content-addressed_ blocks of immutable data. “Content-addressed” means referring to data by the *hash of its content*, which makes the reference unique and verifiable. These two properties make it possible to get data from *any* node in the network that speaks the IPFS protocol, including IPFS content being served by other implementations of IPFS.

This repo is a common core for three distributions of iroh:

- **Iroh Cloud:** core features of iroh split into configurable microservices, optimized for running at datacenter scale.
- **Iroh One:** A select set of iroh cloud features packaged as a single binary for simplified deployment.
- **Iroh Mobile:** iOS & Android libraries that bring efficient data distribution to mobile apps.

## Project Status: Early Days

Iroh has yet to publish a release. We are targeting the end of October 2022 for an initial version, which will coincide with the launch of a proper web site & documentation. Before iroh's first release we're in build-from-source and read-source-to-understand-how-it-works territory.

In the meantime, there's a [quickstart guide](./quickstart.md) if you'd like to get a feel for running an iroh cloud gateway.

## Working on Iroh
Check out the [CONTRIBUTOR docs](./CONTRIBUTOR.md) to get familiar with ways you can contribute to the Iroh project. The [DEVELOPERS docs](./DEVELOPERS.md) will help you get starting with building and developing Iroh.

## Benchmarks

A full suite of automated benchmarks is in the works. [this talk](https://www.youtube.com/watch?v=qPBR2K2X6cs&t=161s) goes into some early numbers.

## Who's behind this?

Iroh is built & maintained by [number 0](https://n0.computer). We're a founder-backed startup hell-bent on building efficient distributed systems software.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br />

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
