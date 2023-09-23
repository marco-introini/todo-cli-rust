use std::{env, process};

fn main() {
    let parametri: Vec<String> = env::args().collect();

    let comando = parametri.get(1).unwrap_or_else(|| {
        help_message();
        process::exit(0);
    });
    dbg!(comando);

}


fn help_message() -> () {
    println!("Uso: <comando> <opzione>");
    println!();
    println!("Comando: add <stringa> o list");
}