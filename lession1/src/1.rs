fn divide_numbers(a: f64, b: f64) -> Result<f64, &'static str> {
    if a < 0.0 || b < 0.0 {
        return Err("Negative values are not allowed");
    }
    if b == 0.0 {
        return Err("Division by zero is not allowed");
    }
    Ok(a / b)
}

fn main() {
    let x = 10.0;
    let y = 0.0;

    let result = divide_numbers(x, y);
    
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("An error occurred: {}", e),
    }

    let result_2 = divide_numbers(x, 5.0);
    if let Ok(value) = result_2 {
        println!("Second result: {}", value);
    }
    let result_3 = divide_numbers(-10.0, 5.0);
    match result_3 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("An error occurred: {}", e),
    }
}