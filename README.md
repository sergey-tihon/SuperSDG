# SuperSDG 3: The Maze Game

Rust edition of my very first maze game initially created in Pascal
then [ported to 3d (C++ & OpenGL)](https://sergeytihon.com/2013/03/16/supersdg2-the-maze-game-с-opengl/).

## Development

Run locally in the native window:

```shell
cargo run
```

Run with development tooling:

```shell
cargo run --features dev_native
```

Set `SUPERSDG_SCHEDULE_SEED` to reproduce `Update` schedule ordering:

```shell
SUPERSDG_SCHEDULE_SEED=1 cargo run --features dev_native
```

Menus use focused Bevy buttons: the first item receives focus when a menu opens; arrow/WASD navigation wraps, mouse hover moves focus, and Enter, Space, or click activates the focused item. `Escape` or `Q` pauses in game and exits from the native start menu. `F1` toggles help, `F3` cycles render-debug modes, and `F4` cycles opacity. macOS may reserve function keys for system shortcuts; use `Fn` when needed.

Run locally in the browser:

```shell
trunk serve
```

## Bevy Learning Materials

- [Bevy](https://bevyengine.org/)
- [Official Bevy Docs](https://docs.rs/bevy/latest/bevy/)
- [Bevy game development](https://taintedcoders.com) & [Bevy Starter](https://github.com/nolantait/bevy-starter/)
- [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/tutorial.html)
- [Making Games in Rust Series' Articles](https://dev.to/sbelzile/series/16367)

### Assets

- [Kenney's Assets](https://kenney.nl/assets)
- <https://github.com/camsjams/rust-cruncher-munchers/blob/main/src/main.rs>
