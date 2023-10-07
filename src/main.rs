use std::env;

fn main() {
    let mut args = env::args();

    args.next();

    let query = args
        .next()
        .expect("Invalid number of arguments. Please provide a query to search for!");

    let file_path = args
        .next()
        .expect("Invalid number of arguments. Please provide the file path!");

    println!("Searching for {query}");
    println!("On file at {file_path}");
}
