use crate::domain::category::Category;
use crate::infrastructure::connection_mongodb::ConnectionMongodb;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use mongodb::sync::Collection;
use std::io::{stdout, Write};

pub fn create_category() {
    execute!(stdout(), Clear(ClearType::All)).unwrap(); //borrar terminal

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

    //crear Category

    //titulo
    let title = read_data("Titulo de la Categoria: ");

    let category = Category::new(title);

    //escribir en mongodb
    let my_coll: Collection<Category> = client.database("belador_db").collection("category");
    let insert_one_result = my_coll.insert_one(category).run().unwrap();
    println!(
        "Inserted document with _id: {}",
        insert_one_result.inserted_id
    );
}

fn read_data(question: &str) -> String {
    println!("{}", question);
    stdout().flush().unwrap(); // necesario para que el print aparezca antes de leer
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}
