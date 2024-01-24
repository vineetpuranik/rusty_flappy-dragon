# Flappy Dragon
Building a flappy dragon game in Rust.

## Setup instructions

## Known errors that might occur during setup
1. Error cmake is not installed.
2. Error pkg-config is not installed
3. Error fontconfig is not installed.
After installing these applications cargo clean and run cargo build again.
    
## Gameloop
For games to operate smoothly they run in a Game loop. The Game loop initializes windowing, graphics and other resources. Subsequently, it runs every time often more than 30 to 60 times per second.
Each time it calls a tick() function.
    
