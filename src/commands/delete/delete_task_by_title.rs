use crate::domain::task::Task;
use crate::infrastructure::connection_mongodb::ConnectionMongodb;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use mongodb::bson::doc;
use mongodb::sync::Collection;
use std::io::{stdout, Write};

pub fn delete_task_by_title() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    let connection_mongodb = ConnectionMongodb::new();

    let client = match connection_mongodb.connection() {
        Ok(c) => {
            println!("mongodb conectado...");
            c
        }
        Err(e) => {
            eprintln!("mongodb error al conectar {e}");
            return;
        }
    };

    //titulo
    let title = read_data("Titulo de la tarea a eliminar: ");

    //eliminar tarea
    let my_coll: Collection<Task> = client.database("belador_db").collection("task");
    let delete_result = my_coll.delete_one(doc! { "title": &title }).run().unwrap();
    println!("Deleted document with _id: {}", delete_result.deleted_count);
}

fn read_data(question: &str) -> String {
    println!("{}", question);
    stdout().flush().unwrap(); // necesario para que el print aparezca antes de leer
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}
