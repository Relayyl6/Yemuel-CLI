mod operation;
use operation::{addition, subtraction, multiplication, division};

#[derive(PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> { 
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

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
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

pub fn precedence(token: &Token) -> u8 {
    match token {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

pub fn evaluate(postfix: Vec<Token>) -> Result<String, String> {
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
