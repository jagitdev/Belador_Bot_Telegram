use serde::Serialize;

pub struct FileManager {
    pub base_path: String,
}

impl FileManager {
    pub fn new(base_path: String) -> Self {
        FileManager { base_path }
    }

    // Returns the base path of the file manager.
    pub fn get_base_path(&self) -> &str {
        &self.base_path
    }

    // Sets a new base path for the file manager.
    pub fn set_base_path(&mut self, new_path: String) {
        self.base_path = new_path;
    }

    pub fn write_to_file<T: Serialize>(&self, content: T) -> Result<(), String> {
        let json_content = serde_json::to_string_pretty(&content).map_err(|e| e.to_string())?;

        let file_path = format!("{}", self.base_path);
        std::fs::write(file_path, json_content).map_err(|e| e.to_string())?;

        Ok(())
    }
}
