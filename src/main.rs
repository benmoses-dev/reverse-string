use rand::Rng;
use std::collections::HashMap;
use std::io;
use std::str;

fn main() {
    let random_number: u32 = rand::thread_rng().gen_range(1..3);

    println!("Welcome to reverse string!");
    println!("The random number is {}", random_number);

    match random_number {
        1 => reverse_string(),
        2 => two_sum(),
        3 => println!("Well done!"),
        _ => println!("Wildcard"),
    }
}

fn reverse_string() {
    let stdin = io::stdin();

    println!("Enter a string of text:");

    let mut text: String = String::new();
    stdin.read_line(&mut text).expect("Failed to read line");
    text = text.trim().to_owned();

    println!("Carrying out a very quick, potentially unsafe, in-place reversal of your text:");

    let text: &mut [u8] = unsafe { text.as_bytes_mut() };

    let mut first: usize = 0;
    let mut last: usize = 0;

    while last < text.len() && text[last].is_ascii() {
        while last < text.len() && text[last].is_ascii() && text[last] != b' ' {
            last = last + 1;
        }

        let next: usize = last + 1;
        last = last - 1;

        while first < last {
            let temp: u8 = text[first];
            text[first] = text[last];
            text[last] = temp;
            first = first + 1;
            last = last - 1;
        }

        last = next;
        first = next;
    }

    let text: &str = str::from_utf8(text).unwrap_or_default();
    println!("{}", text);
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
            println!("{} and {} make {}", other.to_string(), current.to_string(), number.to_string());
            return;
        } else {
            tried.insert(current, other);
        }
    }
}
