

pub fn addition(first_param: i32, second_param: i32) -> i32 {
    let sum = first_param + second_param;
    sum
}

pub fn subtraction(first_param: i32, second_param: i32) -> i32 {
    let difference = first_param - second_param;
    difference
}

pub fn division(first_param: i32, second_param: i32) -> Result<f64, String> {
    let first_param_as_f64 = first_param as f64;
    let second_param_as_f64 = second_param as f64;

    match (first_param_as_f64, second_param_as_f64) {
        (0.0, _) => Err("Error: Division by zero is not allowed for the first parameter".to_string()),
        (_, 0.0) => Err("Error: Division of zero is not allowed for the second parameter".to_string()),
        _ => {
            let dividend: f64 = first_param_as_f64 / second_param_as_f64;

            Ok(dividend)
        }
    }
}

pub fn multiplication(first_param: i32, second_param: i32) -> i32 {
    let multiple = first_param * second_param;
    multiple
}