use std::io::{self, Write};

fn main() {
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();
        print_prompt();
        read_input(&mut input_buffer);

        if input_buffer.starts_with(".exit") {
            break;
        } else {
            println!("Unrecognized command '{input_buffer}'.");
            std::io::stdout().flush().expect("Unable to flush stdout.");
        }
    }
}


fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().expect("Unable to flush stdout.");
}

fn read_input(buffer: &mut String) {
    io::stdin().read_line(buffer).expect("Error reading stdin.");
    if buffer.bytes().last() == Some(b'\n') {
        buffer.truncate(buffer.len()-1);
    }
}
