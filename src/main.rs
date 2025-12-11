mod application;
mod commands;
mod domain;
mod infrastructure;
use crate::domain::command::Command;

//cargo run -- -command --arg1 --arg2

fn main() {
    //skip(1) para saltar el primer argumento (el nombre del programa)
    //tener un vector de strings con los argumentos

    let values: Vec<String> = std::env::args().skip(1).collect();

    let command = values
        .first()
        .filter(|s| s.starts_with('-'))
        .cloned()
        .unwrap();

    let args = &values[1..]; //--arg1 --arg2

    let command = Command::new(command.clone(), args.to_vec()); //-command

    //println!("Command: {:?}", command);

    command.resolve().run();
}
