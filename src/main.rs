use std::io;
use std::fmt;

enum Operation {
    Addition,
    Subtraction,
    Division,
    Multiplication
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

    println!("Enter the operation you would like to perform\n 1.{} \n 2.{} \n 3.{} \n 4.{} \n Any other element implements freestlye calc \n else enter 'end' to quit. ", add_op, sub_op, div_op, mul_op);

    let mut option_type = String::new();

    io::stdin()
        .read_line(&mut option_type)
        .expect("Failed to read input");

    let option_type: i32 = option_type.trim().parse().expect("Failed to read input");

    // note, we expect the number, eg 1 to be parsed into as option type of 32 bit integer
    // "1" is addition. 
    // "2" is subtraction
    // "3" is division
    // "4" is multiplication

    if option_type != 1..=4 {
        simple_calc(option_type);
    } else {
        perform_calculation(option_type);
    } 
}  
   

fn simple_calc(param: i32) {

    Println("Enter string operation you would like to perform\n e.g. {}  ")

    let mut calculation = String::new();
    io::stdin()
        .read_line(&mut calculation)
        .expect("faield to read input");



    let chosen_operation = dissplay;

    let mut count = 0;

    loop {
        


        if count == 5 {
            break
        }
        count += 1
    } 
}
   
   
fn perform_calculation(params: i32 ) {

    // 1 is addition
    // 2 is subtraction
    // 3 is division
    // 4 is multiplication

    println!("Selected operation, {}", params);

    if let Some(symbol) = operation_to_symbol(op) {
        println("Symbol: {}", symbol);

        let result = perform_calculation(symbol, first_input, second_input)

        println!("Please enter the first input");
        let mut first_input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let first_input: i32 = first_input
                                    .trim()
                                    .parse()
                                    .expect("Faield to parse number");

        println!("Please enter the second input");
        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let second_input: i32 = second_input
                                    .trim()
                                    .parse()
                                    .expect("Faield to parse number");

        

    } else {
        println!("No symbol available for this operation.");
    }

    
    
    


        let chosen_operation = display(params)
        

        match chosen_operation {
            Ok(op) => {
    
                match op => {
                    "Addition" => {
                        let result = addition(first_input, second_input),
                        println("the result of your calculation: {}", result);
                    }
                    "Subtraction" => {
                        let result = subtraction(input),
                        println("the result of your calculation: {}", result);
                    }
                    "Division" => {
                        let result = division(input),
                        println("the result of your calculation: {}", result);
                    }
                    "Multiplication" => {
                        let result = multiplication(input),
                        println("the result of your calculation: {}", result);
                    }
                    _ => {
                        panic!("Unexpected operation", op);
                    }
                }
            }
            Err(e) => panic!("Error: {}", e),
        }

}


//fn addition(value: Vec<f64>) {
//    for 
//}




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
