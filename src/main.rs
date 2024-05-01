mod interpreter;

use std::env;
use interpreter::execute;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    parse_args(&args);
}

fn parse_args(args: &Vec<String>) {
    let mut args = args.clone();
    args.remove(0);

    if args.len() == 0 {
        idle();
        return;
    }

    let option = &args[0];

    exec_option(option, &args);
}

fn exec_option(option: &str, args: &Vec<String>) {
    match option {
        "-o" => {
            if args.len() < 2 {
                println!("Option -o requires the file path as argument");
            } else {
                open_file(&args[1]);
            }
        },
        "-h" => {
            print_help();
        },
        _ => {
            println!("Invalid option {}.", option);
        }
    }
}

fn open_file(file_path: &str) {
    let result = fs::File::open(file_path);

    match result {
        Ok(_file) => {
            let program = fs::read_to_string(file_path).unwrap();
            execute(&program);
        },
        Err(_) => {
            println!("Error opening file {}", file_path);
        }
    }
}

fn print_help() {
    println!(r#"Usage: brainfuck [option] <arg>
    
Options:
    -o <file_path>  Opens the specified file.
    -h              Prints this help message."#);
}

fn idle() {
    loop {
        let mut input = String::new();

        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        execute(input);
    }
}
