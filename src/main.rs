use std::{error, fmt, io, num::ParseIntError};

fn get_input() -> PositiveInteger {
    loop {
        println!("Enter a positive number:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match parser(&mut input) {
            Err(e) => println!("Error parsing: {e}"),
            Ok(num) => return num,
        }
        continue
    
    }
}
fn parser(input: &mut str) -> Result<PositiveInteger, CreationError> {
    let out = input.trim().parse().map_err(|e| CreationError::NotNumber(e))?;
    let out = PositiveInteger::new(out)?;
    Ok(out)

}

fn factorial(n: i32) -> Result<i32, String> {
    if n == 0 {
        return Ok(1);
    }
    let result = n * factorial(n - 1)?;
    Ok(result)
}

fn print_factorial(n: i32) -> Result<(), String> {
    let result = factorial(n)?;
    println!("Factorial of {} is: {}", n, result);
    Ok(())
}
#[derive(PartialEq, Debug)]
struct PositiveInteger(i32);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    NotNumber(ParseIntError),
    TooBig,
}

impl PositiveInteger {
    fn new(value: i32) -> Result<PositiveInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x > 12 => Err(CreationError::TooBig),
            x => Ok(PositiveInteger(x)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "input is a number, but is negative. Do not want to use negatives".to_string(),
            CreationError::NotNumber(ref e) => format!("input is not a number: {}",e),
            CreationError::TooBig => "input is too big".to_string(),
        };
        f.write_str(&description)
    }
}

impl error::Error for CreationError {}
fn main() {
    let n = get_input();
    if let Err(err) = print_factorial(n.0) {
        eprintln!("Error calculating factorial of {}: {}", n.0, err);
    }
}
