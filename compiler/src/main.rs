use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
struct JavaFile {
    pub name: String,
    pub abst: bool,
    pub extends: String,
    pub implements: String,
    pub fields: HashMap<String, i32>,
}

impl JavaFile {
    /// Parse a java source file and get relations
    pub fn from_file(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut signature = String::new();
        let mut toplevel = String::new();

        let mut comment_started = false;
        let mut line_comment_started = false;
        let mut signature_started = true;
        let mut block_level = 0;
        // Go through code and skip comments
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

            if signature_started {
                signature.push(current);
            }
            if signature_started && next == '\n' {
                signature_started = false;
            }

            if current == '{' {
                block_level += 1;
                continue;
            }
            if current == '}' {
                block_level -= 1;
                continue;
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

        println!("{}", signature);

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
        let mut fields = HashMap::new();

        for line in toplevel
            .trim()
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty() && x.ends_with(';'))
            .map(|x| x.strip_suffix(';').unwrap_or(x))
        {
            let tokens = line
                .split_whitespace()
                .zip(line.split_whitespace().skip(1))
                .zip(line.split_whitespace().skip(2))
                .zip(line.split_whitespace().skip(3))
                .zip(line.split_whitespace().skip(4));
            for ((((a, b), c), d), e) in tokens {
                println!("Line: {} {} {} {} {}", a, b, c, d, e);
                if (a, b, d) == ("static", "int", "=") {
                    if let Ok(num) = e.parse() {
                        fields.insert(String::from(c), num);
                    }
                }
            }
        }

        Ok(JavaFile {
            name,
            abst: signature.contains("abstract"),
            extends,
            implements,
            fields,
        })
    }
}

fn main() {
    let args = env::args();
    let input_path = args
        .skip(1)
        .next()
        .expect("Please supply the path as the first argument");
    let java = Path::new(&input_path);
    if !java.is_dir() {
        panic!("Passed path is not a directory");
    }
    if !java.exists() {
        panic!("Passed path does not exist");
    }
    let java_dir = java.read_dir().unwrap();
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
        .map(|path| JavaFile::from_file(&path))
        .filter_map(|e| e.ok())
        .collect::<Vec<JavaFile>>();

    println!("{:#?}", java_objects);
}
