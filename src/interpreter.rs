pub fn execute(program: &str) {
    let tokens = tokenize(&program);

    if !check_syntax(&tokens) {
        print!("Syntax error: unbalanced brackets");
        return;
    }

    start_executing(&tokens);
}

#[derive(Clone)]
enum Token {
    IncreaseV,
    DecreaseV,
    IncreaseP,
    DecreaseP,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

fn tokenize(program: &str) -> Vec<Token> {
    let characters: Vec<char> = program.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    for c in characters {
        match c {
            '+' => tokens.push(Token::IncreaseV),
            '-' => tokens.push(Token::DecreaseV),
            '>' => tokens.push(Token::IncreaseP),
            '<' => tokens.push(Token::DecreaseP),
            '.' => tokens.push(Token::Output),
            ',' => tokens.push(Token::Input),
            '[' => tokens.push(Token::LoopStart),
            ']' => tokens.push(Token::LoopEnd),
            _ => continue,
        }
    }

    tokens
}

fn check_syntax(tokens: &Vec<Token>) -> bool {
    let mut loop_start_counter = 0;
    let mut loop_end_counter = 0;

    for token in tokens {
        match token {
            Token::LoopStart => loop_start_counter += 1,
            Token::LoopEnd => loop_end_counter += 1,
            _ => continue,
        }
    }

    if loop_start_counter != loop_end_counter {
        return false;
    }

    true
}


fn start_executing(program_tokens: &Vec<Token>) {
    let mut cells: Vec<u32> = vec![0; 30000];
    let mut pointer = 0;
    let mut ind = 0;
    let mut counter = 0;

    loop {
        if counter >= program_tokens.len() {
            break;
        }
        
        match program_tokens[counter] {
            Token::IncreaseV => cells[pointer] += 1,
            Token::DecreaseV => cells[pointer] -= 1,
            Token::IncreaseP => pointer += 1,
            Token::DecreaseP => pointer -= 1,
            Token::Output => print!("{}", cells[pointer] as u8 as char),
            Token::Input => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                cells[pointer] = input.chars().next().unwrap() as u32;
            },
            Token::LoopStart => {
                ind = counter;                
            }
            Token::LoopEnd => {
                if cells[pointer] != 0 {
                    counter = ind;
                }
            }
        }
        counter += 1;
    }

    println!("");
}
