use crate::application::create::Create;

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
            command: command.into(),
            args: args.into(), // <-- usar los args pasados
        }
    }

    pub fn resolve(self) -> ResolvedCommand {
        let resolved_command = match self.command.as_str() {
            "-create" => ResolvedCommand::Create(Create::new(self.args)),
            _ => ResolvedCommand::Unknown,
        };

        resolved_command
    }

    pub fn resolve_args


}
