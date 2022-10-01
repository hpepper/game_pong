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

## XXX

* https://crates.io/crates/bevy_debug_text_overlay
