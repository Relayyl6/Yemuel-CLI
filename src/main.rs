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
            Operation::Multiplication => write!(f, "Multiplication"),
            Operation::Division => write!(f, "Division"),
        }
    }
}

fn main() {

    let add_op = Operation::Addition;
    let sub_op = Operation::Subtraction;
    let div_op = Operation::Division;
    let mul_op = Operation::Multiplication;

    println!("Enter the operation you would like to perform\n 1.{} \n 2.{} \n 3.{} \n 4.{}", add_op, sub_op, div_op, mul_op);

    let mut option_type = String::new();

    io::stdin()
        .read_line(&mut option_type)
        .expect("Failed to read input");

    let option_type: f64 = option_type.trim().parse().expect("Failed to read input");

    let chosen_option = display(option_type);
    match chosen_option {
        Ok(op) => println!("Selected operation: {}", op),
        Err(e) => panic!("Error: {}", e),
    }

    loop {


        if condition {
            break
        }
    }

    let mut count = 0,



   
}

//fn addition(value: Vec<f64>) {
//    for 
//}



fn display(score_input: f64) -> Result<&'static str, String> {
    if score_input.fract() != 0.0 {
        return Err("Ipnput must be an integer".to_string());
    }

    let op_code = score_input as i32;

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
