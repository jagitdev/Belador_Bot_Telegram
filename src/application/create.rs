use crate::commands::create::create_category::create_category;
use crate::commands::create::create_task::create_task;

#[derive(Debug, Clone)]
pub struct Create {
    args: Vec<String>,
}

impl Create {
    pub fn new(args: Vec<String>) -> Self {
        Create { args: args }
    }

    pub fn resolve_args(self) {
        match self.args.first().map(|s| s.as_str()) {
            Some("--task") => create_task(),
            Some("--category") => create_category(),
            Some(_) => eprintln!("Argumento desconocido para la creación"),
            None => eprintln!("No se ha proporcionado un argumento para la creación"),
        }
    }
}
