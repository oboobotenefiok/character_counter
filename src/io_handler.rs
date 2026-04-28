use std::io;

// I used two 'get input' logic before, now this one shortens it.

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}