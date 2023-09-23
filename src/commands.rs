// command interface
pub trait Command {
    fn handle(&self) -> i32;
}


pub struct AddCommand {
    args: Vec<String>
}

impl AddCommand {
    pub fn new(args: Vec<String>) -> Self {
        return AddCommand {
            args
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
