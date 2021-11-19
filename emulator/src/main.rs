use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, Read};
use std::path::Path;

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
                    0 => String::from("do nothing"),
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
        (0b100, jump) => format!("Jump: with offset {}", -1 * jump as i32),
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
        .expect("Please supply the path to the executable (ending in .bb) as the first argument");
    let path = Path::new(&input_path);
    if !path.is_file() {
        panic!("Passed path is not a file");
    }
    if !path.exists() {
        panic!("Passed path does not exist");
    }
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
    run_binary(bytes, 0);
    println!("{:-^32}", " Program exited ");
}

/// Run a binary comprised of `bytes`
pub fn run_binary(bytes: Vec<u8>, input: i32) -> i32 {
    let mut output = input;
    let mut registers: HashMap<u8, i32> = HashMap::from([(0, 0)]);
    let mut return_address = 0;

    let mut functions: Vec<Vec<u8>> = Vec::new();

    let mut execute_byte = |i: usize, byte: u8, functions: &Vec<Vec<u8>>| -> usize {
        let operation = byte >> 5;
        let immediate = byte & 0b11111;
        match (operation, immediate) {
            (0b001, syscall) => match syscall {
                // do nothing
                0 => {}
                // print
                1 => {
                    println!("{}", output);
                }
                // input
                5 => {
                    let mut line = String::new();
                    stdin().lock().read_line(&mut line).unwrap();
                    output = line
                        .trim()
                        .parse::<i32>()
                        .expect("Only integers are allowed as input");
                }
                // exit
                10 => {
                    println!("{:-^32}", " Exit from syscall ");
                    std::process::exit(0);
                }
                // run function
                16 => {
                    return_address = i;
                    if let Some(func) = functions.get(syscall as usize) {
                        output = run_binary(func.clone(), output);
                    }
                }
                _ => panic!("Invalid system call: {}", syscall),
            },
            (0b010, save) => {
                registers.insert(save, output);
            }
            (0b011, load) => {
                if let Some(&val) = registers.get(&load) {
                    output = val;
                } else {
                    output = 0;
                }
            }
            (0b100, jump) => {
                return i.saturating_sub(jump as usize);
            }
            (0b101, add) => {
                if let Some(&val) = registers.get(&add) {
                    output += val;
                }
            }
            (0b110, addi) => {
                output += addi as i32;
            }
            (0b111, skipeq) => {
                if let Some(&register) = registers.get(&skipeq) {
                    if output == register {
                        return i + 1;
                    }
                }
            }
            _ => {}
        }
        i
    };

    let mut function_started = false;
    let mut current_function = Vec::new();
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        match byte {
            0b001_10100 => {
                function_started = true;
                continue;
            }
            0b001_10101 => {
                function_started = false;
                functions.push(current_function.clone());
                for (i, func) in functions.iter().enumerate() {
                    println!("-- Function {}", i);
                    for &instr in func {
                        print!("{}", describe_byte(instr));
                        println!();
                    }
                    println!();
                }
                continue;
            }
            _ => {}
        }
        if function_started {
            current_function.push(byte);
            continue;
        }

        index = execute_byte(index, byte, &functions);
        index += 1;
    }
    output
}
