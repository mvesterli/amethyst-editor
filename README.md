# Amethyst Electron Editor

[![Join us on Discord](https://img.shields.io/discord/425678876929163284.svg?logo=discord)](https://discord.gg/GnP5Whs)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](COPYING.txt)

An experimental editor/visualizer/debugger for the [Amethyst] game engine, built using [Electron].

## Setup

Use the [amethyst-editor-sync] crate to add support for visualizing your game's state in the editor.

## Status

This project is *highly* experimental. It implements basic functionality for visualizing the
current state of an Amethyst game. It will work with any component that implements the
[`Serialize`] trait (including both user-defined components and ones built into the engine), but
the display is still crude and needs work to display data in a user-friedly way.

## Roadmap/Wishlist

* Application logic written in Rust, likely with [Yew].
* The ability to edit the state of the world (add/remove entities, add/remove/edit components,
  edit resources).
* The ability to create/edit prefabs.
* Preview assets (meshes, textures, prefabs, etc.) in the editor.

[Amethyst]: https://www.amethyst.rs/
[Electron]: https://electronjs.org/
[amethyst-editor-sync]: https://github.com/randomPoison/amethyst-editor-sync
[`Serialize`]: https://docs.rs/serde/1.0.76/serde/trait.Serialize.html
[Yew]: https://github.com/DenisKolodin/yew
