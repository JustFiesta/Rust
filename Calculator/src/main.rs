use std::io;

fn main() {
    println!("Calculator - choose operation (+, -, /, *, power, root)");
    let mut user_operation = String::new();
    io::stdin().read_line(&mut user_operation).expect("Failed to read input");

    println!("Enter first number: ");
    let mut user_number_a = String::new();
    io::stdin().read_line(&mut user_number_a).expect("Failed to read input");
    let user_number_a: f64 = user_number_a.trim().parse().expect("Invalid input");

    println!("Enter second number: ");
    let mut user_number_b = String::new();
    io::stdin().read_line(&mut user_number_b).expect("Failed to read input");
    let user_number_b: f64 = user_number_b.trim().parse().expect("Invalid input");

    let result = calculator(&user_operation.trim(), user_number_a, user_number_b);

    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

fn calculator(user_operation: &str, a: f64, b: f64) -> Result<f64, String> {
    match user_operation {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => Ok(a / b),
        "power" => Ok(a.powf(b)),
        "root" => Ok(a.sqrt()),
        _ => Err(format!("'{}' is not valid in this calculator", user_operation)),
    }
}
