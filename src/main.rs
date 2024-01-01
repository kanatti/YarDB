use std::{
    io::{self, Write},
    process::exit,
};

const LOGO: &str = r#"
 __  __     ______     ______     _____     ______   
/\ \_\ \   /\  __ \   /\  == \   /\  __-.  /\  == \  
\ \____ \  \ \  __ \  \ \  __<   \ \ \/\ \ \ \  __<  
 \/\_____\  \ \_\ \_\  \ \_\ \_\  \ \____-  \ \_____\
  \/_____/   \/_/\/_/   \/_/ /_/   \/____/   \/_____/            
"#;

const META_PREFIX: &str = ".";

fn main() {
    print_and_flush(LOGO);
    print_and_flush("\nYarDB Version 0.0.1\n\n");

    repl();
}

fn repl() {
    loop {
        print_and_flush("yardb> ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading from standard input");

        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        if command.starts_with(META_PREFIX) {
            match handle_meta_command(command) {
                Ok(_) => {}
                Err(e) => {
                    print_and_flush(&format!("Error: {:?}\n", e));
                }
            }
        } else {
            handle_sql_command(command);
        }
    }
}

fn handle_meta_command(command: &str) -> Result<(), MetaCommandHandleError> {
    match command {
        ".exit" => {
            exit(0);
        }
        ".help" => {
            print_and_flush("Available commands:\n");
            print_and_flush(".exit\n");
            print_and_flush(".help\n");
            print_and_flush("select\n");
            print_and_flush("insert\n");
            Ok(())
        }
        _ => Err(MetaCommandHandleError::UnRecognizedCommand),
    }
}

#[derive(Debug)]
enum MetaCommandHandleError {
    UnRecognizedCommand,
}

fn handle_sql_command(command: &str) {
    match prepare_statement(command) {
        Ok(statement) => execute_statement(&statement),
        Err(e) => print_and_flush(&format!("Error: {}", e)),
    }
}

fn prepare_statement(command: &str) -> Result<Statement, String> {
    match command {
        "insert" => Ok(Statement::Insert),
        "select" => Ok(Statement::Select),
        _ => Err(String::from(&format!(
            "Unrecognized keyword at start of {}\n",
            command
        ))),
    }
}

fn execute_statement(statement: &Statement) {
    match statement {
        Statement::Insert => print_and_flush("Executing insert statement\n"),
        Statement::Select => print_and_flush("Executing select statement\n"),
    }
}

fn print_and_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

enum Statement {
    Insert,
    Select,
}
