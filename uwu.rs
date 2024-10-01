use std::env;
use std::fs;
use std::io::{self, Read, Write};

struct UwuInterpreter {
    memory: Vec<u8>,
    pointer: usize,
    program: Vec<char>,
    program_counter: usize,
}

impl UwuInterpreter {
    fn new(program: &str) -> Self {
        UwuInterpreter {
            memory: vec![0; 30000],
            pointer: 0,
            program: program.chars().collect(),
            program_counter: 0,
        }
    }

    fn run(&mut self) {
        while self.program_counter < self.program.len() {
            let command = self.get_command();
            self.execute_command(command);
        }
    }

    fn get_command(&mut self) -> char {
        if self.program_counter + 2 < self.program.len() {
            match &self.program[self.program_counter..=self.program_counter+2] {
                ['>','w','>'] => { self.program_counter += 2; return '>' },
                ['<','w','<'] => { self.program_counter += 2; return '<' },
                ['u','w','u'] => { self.program_counter += 2; return '+' },
                ['n','w','n'] => { self.program_counter += 2; return '-' },
                ['o','w','o'] => { self.program_counter += 2; return '.' },
                ['-','w','-'] => { self.program_counter += 2; return ',' },
                ['o','w','<'] => { self.program_counter += 2; return '[' },
                ['>','w','o'] => { self.program_counter += 2; return ']' },
                ['*','w','*'] => {
                    while self.program_counter < self.program.len() && self.program[self.program_counter] != '\n' {
                        self.program_counter += 1;
                    }
                    return '\0'
                },
                _ => {}
            }
        }
        self.program_counter += 1;
        '\0'
    }

    fn execute_command(&mut self, command: char) {
        match command {
            '>' => self.pointer = (self.pointer + 1) % self.memory.len(),
            '<' => self.pointer = (self.pointer + self.memory.len() - 1) % self.memory.len(),
            '+' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
            '-' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
            '.' => print!("{}", self.memory[self.pointer] as char),
            ',' => {
                let mut input = [0];
                io::stdin().read_exact(&mut input).unwrap();
                self.memory[self.pointer] = input[0];
            },
            '[' => {
                if self.memory[self.pointer] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        let cmd = self.get_command();
                        if cmd == '[' { depth += 1; }
                        else if cmd == ']' { depth -= 1; }
                    }
                }
            },
            ']' => {
                if self.memory[self.pointer] != 0 {
                    let mut depth = 1;
                    loop {
                        if self.program_counter == 0 { break; }
                        self.program_counter -= 1;
                        if self.program_counter >= 2 {
                            let cmd = match &self.program[self.program_counter-2..=self.program_counter] {
                                ['o','w','<'] => '[',
                                ['>','w','o'] => ']',
                                _ => '\0'
                            };
                            if cmd == ']' { depth += 1; }
                            else if cmd == '[' {
                                depth -= 1;
                                if depth == 0 { break; }
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

fn transpile(bf_code: &str, keep_comments: bool, keep_newlines: bool, keep_whitespace: bool) -> String {
    let mut uwu_code = String::new();
    let mut in_comment = false;
    let mut line_has_code = false;

    let chars: Vec<char> = bf_code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if in_comment {
            if chars[i] == '\n' {
                in_comment = false;
                if (line_has_code || keep_comments || keep_newlines) && keep_whitespace {
                    uwu_code.push('\n');
                }
                line_has_code = false;
            } else if keep_comments && keep_whitespace {
                uwu_code.push(chars[i]);
            }
            i += 1;
            continue;
        }

        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            if keep_comments {
                uwu_code.push_str("*w*");
                in_comment = true;
                i += 2;
                continue;
            } else {
                in_comment = true;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
        }

        match chars[i] {
            ' ' | '\t' => {
                if keep_whitespace {
                    uwu_code.push(chars[i]);
                }
            }
            '\n' => {
                if (line_has_code || keep_newlines) && keep_whitespace {
                    uwu_code.push('\n');
                }
                line_has_code = false;
            }
            '>' => {
                uwu_code.push_str(">w>");
                line_has_code = true;
            }
            '<' => {
                uwu_code.push_str("<w<");
                line_has_code = true;
            }
            '+' => {
                uwu_code.push_str("uwu");
                line_has_code = true;
            }
            '-' => {
                uwu_code.push_str("nwn");
                line_has_code = true;
            }
            '.' => {
                uwu_code.push_str("owo");
                line_has_code = true;
            }
            ',' => {
                uwu_code.push_str("-w-");
                line_has_code = true;
            }
            '[' => {
                uwu_code.push_str("ow<");
                line_has_code = true;
            }
            ']' => {
                uwu_code.push_str(">wo");
                line_has_code = true;
            }
            _ => {}
        }
        i += 1;
    }

    uwu_code
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [OPTIONS] [FILE]... <COMMAND>", args[0]);
        eprintln!("\nCommands:");
        eprintln!("  convert  Convert a .bf file to .uwu");
        eprintln!("\nArguments:");
        eprintln!("  [FILE]...  Run the provided .uwu file");
        std::process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "convert" => {
            if args.len() < 3 {
                eprintln!("Usage: {} convert [INPUT]", args[0]);
                eprintln!("\nArguments:");
                eprintln!("  [OUTPUT]  Path to the output .uwu file");
                eprintln!("\nOptions:");
                eprintln!("  -c, --comments    keep comments in the file");
                eprintln!("  -w, --whitespace  keep whitespace in the file");
                eprintln!("  -n, --newline     keep newlines in the file");
                eprintln!("  -a, --all         enable all extras in the file");
                std::process::exit(1);
            }
            
            let input_file = &args[2];
            let output_file = {
                let mut parts = input_file.split('.');
                let name = parts.next().unwrap();
                format!("{}.uwu", name)
            };
            
            if !input_file.ends_with(".bf") {
                eprintln!("Error: Input file must have a .bf extension");
                std::process::exit(1);
            }

            let bf_code = fs::read_to_string(input_file)?;
            
            let mut keep_comments = false;
            let mut keep_whitespace = false;
            let mut keep_newlines = false;
            
            for arg in &args[3..] {
                match arg.as_str() {
                    "-c" | "--comments" => keep_comments = true,
                    "-w" | "--whitespace" => keep_whitespace = true,
                    "-n" | "--newline" => keep_newlines = true,
                    "-a" | "--all" => {
                        keep_comments = true;
                        keep_whitespace = true;
                        keep_newlines = true;
                    }
                    _ => {
                        eprintln!("Error: Unknown option {}", arg);
                        std::process::exit(1);
                    }
                }
            }
            
            let uwu_code = transpile(&bf_code, keep_comments, keep_newlines, keep_whitespace);
            let mut file = fs::File::create(&output_file)?;
            
            file.write_all(uwu_code.as_bytes())?;
            println!("written converted code to {output_file}");
        },
        _ => {
            let file_path = &args[1];
            if !file_path.ends_with(".uwu") {
                eprintln!("Error: File must have a .uwu extension");
                std::process::exit(1);
            }
            
            let program = fs::read_to_string(file_path)?;
            let mut interpreter = UwuInterpreter::new(&program);
            interpreter.run();
        }
    }

    Ok(())
}