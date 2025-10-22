use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct SubTask {
    title: String,       //titulo de la tarea
    description: String, //descripción de la tarea
}

//hacer el serialize

impl SubTask {
    pub fn new(title: String, description: String) -> Self {
        SubTask {
            title: title,
            description: description,
        }
    }
}

impl Serialize for SubTask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Sub_Tasks", 2)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("description", &self.description)?;
        s.end()
    }
}
