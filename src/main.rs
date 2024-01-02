use std::{
    io::{self, Write},
    process::exit,
};

use ast::Statement;
use table::Table;

mod ast;
mod constants;
mod page;
mod pager;
mod parser;
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
    let mut table = Table::new("test".to_owned());

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
            match handle_meta_command(command, &mut table) {
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

fn handle_meta_command(command: &str, table: &mut Table) -> Result<(), MetaCommandHandleError> {
    match command {
        ".exit" => {
            table.close();
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
    parser::parse(command)
}

/// Executes a given statement
fn execute_statement(statement: &Statement, table: &mut Table) {
    match statement {
        Statement::Insert(insert) => {
            let row = get_row(insert.id, insert.name, insert.email);
            table.insert_row(&row)
        }
        Statement::Select(_select) => table.select_rows(),
        Statement::Stats(_stats) => table.stats(),
    }
}

fn print_and_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

fn get_row(id: i32, username: &str, email: &str) -> row::Row {
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
