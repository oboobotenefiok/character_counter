mod io_handler;

fn main() {
    println!(
        "Welcome To Obot's First RUST Character Counter
Ignoring Whitespaces at edges! \n Make your first input. "
    );

    let first_group = io_handler::get_input();
    
    // I'm doing this to avoid crashing when less than 11

    println!("{} received, Thank You! Next Input?",
    if first_group.len() > 10 {
        &first_group[..10]
        // *1 used format before, this shortens it.
    }
    else {
        &first_group
        //*1
    } );
    //I used a variable named preview before; this shortens it.
    let second_group = io_handler::get_input();

    println!(
        "Good one bro! \n The result is: {}
\n Thanks for using my Rusty counter, bro!",
        first_group.len() + second_group.len() 
    // I called an add() function before but this shortens it.
    );
}
