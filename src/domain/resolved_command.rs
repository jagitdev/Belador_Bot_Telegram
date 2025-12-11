use crate::application::create::Create;
use crate::application::delete::Delete;

#[derive(Debug)]
pub enum ResolvedCommand {
    Create(Create),
    Delete(Delete),
    Unknown,
}

impl ResolvedCommand {
    pub fn run(self) {
        match self {
            ResolvedCommand::Create(c) => c.resolve_args(),
            ResolvedCommand::Delete(d) => d.resolve_args(),
            ResolvedCommand::Unknown => {
                eprintln!("Comando desconocido");
            }
        }
    }
}
