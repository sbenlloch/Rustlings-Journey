use std::io::{self, Write};
use std::time::Duration;
use std::thread::sleep;

fn get_user_float(prompt: &str) -> f32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if let Ok(num) = input.parse() {
            return num;
        } else if input == "q" {
            std::process::exit(0);
        }

        println!("Invalid input. Please enter a valid floating-point number.");
    }
}

fn print_binary_representation(number: f32) {
    let bits: u32 = unsafe { std::mem::transmute(number) };
    let binary = format!("{:032b}", bits);

    let (sign, rest) = binary.split_at(1);
    let (exponent, fraction) = rest.split_at(8);

    println!("[{}]\n", number);

    println!("+------+----------------+----------------------------------+");
    println!("| {:^4} | {:^14} | {:^32} |", "Sign", "Exponent", "Fraction/Mantissa");
    println!("+------+----------------+----------------------------------+");
    println!("| {:^4} | {:^14} | {:^32} |", sign, exponent, fraction);
    println!("+------+----------------+----------------------------------+");

    println!();

    sleep(Duration::from_millis(50));
}

fn type_writer_effect(s: &str) {
    for c in s.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(50));    }
    println!();
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        let number1 = get_user_float("Enter the first floating-point number (q to quit): ");
        let number2 = get_user_float("Enter the second floating-point number: ");

        let sum = number1 + number2;
        let product = number1 * number2;

        clear_console();

        type_writer_effect("Binary representation of the first number: ");
        print_binary_representation(number1);

        sleep(Duration::from_secs(1));

        type_writer_effect("Binary representation of the second number: ");
        print_binary_representation(number2);

        sleep(Duration::from_secs(1));
        type_writer_effect("Binary representation of the sum: ");
        print_binary_representation(sum);

        sleep(Duration::from_secs(1));

        type_writer_effect("Binary representation of the product: ");
        print_binary_representation(product);

        sleep(Duration::from_secs(2));    }
}

