use std::io;

fn main() {
    let mut count = 0;
    let mut iterations = String::new();
    let mut n1 = 0;
    let mut n2 = 1;
    let mut next_number = 0;

    println!("Please enter an index:");

    io::stdin()
        .read_line(&mut iterations)
        .expect("Failed to read line");

    let iterations: usize = iterations
        .trim()
        .parse()
        .expect("Index entered was not a number");
        
    loop {
        count += 1;

        n1 = n2;
        n2 = next_number;
        println!("{count}th number: {next_number}");
        next_number = n1 + n2;


        if count == iterations {
            break;
        }
    };
}