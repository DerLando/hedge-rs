# hedge

[![Build Status](https://travis-ci.org/photex/hedge.svg?branch=master)](https://travis-ci.org/photex/hedge)
[![Documentation](https://docs.rs/hedge/badge.svg)](https://docs.rs/hedge)
[![Version](https://img.shields.io/crates/v/hedge.svg)](https://crates.io/crates/hedge)
[![License](https://img.shields.io/crates/l/hedge.svg)](https://github.com/photex/hedge/blob/master/LICENSE)

Indexed based [half-edge] mesh implementation.

Many of the ideas and techniques used in this crate are inspired by [Petgraph] and [OpenMesh].

Early work has begun to enable building meshes for Amethyst. Enable this using the 'amethyst' feature and
specify the graphics backend as well. Sucks to have such spangly feature sprawl but for the moment
that's the only way I know how to get where I need go here.

`--features=amethyst,vulkan`, `--features=amethyst,metal`, or `--features=amethyst,empty`

[half-edge]: https://en.wikipedia.org/wiki/Doubly_connected_edge_list
[Petgraph]: http://crates.io/crates/petgraph
[OpenMesh]: http://openmesh.org
