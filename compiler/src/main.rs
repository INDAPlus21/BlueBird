use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const INT_OPERATIONS: [(&str, u8); 6] = [
    ("call", 1),
    ("save", 2),
    ("load", 3),
    ("add", 5),
    ("addi", 6),
    ("skipeq", 7),
];

/// A java source file representation with abstracted keywords and static fields
#[derive(Debug, Clone, PartialEq, Eq)]
struct JavaFile {
    pub name: String,
    pub abst: bool,
    pub extends: String,
    pub implements: String,
    pub int_fields: HashMap<String, i32>,
    pub str_fields: HashMap<String, String>,
}

impl JavaFile {
    /// Parse a java source file and get relations
    pub fn from_file(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut signature = String::new();
        let mut toplevel = String::new();

        let mut comment_started = false;
        let mut line_comment_started = false;
        let mut block_level = 0;
        // Go through file source code but skip comments
        for (current, next) in contents.chars().zip(contents.chars().skip(1)) {
            if (current, next) == ('/', '*') {
                comment_started = true;
                continue;
            }
            if (current, next) == ('/', '/') {
                line_comment_started = true;
                continue;
            }
            if comment_started && (current, next) == ('*', '/') {
                comment_started = false;
                continue;
            }
            if line_comment_started && current == '\n' {
                line_comment_started = false;
                continue;
            }
            if comment_started || line_comment_started || current == '/' {
                continue;
            }
            // `current` and `next` are verified to not be comments from here on

            if current == '{' {
                block_level += 1;
                continue;
            }
            if current == '}' {
                block_level -= 1;
                continue;
            }
            if block_level == 0 {
                signature.push(current);
            }
            if block_level == 1 {
                toplevel.push(current);
            }
        }
        signature = signature.trim().to_string();
        signature = signature
            .strip_prefix("public")
            .unwrap_or(&signature)
            .to_string();

        // Get keyword from signature
        let get_keyword = |keyword| -> String {
            signature
                .split_whitespace()
                .zip(signature.split_whitespace().skip(1))
                .filter_map(|(a, b)| if a == keyword { Some(b) } else { None })
                .collect()
        };
        let name = get_keyword("class");
        let extends = get_keyword("extends");
        let implements = get_keyword("implements");
        let mut int_fields = HashMap::new();
        let mut str_fields = HashMap::new();

        // Get lines in the top level code block
        for line in toplevel
            .trim()
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty() && x.ends_with(';'))
            .map(|x| x.strip_suffix(';').unwrap_or(x))
        {
            // Get tokens from each line
            let tokens = line
                .split_whitespace()
                .zip(line.split_whitespace().skip(1))
                .zip(line.split_whitespace().skip(2))
                .zip(line.split_whitespace().skip(3))
                .zip(line.split_whitespace().skip(4));
            for ((((accessor, typ), name), equals), value) in tokens {
                // Syntax: `static int [c] = [e]`
                // If the line is a static field expression
                if (accessor, equals) == ("static", "=") {
                    match typ {
                        "int" => {
                            if let Ok(num) = value.parse::<i32>() {
                                int_fields.insert(String::from(name), num);
                            }
                        }
                        "String" => {
                            let value = value.strip_prefix('"').unwrap_or(value);
                            let value = value.strip_suffix('"').unwrap_or(value);
                            str_fields.insert(String::from(name), String::from(value));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(JavaFile {
            name,
            abst: signature.contains("abstract"),
            extends,
            implements,
            int_fields,
            str_fields,
        })
    }

    /// Get the java class that `self` extends
    fn next(self, objects: &Vec<JavaFile>) -> Option<&JavaFile> {
        objects.iter().find(|x| x.name == self.extends)
    }

    /// Get the chain of classes that extend each other, starting at `self`
    fn chain<'a>(&'a self, objects: &'a Vec<JavaFile>) -> Vec<&'a JavaFile> {
        let mut chain = vec![self];
        let mut after = self;
        while let Some(file) = after.clone().next(objects) {
            chain.push(file);
            after = file;
        }
        chain
    }
}

fn main() {
    let args = env::args();
    let input_path = args
        .skip(1)
        .next()
        .expect("Please supply the path as the first argument");
    let path = Path::new(&input_path);
    if !path.is_dir() {
        panic!("Passed path is not a directory");
    }
    if !path.exists() {
        panic!("Passed path does not exist");
    }
    let java_dir = path.read_dir().unwrap();
    let java_files = java_dir
        .filter_map(Result::ok)
        .filter(|d| {
            if let Some(e) = d.path().extension() {
                e == "java"
            } else {
                false
            }
        })
        .map(|x| x.path());
    let java_objects = java_files
        .map(|p| JavaFile::from_file(&p))
        .filter_map(|e| e.ok())
        .collect::<Vec<JavaFile>>();
    let entry = find_entry("main", &java_objects)
        .expect("None of the source files have an entry point `main`")
        .to_owned();

    let chain = entry.chain(&java_objects);
    println!("{:#?}", chain);

    let (coroutines, instructions) = compile_chain(chain, &java_objects);

    println!("Coroutines:");
    for coroutine in &coroutines {
        println!(
            "{:03b} {:05b} ({})",
            coroutine >> 5,
            coroutine & 0b11111,
            coroutine & 0b11111
        );
    }
    println!();

    println!("ope immed (dec)");
    for byte in &instructions {
        // Print operation and following immediate
        println!(
            "{:03b} {:05b} ({})",
            byte >> 5,
            byte & 0b11111,
            byte & 0b11111
        );
    }

    let mut binary = Vec::new();
    binary.extend(&coroutines);
    binary.extend(&instructions);

    let dirname = path.file_name().expect("Unable to find dirname"); // Directory name
    let output_filename = Path::new(dirname).with_extension("bb");
    let mut file = File::create(&output_filename).unwrap();
    file.write_all(&binary[..]).unwrap();
    println!("File binary compiled to {}", &output_filename.display());
}

fn find_entry<'a>(name: &str, objects: &'a Vec<JavaFile>) -> Option<&'a JavaFile> {
    objects.iter().find(|x| x.name == name)
}

/// Returns `(coroutines, instructions)` that merge to create the final binary
fn compile_chain(chain: Vec<&JavaFile>, objects: &Vec<JavaFile>) -> (Vec<u8>, Vec<u8>) {
    let mut coroutines: Vec<u8> = Vec::new();
    // May not exceed 31
    let mut coroutine_idx = 0;

    let mut instructions = Vec::new();
    'chain: for (i, &current) in chain.iter().enumerate() {
        let JavaFile {
            int_fields,
            str_fields,
            ..
        } = current;

        // If instruction is jump
        if let Some(value) = str_fields.get("jump") {
            if let Some(to_jump) = find_entry(value, objects) {
                // If place to jump to is in same chain
                if let Some(to_jump_pos) = chain.iter().position(|&f| f == to_jump) {
                    let offset = to_jump_pos as isize - i as isize;
                    if matches!(offset, 0..=15) {
                        panic!("Tried to jump more than 15 blocks, illegal");
                    }
                    let offset_bin: u8;
                    match offset.signum() {
                        -1 => {
                            offset_bin = 0b1 << 4 | offset.abs() as u8;
                        }
                        _ => {
                            offset_bin = 0b0 << 4 | offset.abs() as u8;
                        }
                    }
                    let mut instruction: u8 = 0b100;
                    instruction = instruction << 5 | offset_bin;
                    instructions.push(instruction);
                } else {
                    // Else: it is a separate function, add it as a coroutine

                    // Start function (syscall with code 20)
                    coroutines.push(0b001 << 5 | 20);

                    let outer_chain = to_jump.chain(objects);
                    // Discard nested subroutines/chains
                    let (_, mut compiled) = compile_chain(outer_chain, &vec![]);
                    coroutines.append(&mut compiled);

                    // End function (syscall with code 21)
                    coroutines.push(0b001 << 5 | 21);

                    // Save previous return value as register 4
                    let mut save = 0b010;
                    save = save << 5 | 4;
                    instructions.push(save);

                    // Load index for coroutine
                    let mut load = 0b011;
                    load = load << 5 | coroutine_idx;
                    instructions.push(load);

                    // Increment coroutine_idx (ready for next coroutine)
                    coroutine_idx += 1;

                    // Set operation to syscall
                    let mut function = 0b001;
                    // Set syscall to run function
                    function = function << 5 | 16;
                    instructions.push(function);
                    continue 'chain;
                }
            }
        }
        // Add instruction if integer operation (i.e. not jump)
        for (name, code) in INT_OPERATIONS {
            if let Some(value) = int_fields.get(name) {
                // Set first 3 bits to instruction code
                let mut instruction: u8 = code;
                // Set rest 5 bits to immediate
                instruction = instruction << 5 | *value as u8;

                instructions.push(instruction);
                continue 'chain;
            }
        }
    }
    (coroutines, instructions)
}
