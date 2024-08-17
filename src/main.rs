use std::{env, fs, io, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("You must give the program's path as an argument. (ex: hello.bf)");
        process::exit(1);
    }

    let code_text: String = match fs::read_to_string(&args[1]) {
        Ok(program_text) => program_text,
        Err(err) => {
            eprintln!(
                "Error reading file. Make sure the path exists. Err: {:?}",
                err
            );
            process::exit(1);
        }
    };
    let mut cells: [u8; 65535] = [0; 65535];
    let mut cell_index: usize = 0;

    parse_code(cells.clone(), cell_index.clone(), &code_text);
}

fn parse_code(mut cells: [u8; 65535], mut cell_index: usize, code: &str) {
    let code_chars: Vec<char> = code.chars().collect();
    let mut bracket_index: usize = 0;
    let mut bracket_indexes: [usize; 100] = [0; 100];
    let mut i = 0;

    while i < code_chars.len() {
        match code_chars[i] {
            '>' => {
                if cell_index < cells.len() - 1 {
                    cell_index += 1;
                } else {
                    cell_index = 0;
                }
            }
            '<' => {
                if cell_index > 0 {
                    cell_index -= 1;
                } else {
                    cell_index = cells.len() - 1;
                }
            }
            '+' => {
                cells[cell_index] = cells[cell_index].wrapping_add(1);
            }
            '-' => {
                cells[cell_index] = cells[cell_index].wrapping_sub(1);
            }
            '.' => {
                print!("{}", cells[cell_index] as char)
            }
            ',' => {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read input.");

                let input_chars: Vec<u8> = input
                    .chars()
                    .filter(|&c| c.is_ascii())
                    .map(|c| c as u8)
                    .collect();

                if let Some(mut c) = input_chars.get(0) {
                    if *c == 10 {
                        c = &0;
                    }
                    cells[cell_index] = *c;
                } else {
                    eprintln!("Enter a valid input.");
                    process::exit(1);
                }
            }
            '[' => {
                bracket_indexes[bracket_index] = i + 1;
                bracket_index += 1;
            }
            ']' => {
                if cells[cell_index] == 0 {
                    bracket_index = bracket_index.wrapping_sub(1);
                } else {
                    i = bracket_indexes[bracket_index - 1];
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
}
