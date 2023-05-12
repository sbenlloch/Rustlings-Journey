use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

const LCG_MULTIPLIER: u64 = 6364136223846793005;
const LCG_INCREMENT: u64 = 1442695040888963407;

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new() -> Lcg {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        Lcg { state: seed }
    }

    fn next(&mut self) -> u64 {
        // Generate a new seed from the current system time and the previous state of the LCG.
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
        self.state ^= seed;

        // Generate the next pseudorandom number.
        self.state = self.state.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_INCREMENT);
        self.state
    }
}

fn get_random_number(min: u64, max: u64) -> u64 {
    let mut rng = Lcg::new();
    rng.next() % (max - min + 1) + min
}

fn get_random_string(length: usize) -> String {
    let mut rng = Lcg::new();
    let mut string = String::new();
    for _ in 0..length {
        string.push(((rng.next() % 26) as u8 + b'a') as char);
    }
    string
}

fn main() {
    let mut args = env::args();
    args.next(); // Skip the executable name.

    let command = args.next().unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "help" => {
            println!("Usage: {} <command> [args]", args.next().unwrap_or_else(|| "random".to_string()));
            println!("Commands:");
            println!("  help - Print this help message.");
            println!("  random - Generate a random number.");
            println!("  string - Generate a random string.");
        }
        "random" => {
            let min = args.next().unwrap_or_else(|| "0".to_string()).parse().unwrap();
            let max = args.next().unwrap_or_else(|| "100".to_string()).parse().unwrap();
            let number = get_random_number(min, max);
            println!("{}", number);
        }
        "string" => {
            let length = args.next().unwrap_or_else(|| "10".to_string()).parse().unwrap();
            let string = get_random_string(length);
            println!("{}", string);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
