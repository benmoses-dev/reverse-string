use std::io::Stdin;
use std::io::{self, Write};
use std::iter::Peekable;
use std::str::{self, Chars};
use std::time::Instant;

fn main() {
    let stdin: Stdin = io::stdin();
    let mut buffer: String = String::new();

    loop {
        let choice: u8 = get_choice(&mut buffer, &stdin);
        match choice {
            1 => {
                println!("You chose the safe algorithm\n");
                reverse_string(&stdin);
            }
            2 => {
                println!("You chose the unsafe algorithm. Ensure the text is raw ASCII\n");
                reverse_string_unsafe(&stdin);
            }
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
                println!("Failed to parse as int. Try again!\n");
                continue;
            }
        };
        return choice;
    }
}

fn read_choice(buffer: &mut String, stdin: &Stdin) -> () {
    println!("Enter your choice of algorithm:");
    println!("1: Safe");
    println!("2: Unsafe");
    print!("Choice > ");
    io::stdout().flush().expect("Could not flush stdout!");
    stdin.read_line(buffer).expect("Error reading line!");
}

fn reverse_string_unsafe(stdin: &Stdin) {
    print!("Enter a string of text > ");
    io::stdout().flush().expect("Could not flush stdout!");

    let mut buffer: String = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer = buffer.trim_end().to_owned();
    let now = Instant::now();

    // Convert to byte array and create pointers
    let bytes: &mut [u8] = unsafe { buffer.as_bytes_mut() };
    let mut head: usize = 0;
    let mut tail: usize = 0;
    let punctuation: [u8; 7] = [b' ', b'.', b',', b'!', b'?', b';', b':'];

    while head < bytes.len() {
        // Find the next word
        while head < bytes.len() && !punctuation.contains(&bytes[head]) {
            head += 1;
        }

        // End of word
        let mut end: usize = head - 1;

        while head < bytes.len() && punctuation.contains(&bytes[head]) {
            head += 1;
        }

        // Reverse the word
        while tail < end {
            let temp: u8 = bytes[tail];
            bytes[tail] = bytes[end];
            bytes[end] = temp;
            tail += 1;
            end -= 1;
        }

        // Set tail to the beginning of next word
        tail = head;
    }

    let text: &str = unsafe { str::from_utf8_unchecked(bytes) };
    let elapsed = now.elapsed();
    println!("\nYour text is > {text}");
    println!("The unsafe algorithm took {} seconds", elapsed.as_secs_f64());
}

fn reverse_string(stdin: &Stdin) {
    print!("Enter a string of text > ");
    io::stdout().flush().expect("Could not flush stdout!");

    let mut buffer: String = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer = buffer.trim_end().to_owned();
    let now = Instant::now();

    let mut buf_iter: Peekable<Chars<'_>> = buffer.chars().peekable();
    let punctuation: [char; 7] = [' ', '.', ',', '!', '?', ';', ':'];
    let mut peeked: &char;
    let mut next: char;
    let mut text: String = String::new();

    loop {
        peeked = match buf_iter.peek() {
            Some(val) => val,
            None => {
                break;
            }
        };

        if punctuation.contains(peeked) {
            // Add the punctuation mark to the result
            next = buf_iter.next().unwrap();
            text.push(next);
            continue;
        }

        // We are at the start of the next word
        let mut temp: String = String::new();
        loop {
            peeked = match buf_iter.peek() {
                Some(val) => val,
                None => {
                    break;
                }
            };
            if punctuation.contains(peeked) {
                break;
            }
            next = buf_iter.next().unwrap();
            temp.push(next);
        }
        temp = temp.chars().rev().collect();
        text.push_str(&temp);
    }
    let elapsed = now.elapsed();
    println!("\nYour text is > {text}");
    println!("The safe algorithm took {} seconds", elapsed.as_secs_f64());
}
