use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

use crate::domain::task::Task;
use crate::infrastructure::folder_fs::FileManager;
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
        Self::user_question();

        let path = Path::new("src\\data\\data.json");
        let file_manager = FileManager::new(path.to_string_lossy().to_string());

        let content = "hola";

        //crear Task
        println!("Creando Tarea");

        println!("Titulo de la Tarea: ");
        stdout().flush().unwrap(); // necesario para que el print aparezca antes de leer
        let mut titulo = String::new();
        std::io::stdin().read_line(&mut titulo).unwrap();

        println!("Descripción de la tarea: ");
        stdout().flush().unwrap();
        let mut descripcion = String::new();
        std::io::stdin().read_line(&mut descripcion).unwrap();

        println!("Hora de creación de la tarea: ");
        stdout().flush().unwrap();
        let mut hora_creacion = String::new();
        std::io::stdin().read_line(&mut hora_creacion).unwrap();

        println!("Fecha de creación de la tarea: ");
        stdout().flush().unwrap();
        let mut fecha_creacion = String::new();
        std::io::stdin().read_line(&mut fecha_creacion).unwrap();

        println!("Categoria de la tarea: ");
        stdout().flush().unwrap();
        let mut categoria = String::new();
        std::io::stdin().read_line(&mut categoria).unwrap();

        println!("Fecha limite de la tarea: ");
        stdout().flush().unwrap();
        let mut fecha_limite = String::new();
        std::io::stdin().read_line(&mut fecha_limite).unwrap();

        println!("Progreso de la tarea: ");
        stdout().flush().unwrap();
        let mut progreso = String::new();
        std::io::stdin().read_line(&mut progreso).unwrap();

        println!("Descripción de la tarea: ");
        stdout().flush().unwrap();
        let mut descripcion = String::new();
        std::io::stdin().read_line(&mut descripcion).unwrap();

        match file_manager.write_to_file(content) {
            Ok(_) => println!("File written successfully!"),
            Err(e) => eprintln!("Error writing to file: {:?}", e),
        }
    }

    fn user_question() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
}
