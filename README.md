# Exodus

A Re-Implementation of the
game [Space Exodus](https://web.archive.org/web/20010609173820/http://www.davidsansome.co.uk/pages/psion/exodus/index.htm)
, originally programmed by [David Sansome](http://www.davidsansome.com/) for Psion EPOC-based handheld computers.
This game is a stand-alone cross-platform project implemented in [Rust](https://www.rust-lang.org/) using
the [Bevy](https://bevyengine.org/) engine.

The original EPOC game can be downloaded under the following links:

Psion Revo | Psion 5mx
------------|-----------
[Download](https://archive.org/details/tucows_55899_Space_Exodus_Revo_version) | [Download](https://archive.org/details/tucows_45515_Space_Exodus)

## Compilation Instructions

To compile the game, you need to set up a [Rust Toolchain](https://www.rust-lang.org/learn/get-started) on your
computer, running an operating system supported by
the [Bevy Game Engine](https://bevy-cheatbook.github.io/platforms.html).

First, you need to update your Rust version to the latest stable release using `rustup`, by
typing `rustup update stable`.
Then, you can build the game by running `cargo build` and `cargo run` in the cloned repository path.

As soon as the game is ready for a first release, I will provide pre-compiled artifacts here which can be run
effortlessly without installing any additional software.
