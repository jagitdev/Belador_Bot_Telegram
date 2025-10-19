use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

use crate::domain::task::Task;
use crate::infrastructure::folder_fs::FileManager;
use chrono::Local;
use std::io::{stdout, Read, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Create {
    args: Vec<String>,
}

impl Create {
    pub fn new(args: Vec<String>) -> Self {
        Create { args: args }
    }

    pub fn run_create(self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap(); //borrar terminal
        let path = Path::new("src\\data\\data.json");
        let file_manager = FileManager::new(path.to_string_lossy().to_string());

        //crear Task
        println!("Creando Tarea");

        //titulo
        let title = Self::read_data("Titulo de la Tarea: ");

        //descripcion
        let description = Self::read_data("Descripción de la tarea: ");

        //hora
        let hours_create = Local::now().format("%H:%M").to_string();

        //fecha
        let date_create = Local::now().format("%d-%m-%Y").to_string();

        //categoria
        let category = Self::read_data("Categoria de la tarea: ");

        //fecha limite
        let date_limit = Self::read_date();

        //progreso
        let progress = Self::read_progress();

        //subtareas
        println!("Subtareas de la tarea(subt1,subt2,subt3): ");
        stdout().flush().unwrap();
        let mut sub_tasks_string = String::new();
        std::io::stdin().read_line(&mut sub_tasks_string).unwrap();
        let sub_tasks: Vec<&str> = sub_tasks_string.split(",").collect();

        let task = Task::new(
            &title,
            &description,
            &hours_create,
            &date_create,
            &category,
            &date_limit,
            &progress,
            Some(sub_tasks),
        );

        match file_manager.write_to_file(task) {
            Ok(_) => println!("File written successfully!"),
            Err(e) => eprintln!("Error writing to file: {:?}", e),
        }
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
}
