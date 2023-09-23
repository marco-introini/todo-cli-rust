// command interface
pub trait Command {
    fn handle(&self) -> i32;
}


pub struct AddCommand {
    
}

impl AddCommand {
    pub fn new() -> Self {
        return AddCommand {

        }
    }
}

impl Command for AddCommand{
    fn handle (&self) -> i32 {
        println!("Add todo");
        0
    }
}

pub struct ListCommand {

}

impl ListCommand {
    pub fn new() -> Self {
        return ListCommand {

        }
    }
}

impl Command for ListCommand {
    fn handle (&self) -> i32 {
        println!("List todo");
        0
    }
}
