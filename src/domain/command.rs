use crate::application::create::Create;
use crate::application::delete::Delete;
use crate::domain::resolved_command::ResolvedCommand;

#[derive(Debug)]
pub struct Command {
    command: String,
    args: Vec<String>,
}

impl Command {
    pub fn new<C, A>(command: C, args: A) -> Self
    where
        C: Into<String>, //Into para que en firmas de funciones acepten "Cualquier cosa" que se convierta al tipo objetivo
        A: Into<Vec<String>>,
    {
        Command {
            command: command.into(), //into() para que se convierta al tipo objetivo
            args: args.into(),       // <-- usar los args pasados
        }
    }

    pub fn resolve(self) -> ResolvedCommand {
        let resolved_command = match self.command.as_str() {
            "-create" => ResolvedCommand::Create(Create::new(self.args)),
            "-delete" => ResolvedCommand::Delete(Delete::new(self.args)),
            _ => ResolvedCommand::Unknown,
        };

        resolved_command
    }
}
