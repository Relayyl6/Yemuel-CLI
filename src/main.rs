use std::io;
use std::fmt;
use rand::Rng;

enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    CgpaCalculator,
}

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
        Any other integer element implements freestyle calculation \t 
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
        Ok(num) if (1..=4).contains(&num) => {
            perform_calculation(num);
        }
        Ok(num) if num == 5 => {
            cgpa_calc()
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

fn cgpa_calc() {

    println!("How many courses do you have?");
    
    let mut num_of_courses = String::new();
    io::stdin()
        .read_line(&mut num_of_courses)
        .expect("Enter a valid student number");
    
    let num_of_courses: i32 = num_of_courses.trim()
                                            .parse()
                                            .expect("Please enter a valid number");
    println!("This is the number of courses: {}", num_of_courses);

    // let count: &i32 = &num_of_courses;

    let mut count = 0;
    let mut total_credit_load = 0;
    let mut total_grade_points  = 0;
    let remaining = 22;

    while count < num_of_courses {
        println!("Credit load left = {}", remaining-total_credit_load);

        // Course code 
        println!("Enter Course Code: ");
        let mut course_code = String::new();
        io::stdin() 
            .read_line(& mut course_code)
            .expect("Failed to read input");
        let _course_code = course_code.trim();
                                          

        // Course credit
        println!("Enter Course Credit: ");
        let mut course_credit = String::new();
        io::stdin() 
            .read_line(& mut course_credit)
            .expect("Failed to read input");
        let course_credit: i32 = course_credit.trim()
                                              .parse()
                                              .expect("Make enter a valid number");

        // Grade input
        println!("Enter Grade (A-F): ");
        let mut grade_score = String::new();
        io::stdin() 
            .read_line(& mut grade_score)
            .expect("Failed to read input");
        // .chars() turns a string slice into an iterator over its characters, .next() gets the first character from that iterator
        let grade_score = grade_score.trim()
                                     .chars()
                                     .next()
                                     .expect("No grade entered");

        let grade_point =  grade_calculation(grade_score);
        let grade_point: i32 = grade_point.trim()
                                     .parse()
                                     .expect("Invalid grade point");

        total_grade_points += grade_point * course_credit;
        total_credit_load += course_credit;
        count += 1;
                             
        if total_credit_load > remaining {
            panic!("Exceeded maximum credit load");
        }


        let cgpa = total_grade_points as f64 / total_credit_load as f64;
        let formatted_cgpa = format!("{:.2}", cgpa);
        println!("The overall cgpa of the {} number of courses is {}", num_of_courses, formatted_cgpa)
    }
}

fn grade_calculation(param: char) -> String {
    match param.to_ascii_uppercase() {
        'A' => String::from("5"),
        'B' => String::from("4"),
        'C' => String::from("3"),
        'D' => String::from("2"),
        'E' => String::from("1"),
        'F' => String::from("0"),
        _ => panic!("Invalid day")
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

fn evaluate(postfix: Vec<Token>) -> Result<String, String> {
    let mut stack = Vec::new();

    for token in postfix {
        match token {
            Token::Number(num) => stack.push(num),
            _ => {
                let right: i32 = stack.pop().ok_or("Missing operand")? as i32; // .ok_or Converts the Option returned by pop() into a Result.
                let left: i32 = stack.pop().ok_or("Missing operand")? as i32;
                let result = match token {
                    Token::Plus => addition(left, right) as f64,
                    Token::Minus => subtraction(left, right) as f64,
                    Token::Multiply => multiplication(left, right) as f64,
                    Token::Divide => match division(left, right) {
                        Ok(result) => result as f64,
                        Err(e) => return Err(format!("Error: {e}"))
                        }
                    _ => return Err("Invalid operator in evaluation".to_string()),
                };
                stack.push(result);
            },
        }
    }
    
    if stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }

    Ok(stack[0].to_string())
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

            if let Some(symbol) = operation_to_symbol(chosen_operation) {
                // println!("Symbol: {}", symbol);

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
                }
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
        (_, 0.0) => Err("Error: Division of zero is not allowed for the second parameter".to_string()),
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
        5 => Ok("CGPA calculator"),
        _ if op_code < 1 => Err("Number too small".to_string()),
        _ if op_code > 5 => Err("Number too large".to_string()),
        _ => Err("Invalid input".to_string()),
    }
}  