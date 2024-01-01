use std::{
    io::{self, Write},
    process::exit,
};

use table::Table;

mod constants;
mod page;
mod row;
mod table;

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
    let mut table = Table::new();

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
            handle_sql_command(command, &mut table);
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
            print_and_flush("stats\n");
            Ok(())
        }
        _ => Err(MetaCommandHandleError::UnRecognizedCommand),
    }
}

#[derive(Debug)]
enum MetaCommandHandleError {
    UnRecognizedCommand,
}

fn handle_sql_command(command: &str, table: &mut Table) {
    match prepare_statement(command) {
        Ok(statement) => execute_statement(&statement, table),
        Err(e) => print_and_flush(&format!("Error: {}\n", e)),
    }
}

/// Parse raw command into Statement
fn prepare_statement(command: &str) -> Result<Statement, String> {
    let mut parts = command.split_whitespace();
    let command = parts.next().expect("Unreachable");
    let args = parts.collect::<Vec<&str>>();

    match command {
        "insert" => {
            if args.len() != 3 {
                Err(String::from("Invalid number of arguments"))
            } else {
                Ok(Statement::Insert(row(
                    args[0].parse().unwrap(),
                    args[1],
                    args[2],
                )))
            }
        }
        "select" => Ok(Statement::Select),
        "stats" => Ok(Statement::Stats),
        _ => Err(String::from(&format!(
            "Unrecognized keyword at start of {}\n",
            command
        ))),
    }
}

/// Executes a given statement
fn execute_statement(statement: &Statement, table: &mut Table) {
    match statement {
        Statement::Insert(row) => table.insert_row(row),
        Statement::Select => table.select_rows(),
        Statement::Stats => table.stats(),
    }
}

fn print_and_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

enum Statement {
    Insert(row::Row),
    Select,
    Stats,
}

fn row(id: i32, username: &str, email: &str) -> row::Row {
    row::Row {
        id,
        username: str_boxed_array(username),
        email: str_boxed_array(email),
    }
}

fn str_boxed_array<const SIZE: usize>(s: &str) -> Box<[u8; SIZE]> {
    let limit = std::cmp::min(s.len(), SIZE);
    let truncated = &s[..limit];

    let mut boxed_array = Box::new([0; SIZE]);
    boxed_array[..limit].copy_from_slice(truncated.as_bytes());

    boxed_array
}
