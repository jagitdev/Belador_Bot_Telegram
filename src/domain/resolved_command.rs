use crate::application::create::Create;

#[derive(Debug)]
pub enum ResolvedCommand {
    Create(Create),
    Unknown,
}

impl ResolvedCommand {
    pub fn run(self) {
        match self {
            ResolvedCommand::Create(c) => c.resolve_args(),
            ResolvedCommand::Unknown => {
                eprintln!("Comando desconocido");
            }
        }
    }
}
