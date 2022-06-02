use std::io;

fn main(){
    print!("Tell me what's your name: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    println!("Rust says Hello to {}", buffer);
}