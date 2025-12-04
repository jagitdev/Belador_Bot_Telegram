use crate::domain::subtask::SubTask;
use mongodb::bson::oid::ObjectId;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Task {
    title: String,                   //titulo de la tarea
    description: String,             //descripción de la tarea
    hours_create: String,            //horas de creación
    date_create: String,             //fecha de creación
    category: ObjectId,              //categoría de la tarea
    date_limit: String,              //fecha límite de la tarea
    progress: String,                //progreso de la tarea
    sub_tasks: Option<Vec<SubTask>>, //subtareas de la tarea
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        hours_create: String,
        date_create: String,
        category: ObjectId,
        date_limit: String,
        progress: String,
        sub_tasks: Option<Vec<SubTask>>,
    ) -> Self {
        Task {
            title,
            description,
            hours_create,
            date_create,
            category,
            date_limit,
            progress,
            sub_tasks,
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
