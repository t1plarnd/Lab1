use std::io;

fn main() {
    println!("Enter number:");

    let mut input = String::new();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit() {
        assert_eq!(sum_of_digits(5), 5);
    }

    #[test]
    fn test_multiple_digits() {
        assert_eq!(sum_of_digits(228), 12); 
    }

    #[test]
    fn test_large_number() {
        assert_eq!(sum_of_digits(12345), 15); 
    }

    #[test]
    fn test_zero() {
        assert_eq!(sum_of_digits(0), 0);
    }

    #[test]
    fn test_negative_number() {
        let result = sum_of_digits(-15);
        assert!(result <= 0, "");
    }
}