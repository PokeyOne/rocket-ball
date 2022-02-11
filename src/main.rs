use bevy::prelude::*;

fn say_hello() {
    println!("Hello, World!");
}

fn main() {
    App::new()
        .add_system(say_hello)
        .run();
}
