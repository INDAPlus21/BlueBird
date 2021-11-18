use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{env, future};

pub fn describe_byte(byte: u8) -> String {
    let operation = byte >> 5;
    let immediate = byte & 0b11111;

    let comment = match (operation, immediate) {
        (0b001, 20) => format!("-- Start function"),
        (0b001, 21) => format!("-- End function"),
        (0b001, syscall) => {
            format!(
                "Syscall: {}",
                match syscall {
                    0 => String::from("reserved for compiler"),
                    1 => String::from("print to stdout"),
                    5 => String::from("read from stdin"),
                    10 => String::from("exit program"),
                    16 => String::from("run function"),
                    20 => String::from("start of function"),
                    21 => String::from("end of function"),
                    c => c.to_string(),
                }
            )
        }
        (0b010, save) => format!("Save: register {}", save),
        (0b011, load) => format!("Load: register {}", load),
        (0b100, jump) => format!("Jump: to address {}", jump),
        (0b101, add) => format!("Add: register {}", add),
        (0b110, addi) => format!("Addi: immediate {}", addi),
        (0b111, skipeq) => format!("Skipeq: register {}", skipeq),
        _ => format!(""),
    };

    format!(
        "{:03b} {:05b} {:<8}\t# {}",
        operation, immediate, immediate, comment
    )
}

fn main() {
    let args = env::args();
    let input_path = args
        .skip(1)
        .next()
        .expect("Please supply the path as the first argument");
    let path = Path::new(&input_path);
    let mut file = File::open(path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    println!(
        "{:<4} {} {} {:<8}\t# comment",
        "addr", "ope", "immed", "decimal"
    );
    println!("{:-^1$}", "", 32);
    for (i, &byte) in bytes.iter().enumerate() {
        println!("{:<4} {}", i, describe_byte(byte));
    }
    println!("");

    println!("{:-^32}", " Running program ");
    run_binary(bytes);
    println!("");
    println!("{:-^32}", " Program exited ");
}

/// Run a binary comprised of `bytes`
pub fn run_binary(bytes: Vec<u8>) {
    let mut functions: Vec<Vec<u8>> = Vec::new();

    let mut function_started = false;
    let mut current_function = Vec::new();
    for byte in bytes {
        match byte {
            0b001_10100 => {
                function_started = true;
                continue;
            }
            0b001_10101 => {
                function_started = false;
                functions.push(current_function.clone());
                println!("{:#?}", &functions);
                continue;
            }
            _ => {}
        }
        if function_started {
            current_function.push(byte);
        }

        let operation = byte >> 5;
        let immediate = byte & 0b11111;
        match (operation, immediate) {
            _ => {}
        }
    }
}
