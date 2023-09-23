mod commands;

use std::{env, process};

use commands::*;

fn main() {
    let parameters: Vec<String> = env::args().collect();

    let command = parameters.get(1).unwrap_or_else(|| {
        help_message();
        process::exit(0);
    });

    let exit_code = match command.as_str() {
        "add" => AddCommand::new().handle(),
        "list" => ListCommand::new().handle(),
        _ => {
            println!("Unknown command");
            1
        } // default
    };

    process::exit(exit_code);

}


fn help_message() -> () {
    println!("Uso: <comando> <opzione>");
    println!();
    println!("Comando: add <stringa> o list");
}