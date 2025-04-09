use std::io;
use std::fmt;

enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication
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


impl fmt::Display for Operation {  // implementing the Display trait from the std::fmt module for our Operation enum.
                                                            // &mut fmt::Formatter): A mutable reference to a formatter that handles the output
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { // &self: Borrows the current Operation instance
        match self {
            Operation::Addition => write!(f, "Addition"), // write! is a macro that writes formatted text to the formatter f
            Operation::Subtraction => write!(f, "Subtraction"),
            Operation::Division => write!(f, "Division"),
            Operation::Multiplication => write!(f, "Multiplication"),
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    // .chars() convers a string into an iterator over its characters
    // .peekable() allows you to look the next item in the iterator wthout consuming it
    let mut chars = input.chars().peekable(); 
    let mut current_number = String::new();

    // rust construct used in a loop to process characters from a peekable iterator.
    // chars.peek() returns an Option<&char> (an option return type with a borrowed char warranting the let Some(&c)) 
    // Some(&c) destructures the Option<&char> returned by peek()  
    // &c dereferences the reference to the character, so c becomes the actual character.
    while let Some(&c) = chars.peek() { 
        if c.is_whitespace() { // .is_whitespace() checks for whitepscae, new line, new tab etc
            chars.next();
            continue;
        }

        if c.is_digit(10) || c == '.' {
            current_number.push(c);
            chars.next();
        } else {
            if !current_number.is_empty() {
                let num = current_number.parse::<f64>()
                            .map_err(|_| format!("Invalid number: {}", current_number))?;
                tokens.push(Token::Number(num));
                current_number.clear();
            
            match c {
                "+" => tokens.push(Token::Plus),
                "-" => tokens.push(Token::Minus),
                "/" => tokens.push(Token::Divide),
                "*" => tokens.push(Token::Multiply),
                "(" => tokens.push(Token::LParen),
                ")" => tokens.push(Token::RParen),
            } // eg  "123 + 456" results in tokens = vec![Token::Number(123.0), Token::Plus, Token::Number(456.0)]
            chars.next();
        }

        if !current_number.is_empty() {
            let num = current_number.parse::<f64>()
                        .map_err(|_| format!("Invalid number: {}", current_number))?;
            tokens.push(Token::Number(num));

    }

    Ok(tokens)

    // eg of implementation in fn main
    // let input = "123 + 456";
    // match tokenize(input) {
    //     Ok(tokens) => {
    //         println!("Tokens: {:?}", tokens);
    //         // Process the tokens further (e.g., parsing or evaluation)
    //     }
    //     Err(e) => {
    //         println!("Error during tokenization: {}", e);
    //     }

    //     Output for "123 + 456":
    //         Tokens: [Number(123.0), Plus, Number(456.0)]
    //     Output for Invalid Input (e.g., "123 + abc"):
    //         Error during tokenization: Invalid number: abc
        

}

fn parse(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut operator = Vec::new()

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token)
        }
    }
}


fn operation_to_symbol(param: &str ) -> Option<&'static str> {
    match param {
        "Addition" => Some("+"),
        "Subtraction" => Some("-"),
        "division" => Some("/"),
        "Multiplication" => Some("*"),
        _ => None
    }
}

fn main() {

    let add_op = Operation::Addition;
    let sub_op = Operation::Subtraction;
    let div_op = Operation::Division;
    let mul_op = Operation::Multiplication;

    println!("welcome to the rust CLI calculator");
    println!("Enter the operation you would like to perform\n 1.{} \n 2.{} \n 3.{} \n 4.{} \n Any other element implements freestlye calc \n Enter 'end' to quit. ", add_op, sub_op, div_op, mul_op);

    let mut option_type = String::new();

    io::stdin()
        .read_line(&mut option_type)
        .expect("Failed to read input");

    if option_type.trim().to_lowercase() == "end" {
        panic!("Quitting operation");
    }

    let option_type: i32 = option_type.trim().parse().expect("Failed to read input");

    // note, we expect the number, eg 1 to be parsed into as option type of 32 bit integer
    // "1" is addition. 
    // "2" is subtraction
    // "3" is division
    // "4" is multiplication

    if !(1..=4).contains(&option_type) {
        simple_calc(option_type);
    } else if (1..=4).contains(&option_type) {
        perform_calculation(option_type);
    } else {
        println!("Thank you for your time")
    } 
}  
   

fn simple_calc(param: i32) -> Result<&static str, String> {

    // tokenizing the input, to seperate integeer from operation sign // ref fn tokenize

    let add_op = Operation::Addition;
    let sub_op = Operation::Subtraction;
    let div_op = Operation::Division;
    let mul_op = Operation::Multiplication;
    

    let mut count = 0;

    loop {

        Println("Enter string operation you would like to perform\n e.g. {}  ")
        
        let mut calculation = String::new();
        io::stdin()
            .read_line(&mut calculation)
            .expect("failed to read input");

        if param.trim() == "end" {
            println!("\nThank you for using this CLI calculator\n");
            break
        }
        count += 1
    } 
}
   
   
fn perform_calculation(params: i32) {

    // 1 is addition
    // 2 is subtraction
    // 3 is division
    // 4 is multiplication

    let chosen_operation = display(params);

    println!("Selected operation, {}", params);

    if let Some(symbol) = operation_to_symbol(chosen_operation) {
        println("Symbol: {}", symbol);

        println!("Please enter your first input: ");
        let mut first_input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let first_input: i32 = first_input
                                    .trim()
                                    .parse()
                                    .expect("Faield to parse number");

        println!("Please enter your second input: ");
        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let second_input: i32 = second_input
                                    .trim()
                                    .parse()
                                    .expect("Faield to parse number");

        // let result = perform_calculation(symbol, first_input, second_input)

        
        match chosen_operation {
            Ok(op) => {
                match op => {
                    "Addition" => {
                        let result: i32 = addition(first_input, second_input),
                        println("the result of your calculation: {} {} {} = {}", first_input, symbol, second_input, result);
                    }
                    "Subtraction" => {
                        let result: i32 = subtraction(first_input, second_input),
                        println("the result of your calculation: {} {} {} = {}", first_input, symbol, second_input, result);
                    }
                    "Division" => {
                        match division(first_input, second_input) {
                            Ok(result) => {
                                println!("the result of your calculation: {} {} {} = {}", first_input, symbol, second_input, result);
                            }
                            Err(e) => {
                                println!("Error: {}", e)
                            }
                        }
                    }
                    "Multiplication" => {
                        let result: i32 = multiplication(first_input, second_input),
                        println("the result of your calculation: {} {} {} = {}", first_input, symbol, second_input, result);
                    }
                    _ => {
                        panic!("Unexpected operation", op);
                    }
                }
            }
            Err(e) => panic!("Error: {}", e),
        }

    } else {
        println!("No symbol available for this operation.");
    }

}


fn addition(first_param: i32, second_param: i32) -> i32 {
    let sum = first_param + second_param;
    sum
}
fn subtraction(first_param: i32, second_param: i32) -> i32 {
    let difference = first_input - &second_input;
    difference
}
fn division(first_param: i32, second_param: i32) -> Result<i32, String> {
    let first_param_as_f64 = first_param as f64;
    let second_param_as_f64 = second_param as f64;
    // if first_param_as_f64 == 0 {
    //     panic!("Error: Division by zero is not allowed");
    // } else if second_param_as_f64 == 0.0 {
    //     panic!("Error: Math Error. Infinite solution");
    // } else {
    //     let dividend: f64 = first_param_as_f64 / second_param_as_f64;
    // }

    match (first_param_as_f64, second_param_as_f64) {
        (0.0, _) => Err("Error: Division by zero is not allowed for the first parameter".to_string()),
        (_, 0.0)=>  Err("Error: Division by zero is not allowed for the first parameter".to_string()),
        _ => {
            let dividend: f64 = first_param_as_f64 / second_param_as_f64;
            let dividend_as_i32 = dividend as i32;
            Ok(dividend_as_i32)
        },
    } 

    let dividend_as_i32 = dividend.round() as i32;
}
fn multiplication(first_param: i32, second_param: i32) -> i32 {
    let multiple = first_input * second_input;
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
