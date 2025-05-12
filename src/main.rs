use std::io;
use std::fmt;
use rand::Rng;
mod cgpa;
mod shunting_yard;
use cgpa::cgpa_calc;
use shunting_yard::{tokenize, parse, evaluate};
mod operation;
use operation::{addition, subtraction, multiplication, division};

enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    CgpaCalculator,
}

#[allow(dead_code)]
#[derive(PartialEq)]
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
            Operation::CgpaCalculator => write!(f, "CGPA Calculator")
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
            Token::Number(_) => write!(f, "1.0"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

fn main() {
    let add_op = Operation::Addition;
    let sub_op = Operation::Subtraction;
    let div_op = Operation::Division;
    let mul_op = Operation::Multiplication;
    let cgpa_op = Operation::CgpaCalculator;

    println!(
        "\twelcome to the rust CLI calculator\t
        Enter the operation you would like to perform\t
        1. {} \t
        2. {} \t 
        3. {} \t 
        4. {} \t 
        5. {} \t 
        Any other integer element implements freestyle calculation (i.e. outside 1..=5) \t 
        Enter 'end' to quit. Need more info? enter 'info'", 
        add_op, sub_op, div_op, mul_op, cgpa_op
    );

    let mut option_type = String::new();

    io::stdin()
        .read_line(&mut option_type)
        .expect("Failed to read input");

    if option_type.trim().to_lowercase() == "end" {
        panic!("Quitting operation");
    }

    let option_type = option_type.trim();

    match option_type.parse::<i32>() {
        Ok(num) if (1..=5).contains(&num) => {
            perform_calculation(num);
        }
        Ok(num) => {
            println!("Since you wouldn't fancy a single operation");
            let _ = simple_calc(Ok(num));
        }
        Err(_) => {
            // println!("Since you wouldn't fancy a single operation");
            // let _ = simple_calc(Err(option_type.to_string()));
            if option_type.trim().to_lowercase() == "info" {
                let _ = Err::<String, ()>(more_info());
            }
        }
    }

    // let option_type = option_type.trim().parse().expect("Failed to read input");

    // if !(1..=4).contains(&option_type) {
    //     println!("Since you wouldn't fancy a single operation");
    //     let _ = simple_calc(Ok(option_type));
    // } else if (1..=4).contains(&option_type) {
    //     perform_calculation(option_type);
    // } else {
    //     println!("Thank you for your time");
    // }
}

fn more_info() {
    println!("These are the expected features");

}

fn operation_to_symbol(param: &str) -> Option<&'static str> {
    match param {
        "Addition" => Some("+"),
        "Subtraction" => Some("-"),
        "Division" => Some("/"),
        "Multiplication" => Some("*"),
        "CGPA calculator" => Some("5.0 scale"),
        _ => None,
    }
}

fn process_calc(input: &str) -> Result<String, String> {
    let tokens = tokenize(input)?;
    let postfix = parse(tokens)?;
    let result = evaluate(postfix)?;
    Ok(result)
}

fn simple_calc(param: Result<i32, String>) -> Result<&'static str, String> {
    let add_op = Token::Plus;
    let sub_op = Token::Minus;
    let div_op = Token::Divide;
    let mul_op = Token::Multiply;

    let secret_number1: u32 = rand::rng().random_range(1..=10);
    let secret_number2: u32 = rand::rng().random_range(1..=10);
    let secret_number3: u32 = rand::rng().random_range(1..=10);
    let secret_number4: u32 = rand::rng().random_range(1..=10);
    let secret_number5: u32 = rand::rng().random_range(1..=10);

    match param {
        Ok(_) => {
            'counting: loop {
                // let token = tokenize(param)?;
                // let postfix = parse(token)?;
                // evaluate(postfix);

                println!(
                    "Enter string operation you would like to perform\n e.g. {} {} ({} {} {}) {} {} {} {}",
                    secret_number1, add_op, secret_number2, sub_op, secret_number3, div_op, secret_number4, mul_op, secret_number5
                );

                let mut calculation = String::new();
                io::stdin()
                    .read_line(&mut calculation)
                    .expect("failed to read input");
                let calculation = calculation.trim();

                match process_calc(&calculation) {
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
            println!("Selected operation, {}", chosen_operation);

            if chosen_operation == "CGPA calculator" {
                cgpa_calc();
                return;
            }

            if let Some(symbol) = operation_to_symbol(chosen_operation) {
                // println!("Symbol: {}", symbol);
                loop {
                    
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
                            "Addition" => {
                                let result: i32 = addition(first_input, second_input);
                                println!(
                                    "The result of your calculation: {} {} {} = {}",
                                    first_input, symbol, second_input, result
                                );
                                break;
                            }
                            "Subtraction" => {
                                let result: i32 = subtraction(first_input, second_input);
                                println!(
                                    "The result of your calculation: {} {} {} = {}",
                                    first_input, symbol, second_input, result
                                );
                                break;
                            }
                            "Division" => match division(first_input, second_input) {
                                Ok(result) => {
                                    println!(
                                        "The result of your calculation: {} {} {} = {:.3}",
                                        first_input, symbol, second_input, result
                                    );
                                    break;
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                    continue;
                                }
                            },
                            "Multiplication" => {
                                let result: i32 = multiplication(first_input, second_input);
                                println!(
                                    "The result of your calculation: {} {} {} = {}",
                                    first_input, symbol, second_input, result
                                );
                                break;
                            },
                            _ => {
                                panic!("Unexpected operation");
                            }
                    }
                }
                
            } 
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }

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
        5 => Ok("CGPA calculator"),
        _ if op_code < 1 => Err("Number too small".to_string()),
        _ if op_code > 5 => Err("Number too large".to_string()),
        _ => Err("Invalid input".to_string()),
    }
}  