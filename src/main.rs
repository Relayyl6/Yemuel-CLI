use std::io;
use std::fmt;
use rand::Rng;

enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Addition => write!(f, "Addition"),
            Operation::Subtraction => write!(f, "Subtraction"),
            Operation::Division => write!(f, "Division"),
            Operation::Multiplication => write!(f, "Multiplication"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Divide => write!(f, "/"),
            Token::Multiply => write!(f, "*"),
            _ => Ok(()), // Handle other cases
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_number = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c.is_digit(10) || c == '.' {
            current_number.push(c);
            chars.next();
        } else {
            if !current_number.is_empty() {
                let num = current_number
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid number: {}", current_number))?;
                tokens.push(Token::Number(num));
                current_number.clear();
            }

            match c {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '/' => tokens.push(Token::Divide),
                '*' => tokens.push(Token::Multiply),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                _ => return Err(format!("Invalid character: {}", c)),
            }
            chars.next();
        }
    }

    if !current_number.is_empty() {
        let num = current_number
            .parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", current_number))?;
        tokens.push(Token::Number(num));
    }
    Ok(tokens)
}

fn parse(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output_stack = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output_stack.push(token),
            Token::LParen => operator_stack.push(token),
            Token::RParen => {
                while let Some(op) = operator_stack.pop() {
                    if op == Token::LParen {
                        break;
                    }
                    output_stack.push(op);
                }
            }
            _ => {
                while let Some(op) = operator_stack.last() {
                    if op == &Token::LParen {
                        break;
                    }
                    if precedence(&token) <= precedence(op) {
                        output_stack.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token);
            }
        }
    }
    while let Some(op) = operator_stack.pop() {
        if op == Token::LParen {
            return Err("Mismatched parenthesis".to_string());
        }
        output_stack.push(op);
    }

    Ok(output_stack)
}

fn precedence(token: &Token) -> u8 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

fn operation_to_symbol(param: &str) -> Option<&'static str> {
    match param {
        "Addition" => Some("+"),
        "Subtraction" => Some("-"),
        "division" => Some("/"),
        "Multiplication" => Some("*"),
        _ => None,
    }
}

fn main() {
    let add_op = Operation::Addition;
    let sub_op = Operation::Subtraction;
    let div_op = Operation::Division;
    let mul_op = Operation::Multiplication;

    println!(
        "welcome to the rust CLI calculator\nEnter the operation you would like to perform\n 1.{} \n 2.{} \n 3.{} \n 4.{} \n Any other element implements freestyle calc \n Enter 'end' to quit.",
        add_op, sub_op, div_op, mul_op
    );

    let mut option_type = String::new();

    io::stdin()
        .read_line(&mut option_type)
        .expect("Failed to read input");

    if option_type.trim().to_lowercase() == "end" {
        panic!("Quitting operation");
    }

    let option_type: i32 = option_type.trim().parse().expect("Failed to read input");

    if !(1..=4).contains(&option_type) {
        simple_calc(option_type);
    } else if (1..=4).contains(&option_type) {
        perform_calculation(option_type);
    } else {
        println!("Thank you for your time");
    }
}

fn simple_calc(param: i32) -> Result<&'static str, String> {
    let add_op = Operation::Addition;
    let sub_op = Operation::Subtraction;
    let div_op = Operation::Division;
    let mul_op = Operation::Multiplication;

    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);

    let mut count = 0;

    match param {
        Ok(value) if !(1..=4).contains(&value) => {
            'counting: loop {
                println!(
                    "Enter string operation you would like to perform\n e.g. {}, {}, {}, {}, {}, {}, {}, {}",
                    secret_number, add_op, secret_number, sub_op, secret_number, div_op, secret_number, mul_op
                );

                let mut calculation = String::new();
                io::stdin()
                    .read_line(&mut calculation)
                    .expect("failed to read input");

                match tokenize(&calculation) {
                    Ok(tokens) => {
                        println!("The result of your calculation is {}", tokens);
                        println!("Would you like to continue? Y/N");

                        let mut question = String::new();
                        io::stdin()
                            .read_line(&mut question)
                            .expect("Failed to read input");

                        let question = question.trim();

                        if question == "Y" {
                            continue;
                        } else if question == "N" {
                            break 'counting;
                        } else {
                            panic!("No option selected. Panic notion");
                        }
                    }
                    Err(e) => {
                        panic!("Error during tokenization: {}", e);
                    }
                }

                count += 1;
            }
        }
        Ok(value) => {
            if param == "end" {
                println!("\nThank you for using this CLI calculator\n");
            }
        }
        Err(e) => {
            panic!("Error: Invalid input: {}", e);
        }
    }

    Ok("Calculation complete")
}

fn perform_calculation(params: i32) {
    // let chosen_operation = display(params);

    match display(params) {
        Ok(chosen_operation) => {
            println!("Selected operation, {}", params);

            if let Some(symbol) = operation_to_symbol(chosen_operation) {
                println!("Symbol: {}", symbol);

                println!("Please enter your first input: ");
                let mut first_input = String::new();
                io::stdin()
                    .read_line(&mut first_input)
                    .expect("Failed to read input");
                let first_input: i32 = first_input
                    .trim()
                    .parse()
                    .expect("Failed to parse number");

                println!("Please enter your second input: ");
                let mut second_input = String::new();
                io::stdin()
                    .read_line(&mut second_input)
                    .expect("Failed to read input");
                let second_input: i32 = second_input
                    .trim()
                    .parse()
                    .expect("Failed to parse number");

                match chosen_operation {
                    Ok(op) => match op {
                        "Addition" => {
                            let result: i32 = addition(first_input, second_input);
                            println!(
                                "The result of your calculation: {} {} {} = {}",
                                first_input, symbol, second_input, result
                            );
                        }
                        "Subtraction" => {
                            let result: i32 = subtraction(first_input, second_input);
                            println!(
                                "The result of your calculation: {} {} {} = {}",
                                first_input, symbol, second_input, result
                            );
                        }
                        "Division" => match division(first_input, second_input) {
                            Ok(result) => {
                                println!(
                                    "The result of your calculation: {} {} {} = {}",
                                    first_input, symbol, second_input, result
                                );
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        },
                        "Multiplication" => {
                            let result: i32 = multiplication(first_input, second_input);
                            println!(
                                "The result of your calculation: {} {} {} = {}",
                                first_input, symbol, second_input, result
                            );
                        }
                        _ => {
                            panic!("Unexpected operation");
                        }
                    },
                    Err(e) => panic!("Error: {}", e),
                }
            } else {
                println!("No symbol available for this operation.");
            }

                }
        Err(e) => {
            println!("Error: {}", e)
        }
    }

}

fn addition(first_param: i32, second_param: i32) -> i32 {
    let sum = first_param + second_param;
    sum
}

fn subtraction(first_param: i32, second_param: i32) -> i32 {
    let difference = first_param - second_param;
    difference
}

fn division(first_param: i32, second_param: i32) -> Result<i32, String> {
    let first_param_as_f64 = first_param as f64;
    let second_param_as_f64 = second_param as f64;

    match (first_param_as_f64, second_param_as_f64) {
        (0.0, _) => Err("Error: Division by zero is not allowed for the first parameter".to_string()),
        (_, 0.0) => Err("Error: Division by zero is not allowed for the second parameter".to_string()),
        _ => {
            let dividend: f64 = first_param_as_f64 / second_param_as_f64;
            let dividend_as_i32 = dividend.round() as i32;
            Ok(dividend_as_i32)
        }
    }
}

fn multiplication(first_param: i32, second_param: i32) -> i32 {
    let multiple = first_param * second_param;
    multiple
}

fn display(option_input: i32) -> Result<&'static str, String> {
    let option_input: f64 = option_input.into();

    if option_input.fract() != 0.0 {
        return Err("Input must be an integer".to_string());
    }

    let op_code = option_input as i32;

    match op_code {
        1 => Ok("Addition"),
        2 => Ok("Subtraction"),
        3 => Ok("Division"),
        4 => Ok("Multiplication"),
        5 => Ok("Higher order calculations pending"),
        _ if op_code < 1 => Err("Number too small".to_string()),
        _ if op_code > 5 => Err("Number too large".to_string()),
        _ => Err("Invalid input".to_string()),
    }
}
