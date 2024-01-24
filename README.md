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
    
## Bracket-Lib and Bracket-Terminal
bracket-lib is Rust game programming library. Its designed as a simplified learning tool abstracting away more complicated aspects of game development while retaining concepts required for complex games. It includes a family of libraries including random number generations, geometry , path-finding, color-handling and common game-development algorithms.
bracket-terminal is the display portion of bracket-lib. It provides an emulated console. It can work with various rendering from text consoles to web assembly including OpenGL, Vulkan and Metal.

## Storing Game State
The game loop runs by calling your application's tick() function with every frame. The tick() function does not know anything about our game. So, we need to store the current status of our game which is referred to as the Game State. Everything we need to know to preserve between frames is stored in the Game State. The state represents the snapshot of the current state of the game. Bracket-lib defines a trait for the games state structures known as GameState. GameState requires that the object implement a tick() function.
