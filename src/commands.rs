use std::fs;
use std::io::Write;

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

        let todo_option = &self.args.get(2);
        return if let Some(todo) = todo_option {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("Storage.txt")
                .expect("File not found");

            writeln!(file,"{}",todo).expect("File not writable");
            0
        } else {
            println!("ToDo description not typed");
            1
        }
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
        let todos = fs::read_to_string("Storage.txt").expect("File not found");
        println!("{todos}");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_command() {
        // arrange
        let args = vec![
          "todo".to_string(),
          "add".to_string(),
          "example1".to_string(),
        ];
        let add = AddCommand::new(args);

        // act
        let exit_code = add.handle();

        // assert
        assert_eq!(exit_code,0);
    }
}


