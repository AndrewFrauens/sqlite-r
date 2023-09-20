use std::io::{self, Write};

fn main() {
    let mut input_buffer = String::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer);

        if input_buffer.bytes().len() == 0 {
            continue;
        }

        if input_buffer
            .bytes()
            .next()
            .expect("There should be at least 1 character in input_buffer.")
            == b'.'
        {
            match process_meta_command(&input_buffer) {
                Ok(()) => {
                    todo!("Should something happen here?");
                }
                Err(MetaCommandError) => {
                    eprintln!("Unrecognized command '{input_buffer}'");
                }
            }
        } else {
            match prepare_statement(&input_buffer) {
                Ok(statement) => {
                    execute_statement(&statement);
                    println!("Executed");
                }
                Err(_) => {
                    eprintln!("Unrecognized keyword at start of '{input_buffer}'");
                }
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().expect("Unable to flush stdout.");
}

fn read_input(buffer: &mut String) {
    buffer.clear();
    io::stdin().read_line(buffer).expect("Error reading stdin.");
    if buffer.bytes().last() == Some(b'\n') {
        buffer.truncate(buffer.len() - 1);
    }
}

struct MetaCommandError;

fn process_meta_command(command: &str) -> Result<(), MetaCommandError> {
    if command.starts_with(".exit") {
        std::process::exit(0);
    }
    Err(MetaCommandError)
}

enum StatementKind {
    Insert,
    Select,
}

struct Statement {
    kind: StatementKind,
}
struct PrepareError;

fn prepare_statement(command: &str) -> Result<Statement, PrepareError> {
    if command.starts_with("insert") {
        Ok(Statement {
            kind: StatementKind::Insert,
        })
    } else if command.starts_with("select") {
        Ok(Statement {
            kind: StatementKind::Select,
        })
    } else {
        Err(PrepareError)
    }
}

fn execute_statement(statement: &Statement) {
    match statement.kind {
        StatementKind::Insert => {
            println!("Insert command detected");
        }
        StatementKind::Select => {
            println!("Select command detected");
        }
    }
}
