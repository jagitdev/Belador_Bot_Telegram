use crate::domain::subtask::SubTask;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Category {
    title: String,
}

impl Category {
    pub fn new(title: String) -> Self {
        Category { title }
    }
}

impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Category", 1)?;
        s.serialize_field("title", &self.title)?;
        s.end()
    }
}
