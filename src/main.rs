use std::collections::HashMap;
use std::io::Stdin;
use std::io::{self, Write};
use std::str;

fn main() {
    let stdin: Stdin = io::stdin();
    let mut buffer: String = String::new();

    loop {
        let choice: u8 = get_choice(&mut buffer, &stdin);
        match choice {
            1 => reverse_string(&stdin),
            2 => two_sum(),
            _ => {
                println!("Please make a valid choice\n");
                continue;
            }
        }
        break;
    }
}

fn get_choice(buffer: &mut String, stdin: &Stdin) -> u8 {
    loop {
        buffer.clear();
        read_choice(buffer, stdin);
        let choice: u8 = match buffer.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to parse int. Try again!\n");
                continue;
            }
        };
        return choice;
    }
}

fn read_choice(buffer: &mut String, stdin: &Stdin) {
    println!("Enter your choice of algorithm:");
    println!("1: Reverse String");
    println!("2: Two Sum");
    print!("Choice > ");
    io::stdout().flush().expect("Could not flush stdout!");
    match stdin.read_line(buffer) {
        Ok(n) => {
            println!("\nRead {n} bytes");
            println!("You chose {}\n", buffer.trim());
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

fn reverse_string(stdin: &Stdin) {
    println!("Welcome to reverse string!");
    print!("Enter a string of text > ");
    io::stdout().flush().expect("Could not flush stdout!");

    let mut buffer: String = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer = buffer.trim_end().to_owned();

    // Convert to byte array and create pointers
    let bytes: &mut [u8] = unsafe { buffer.as_bytes_mut() };
    let mut first: usize = 0;
    let mut last: usize = 0;

    while last < bytes.len() {
        // Find the first word
        while last < bytes.len() && bytes[last] != b' ' {
            last += 1;
        }

        let next: usize = last + 1;
        last -= 1;

        // Reverse the word
        while first < last {
            let temp: u8 = bytes[first];
            bytes[first] = bytes[last];
            bytes[last] = temp;
            first += 1;
            last -= 1;
        }

        // Reset at beginning of next word
        last = next;
        first = next;
    }

    let text: &str = unsafe { str::from_utf8_unchecked(bytes) };
    println!("\n{text}");
}

fn two_sum() {
    let stdin = io::stdin();

    println!(
        "Enter as many numbers as you would like. Enter a blank line to go to the next stage..."
    );

    let mut num_list: Vec<i32> = Vec::new();

    loop {
        let mut input: String = String::new();

        stdin.read_line(&mut input).expect("Failed to read line");

        if input == "\n" {
            break;
        }

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to enter a valid number or a blank line");
                continue;
            }
        };
        num_list.push(number);
    }

    println!("Enter a single number that is the sum of two of the previous numbers");

    let number: i32;

    loop {
        let mut input: String = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to enter a valid number");
                continue;
            }
        };
        break;
    }

    let mut tried: HashMap<i32, i32> = HashMap::new();

    for i in 0..num_list.len() {
        let current: i32 = num_list[i];
        let other = number - current;

        if tried.contains_key(&other) {
            println!(
                "{} and {} make {}",
                other.to_string(),
                current.to_string(),
                number.to_string()
            );
            return;
        } else {
            tried.insert(current, other);
        }
    }
}
