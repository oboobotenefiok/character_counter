use std::io;

fn main() {
    println!(
        "Welcome To Obot's First RUST Character Counter
Ignoring Whitespaces! \n Make your first input. "
    );

    let mut first_group = String::new();
    io::stdin()
        .read_line(&mut first_group)
        .expect("Failed to read input");
    first_group = first_group.trim().to_string();
    
    // I'm doing this to avoid crashing when less than 11
    let preview = if first_group.len() > 10 {
        format!("{}...", &first_group[..10])
    } else {
        first_group.clone()
    };

    println!("{} received, Thank You! Next Input?", preview);

    let mut second_group = String::new();
    io::stdin()
        .read_line(&mut second_group)
        .expect("Failed to read input");
    second_group = second_group.trim().to_string();

    println!(
        "Good one bro! \n The result is: {}
\n Thanks for using my Rusty counter, bro!",
        add(first_group, second_group)
    );
}

fn add(x: String, y: String) -> usize {
    x.len() + y.len()
}