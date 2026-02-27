use std::io::{self, Write};

fn main() {
    print!("Enter number: "); 
    io::stdout().flush().unwrap(); 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Problem with reading");

    io::stdin()
        .read_line(&mut input)
        .expect("Problem with reading");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter natural number!");
            return;
        }
    };

    let result = sum_of_digits(number);

    println!("Sum is: {}", result);
}

fn sum_of_digits(n: i32) -> i32 {
    if n < 10 {
        return n;
    }

    (n % 10) + sum_of_digits(n / 10)
}
