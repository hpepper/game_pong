# Bevy documentation

Coordinate system

See: https://bevy-cheatbook.github.io/features/coords.html

* 2D the 0,0 is a the center of the screen.

## Resources

Resource: a shared global piece of data[EcsGuide](https://github.com/bevyengine/bevy/blob/v0.7.0/examples/ecs/ecs_guide.rs)

* https://bevyengine.org/learn/book/getting-started/resources/
* https://bevy-cheatbook.github.io/programming/res.html

Resources allow you to store a single global instance of some data type, independently of entities[BCheat](https://bevy-cheatbook.github.io/programming/res.html).
Use them for data that is truly global for your app, such as configuration / settings[BCheat](https://bevy-cheatbook.github.io/programming/res.html).

Resources are accessed in much the same way that we access components.

* app.insert_resource()

### Timers

* https://bevy-cheatbook.github.io/features/time.html

## WASM

* [Unofficial Bevy Cheat Book - WebAssembly](https://bevy-cheatbook.github.io/platforms/wasm.html)
* [wasm-bind source](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)
* [wasm optimization(speed)](https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html)
* [Bevy in the Browser!](https://deepstacker.com/2021-07-08-bevy-in-the-browser/)

1. sudo apt install binaryen
2. update the [Cargo.toml](https://bevy-cheatbook.github.io/platforms/wasm/panic-console.html)
3. rustup target install wasm32-unknown-unknown
4. cargo install -f wasm-bindgen-cli
5. cargo build --release --target wasm32-unknown-unknown --target-dir ~/wasm_target/
6. wasm-bindgen --out-dir ~/wasm_out/ --target web ~/wasm_target/wasm32-unknown-unknown/release/pong.wasm
7. mv ~/wasm_out/pong_bg.wasm ~/wasm_out/org_pong_bg.wasm
8. wasm-opt -O3 -o ~/wasm_out/pong_bg.wasm ~/wasm_out/org_pong_bg.wasm
9. cp index.html ~/wasm_out
10. cp bevy.ico ~/wasm_out
11. cd ~/wasm_out
12. python -m http.server

## XXX

* https://crates.io/crates/bevy_debug_text_overlay
