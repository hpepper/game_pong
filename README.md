# Pong game in Bevy

## Introduction

### References

* https://bevyengine.org/learn/book/getting-started/
* https://bevy-cheatbook.github.io/

### Vocabulary

* ECS - Entity Component System
* Startup systems are just like normal systems, but they run exactly once, before all other systems, right when our app starts[Startup system](https://bevyengine.org/learn/book/getting-started/ecs/).

## Troubleshooting

### compile

#### alsa lib missing

See: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

* sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev

### Ittereation 1

1. Create game scene
2. add and move two paddles, independently
3. add ball and handle colision detection
4. bounce ball, on paddle
5. bounce ball against top and bottom boundary
6. handle ball going behind backline(give one point to other player/team)

* Support four players, smaller paddles, and they are on different tracks
* Support single player?
