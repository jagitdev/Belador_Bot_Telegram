#[derive(Debug, Clone)]
pub struct Create {
    args: Vec<String>,
}

impl Create {
    pub fn new(args: Vec<String>) -> Self {
        Create { args: args }
    }

    pub fn run_create(self) {
        println!("{:#?}", self.args)
    }
}
