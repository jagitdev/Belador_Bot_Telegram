use crate::domain::subtask::SubTask;
use chrono::{DateTime, Local};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Task {
    title: String,                   //titulo de la tarea
    description: String,             //descripción de la tarea
    hours_create: String,            //horas de creación
    date_create: String,             //fecha de creación
    category: String,                //categoría de la tarea
    date_limit: String,              //fecha límite de la tarea
    progress: String,                //progreso de la tarea
    sub_tasks: Option<Vec<SubTask>>, //subtareas de la tarea
}

impl Task {
    pub fn new(
        title: &str,
        description: &str,
        hours_create: &str,
        date_create: &str,
        category: &str,
        date_limit: &str,
        progress: &str,
        sub_tasks: Option<Vec<SubTask>>,
    ) -> Self {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            hours_create: hours_create.to_string(),
            date_create: date_create.to_string(),
            category: category.to_string(),
            date_limit: date_limit.to_string(),
            progress: progress.to_string(),
            sub_tasks: sub_tasks,
        }
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Task", 9)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("hours_create", &self.hours_create)?;
        s.serialize_field("date_create", &self.date_create)?;
        s.serialize_field("category", &self.category)?;
        s.serialize_field("date_limit", &self.date_limit)?;
        s.serialize_field("progress", &self.progress)?;
        s.serialize_field("sub_tasks", &self.sub_tasks)?;
        s.end()
    }
}
