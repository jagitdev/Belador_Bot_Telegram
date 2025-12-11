use crate::commands::delete::delete_category_by_title::delete_category_by_title;
use crate::commands::delete::delete_task_by_title::delete_task_by_title;

#[derive(Debug, Clone)]
pub struct Delete {
    args: Vec<String>,
}

impl Delete {
    pub fn new(args: Vec<String>) -> Self {
        Delete { args: args }
    }

    pub fn resolve_args(self) {
        match self.args.first().map(|s| s.as_str()) {
            Some("--title-task") => delete_task_by_title(),
            Some("--title-category") => delete_category_by_title(),
            Some(_) => eprintln!("Argumento desconocido para la eliminación"),
            None => eprintln!("No se ha proporcionado un argumento para la eliminación"),
        }
    }
}
