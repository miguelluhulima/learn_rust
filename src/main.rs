use std::io;

fn main() {
    let mut count = 0;
    let mut len = String::new();
    let mut n1 = 0;
    let mut n2 = 1;
    let mut next_number = 0;
    let mut a: [i32; 100] = [0; 100];

    println!("Please enter length:");

    io::stdin()
        .read_line(&mut len)
        .expect("Failed to read line");

    let len: usize = len
        .trim()
        .parse()
        .expect("Index entered was not a number");

    loop {
        n1 = n2;
        n2 = next_number;
        a[count] = next_number;
        next_number = n1 + n2;

        count += 1;
        if count == len {
            break;
        }
    }

    println!("{:?}", &a[0 .. len]);
}
