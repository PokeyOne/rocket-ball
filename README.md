# Rocket Ball

This is a small test-project for the Bevy game-engine to get comfortable with
working with engine and to see the state of game-engines in Rust.

Essentially there is a ball on the screen that is affected by gravity, and
it will bounce off the walls. The user may also press buttons to activate
thrusters side-to-side and up-and-down.

That's it.

## Running

To run the game from source (the only option to run the game), you will have to
have Rust installed, then follow steps below:

1. Clone this repo
1. Run `cargo run`
1. The game will be running
1. Press escape or the close button to exit the game.

## Project Status

- [x] Ball sprite that is affected by gravity
- [x] Ball collides with bottom of window
- [x] Collision boundaries adapt reactively with window size changes
- [ ] Collide with all sides
- [ ] Button presses and rocket propulsion
- [ ] Rocket animation when on
