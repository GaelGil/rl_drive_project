# A reinforcement learning project

# Overview
I wanted to learn rust and the basics of reinforcement learning so I decided to create this project. It is a asteroids game where you are a player and asteroids are coming at you that you can shoot. There is a score and rounds. Each round the asteroid frequency is higher.


# Running Just the Game
Make sure you have rust installed. You can go [here](https://rust-lang.org/tools/install/)

Once you have it installed follow these steps

```
cd rl_drive_project
```

Run the game

```
cargo run -p asteroids_client
```


Run the local server
```
uv run python -m http.server 8000 --directory web
```
