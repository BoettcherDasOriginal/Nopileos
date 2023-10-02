use crate::position::Position;


#[derive(Clone, Debug)]
pub enum EntityCommand {
    Null,
    FlyToPos(Position),
}

#[derive(Clone, Debug)]
pub struct EntityCommandHandler {
    pub current_command: EntityCommand,
    pub commands: Vec<EntityCommand>,
}

impl EntityCommandHandler {
    pub fn new(commands: Vec<EntityCommand>) -> Self{
        Self { current_command: EntityCommand::Null, commands: commands }
    }

    pub fn get_current_command(&mut self) {
        if self.commands.clone().len() != 0 {
            self.current_command = self.commands[0].clone();
        }
        else {
            self.current_command = EntityCommand::Null;
        }
    }

    /// gets the next command in the vec & removes the old one from it
    pub fn get_next_command(&mut self) -> EntityCommand{
        if self.commands.clone().len() > 1 {
            self.commands.remove(0);
            return self.commands[0].clone();
        }
        else if self.commands.clone().len() == 1 {
            self.commands.remove(0);
            return EntityCommand::Null;
        }
        else {
            return EntityCommand::Null;
        }
    }
}