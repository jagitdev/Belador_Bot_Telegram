mod application;
mod domain;
use crate::domain::command::Command;

//cargo run -- -command --arg1 --arg2

fn main() {
    //skip(1) para saltar el primer argumento (el nombre del programa)
    //tener un vector de strings con los argumentos

    let args: Vec<String> = std::env::args().skip(1).collect();

    let mode = args
        .first()
        .filter(|s| s.starts_with('-'))
        .cloned()
        .unwrap();

    let rest = &args[1..]; //--arg1 --arg2

    let command = Command::new(mode.clone(), rest.to_vec()); //-command

    println!("Command: {:?}", command);

    command.resolve().run();
}
