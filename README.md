# The Shadow Of The West

## Describe

  >Shadow of the West is a real-time strategy game written in Rust on Amethyst game engine.
  
  Game is steampunk variation of Wild West with 3 fractions: Native Americans
  
  

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.
