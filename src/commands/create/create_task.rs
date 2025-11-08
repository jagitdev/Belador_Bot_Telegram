pub enum Args {
    Task(Vec<String>),
    Category(Vec<String>),
    Unknown,
}

impl Args {
    pub fn new(args: Vec<String>) -> Self {
        match args.first().map(|s| s.as_str()) {
            Some("--task") => Args::Task(args),
            Some("--category") => Args::Category(args),
            _ => Args::Unknown,
        }
    }
}
