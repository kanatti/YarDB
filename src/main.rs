use std::{io::{self, Write}, process::exit};

const LOGO: &'static str = r#"
 __  __     ______     ______     _____     ______   
/\ \_\ \   /\  __ \   /\  == \   /\  __-.  /\  == \  
\ \____ \  \ \  __ \  \ \  __<   \ \ \/\ \ \ \  __<  
 \/\_____\  \ \_\ \_\  \ \_\ \_\  \ \____-  \ \_____\
  \/_____/   \/_/\/_/   \/_/ /_/   \/____/   \/_____/            
"#;

fn main() {
    print_and_flush(LOGO);
    print_and_flush("YarDB Version 0.0.1\n\n");

    repl();
}

fn repl() {
    loop {
        print_and_flush("yardb> ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading from standard input");

        if input.trim() == ".exit" {
            exit(0);
        }

        print_and_flush(&input);
    }
}


fn print_and_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}