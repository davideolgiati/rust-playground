use std::io;

fn main(){
    let mut buffer = String::new();
    let mut a = 0;
    let mut b = 0;

    for i in 1..3 {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if (i % 2) == 0 {
            a = buffer.trim().parse().expect("Not an integer!")
        } else {
            b = buffer.trim().parse().expect("Not an integer!")
        }

        buffer = String::new();
    }

    println!("The sum of {} and {} is {}", a, b, sum(a,b))
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}