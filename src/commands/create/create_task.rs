use crate::domain::subtask::SubTask;
use crate::domain::task::Task;
use crate::infrastructure::connection_mongodb::ConnectionMongodb;
use chrono::Local;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::sync::{Client, Collection};
use std::io::{stdout, Write};
use std::process;

pub fn create_task() {
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

    //crear Task
    println!("Creando Tarea");

    //titulo
    let title = read_data("Titulo de la Tarea: ");

    //descripcion
    let description = read_data("Descripción de la tarea: ");

    //hora
    let hours_create = Local::now().format("%H:%M").to_string();

    //fecha
    let date_create = Local::now().format("%d-%m-%Y").to_string();

    //categoria
    //
    let category = read_category(&client);

    //fecha limite
    let date_limit = read_date();

    //progreso
    let progress = read_progress();

    //subtareas
    let sub_tasks = read_subtask();

    let task = Task::new(
        title,
        description,
        hours_create,
        date_create,
        category,
        date_limit,
        progress,
        sub_tasks,
    );

    //escribir en mongodb
    let my_coll: Collection<Task> = client.database("belador_db").collection("task");
    let insert_one_result = my_coll.insert_one(task).run().unwrap();
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

fn read_progress() -> String {
    loop {
        println!("Progreso de la tarea(0%...100%): ");
        stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input: u32 = input.trim().parse().unwrap();

        match input {
            0..=100 => {
                return input.to_string();
            }
            _ => println!("Progreso debe estar entre 0 y 100"),
        }
    }
}

fn read_subtask() -> Option<Vec<SubTask>> {
    //preguntar hasta que sea un signo de no hay mas tareas

    /*

    --subtareas--
    1º Subtarea (añade un + si quieres añadir subtareas y añade un ! para no añadir mas subtareas
    +
    Titulo de la subtarea:
    ..
    Descripcion de la subtarea:
    ..
     */
    println!("--Subtareas--");
    let mut count = 1;

    let mut vec_subtask: Vec<SubTask> = Vec::new();

    loop {
        println!("Subtarea Añade un + si quieres añadir subtareas y añade un ! para no añadir mas subtareas");
        stdout().flush().unwrap();
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        let value_option = value.trim();

        if value_option == "+" {
            //si es + el valor es añadir una subtarea mas
            println!("{count}º Subtarea");

            //titulo subtarea
            println!("Titulo de la subtarea: ");
            stdout().flush().unwrap();
            let mut titulo = String::new();
            std::io::stdin().read_line(&mut titulo).unwrap();
            titulo = titulo.trim().to_string();

            //descripcion de la subtarea
            println!("Descripcion de la subtarea: ");
            stdout().flush().unwrap();
            let mut descripcion = String::new();
            std::io::stdin().read_line(&mut descripcion).unwrap();
            descripcion = descripcion.trim().to_string();

            let subtask = SubTask::new(titulo, descripcion);

            vec_subtask.push(subtask);
        } else if value_option == "!" {
            //si es ! el valor no añadir mas subtareas

            if !vec_subtask.is_empty() {
                return Some(vec_subtask);
            } else {
                return None;
            }
        }

        count += 1;
    }
}

fn read_date() -> String {
    loop {
        println!("Fecha limite de la tarea(DD-MM-YYYY): ");
        stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.matches('-').count() == 2 && input.len() == 10 {
            let parts: Vec<&str> = input.split('-').collect();
            if parts.len() == 3
                && parts[0].len() == 2
                && parts[0].parse::<u32>().is_ok()
                && parts[1].len() == 2
                && parts[1].parse::<u32>().is_ok()
                && parts[2].len() == 4
                && parts[2].parse::<u32>().is_ok()
            {
                return input.to_string();
            }
        }

        println!("Formato incorrecto. Usa DD-MM-YYYY (ej: 15-01-2024)")
    }
}

fn read_category(client: &Client) -> ObjectId {
    println!("Categoria de la tarea: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    //tenemos que buscar la categoria en la base de datos
    let my_coll: Collection<Document> = client.database("belador_db").collection("category");
    let find_result: Option<Document> = my_coll.find_one(doc! { "title": &input }).run().unwrap();

    if let Some(doc) = find_result {
        if let Ok(oid) = doc.get_object_id("_id") {
            return oid.clone();
        }
    }

    // Si no se encuentra la categoría, terminar el programa
    eprintln!("❌ Categoría '{}' no encontrada.", input);
    eprintln!("💡 Por favor, crea la categoría primero usando el comando:");
    eprintln!("   -create --category");
    process::exit(1);
}
