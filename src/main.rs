use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    let names: Vec<String> = vec!["Bob", "Rob", "Dob"].into_iter().map(|s| s.to_string()).collect();

    for name in names {
        commands.spawn().insert(Person).insert(Name(name));
    }
}

fn say_hello(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}", name.0);
    }
}

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(say_hello)
        .run();
}
