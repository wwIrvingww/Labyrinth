# Labyrinth
=========

Labyrinth is an engaging maze-solving game that allows you to navigate through intricate labyrinths. The game provides a dynamic experience by offering both 2D and 3D rendering modes, which can be toggled with the press of a key.

Features
--------

-   **2D and 3D Render Modes:** Switch between 2D and 3D views using the 'M' key on your keyboard or the 'B/Circle' button on a gamepad.
-   **Gamepad Support:** Fully integrated gamepad support for a more immersive experience.
-   **Randomized Sprites:** The game features randomly appearing ghosts in the 3D mode, adding an element of surprise.
-   **Minimap:** Navigate more easily with a handy minimap that shows your location within the maze.
-   **Audio:** Enjoy an immersive experience with background music that starts playing when you begin the labyrinth.

How to Play
-----------

1.  Launch the game.
2.  From the main menu, select a maze to play.
3.  Use the 'ASDW' keys or the gamepad to move your character through the maze.
4.  Press the 'M' key to switch between 2D and 3D modes.
5.  Reach the end of the maze to succeed and return to the menu.

Installation
------------

1.  Clone this repository:

    ```
    git clone https://github.com/wwIrvingww/Labyrinth.git
    ```

2.  Install the required dependencies:

    ```
    cargo build
    ```

3.  Run the game:

    ```
    cargo run
    ```

Dependencies
------------

This project utilizes several Rust crates:

-   `rand`: For generating random numbers.
-   `minifb`: For creating a minimal frame buffer window.
-   `nalgebra`: For linear algebra operations.
-   `image`: For loading and manipulating images.
-   `gilrs`: For gamepad support.
-   `rodio`: For audio playback.
