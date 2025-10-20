#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubTask {
    title: String,       //titulo de la tarea
    description: String, //descripción de la tarea
}

impl SubTask {
    pub fn new(title: &str, description: &str) -> Self {
        SubTask {
            title: title.to_string(),
            description: description.to_string(),
        }
    }
}
