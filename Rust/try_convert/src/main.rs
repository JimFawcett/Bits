
//https://www.reddit.com/r/learnrust/comments/df65g9/checked_conversion_of_usize_to_f64/
fn checked_usize_to_f64(value: usize) -> Result<f64, &'static str> {
    let result = value as f64;
    if result as usize != value {
        return Err("cannot convert");
    }
    Ok(result)
}
fn unchecked_usize_to_f64(value: usize) -> f64 {
    value as f64
}

fn test1(value:usize) {
    let result = checked_usize_to_f64(value);
    match result {
        Ok(num) => println!("Successfully converted {:?} to {:?}", value, num),
        Err(e) => println!("Error: {}", e),
    }
}
fn test2(value:usize) {
    let result = unchecked_usize_to_f64(value);
    println!("converted {:?} to {:?}", value, result);
}

fn main() {
    test1(100);
    test2(100);
    test1(usize::max_value());
    test2(usize::max_value());
}
