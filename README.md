# A reinforcement learning project

# Overview
I wanted to learn rust and the basics of reinforcement learning so I decided to create this project. It is a asteroids game where you are a player and asteroids are coming at you that you can shoot. There is a score and rounds. Each round the asteroid frequency is higher.


# Running Just the Game
Make sure you have rust installed. You can go [here](https://rust-lang.org/tools/install/)

Once you have it installed follow these steps

```
cd rl_drive_project
```

if on Mac

```
RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup" cargo run
```

if on Linux

```
cargo run
```
