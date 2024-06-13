use clap::{App, Arg};

fn main() {
    let matches = App::new("cli-calc")
        .version("1.0")
        .author("LOHITH")
        .about("Performs basic arithmetic operations")
        .arg(
            Arg::new("operation")
                .short('o')
                .long("operation")
                .value_name("OPERATION")
                .possible_values(&["add", "sub", "mul", "div", "+", "-", "*", "/"])
                .takes_value(true)
                .help("Specifies the operation to perform: add, sub, mul, div"),
        )
        .arg(
            Arg::new("operand1")
                .short('1')
                .long("operand1")
                .value_name("OPERAND1")
                .takes_value(true)
                .help("First operand"),
        )
        .arg(
            Arg::new("operand2")
                .short('2')
                .long("operand2")
                .value_name("OPERAND2")
                .takes_value(true)
                .help("Second operand"),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .takes_value(false)
                .help("Prints version information"),
        )
        .arg(
            Arg::new("about")
                .long("about")
                .takes_value(false)
                .help("Prints application details"),
        )
        .get_matches();

   
    if matches.is_present("version") {
        println!("cli-calc version 1.0");
        return;
    }

 
    if matches.is_present("about") {
        println!("cli-calc - Performs basic arithmetic operations");
        println!("Author: LOHITH");
        return;
    }


    let operation = matches.value_of("operation").unwrap_or_else(|| {
        eprintln!("Error: Missing operation");
        std::process::exit(1);
    });

    let operand1 = matches.value_of("operand1").unwrap_or_else(|| {
        eprintln!("Error: Missing operand1");
        std::process::exit(1);
    });

    let operand2 = matches.value_of("operand2").unwrap_or_else(|| {
        eprintln!("Error: Missing operand2");
        std::process::exit(1);
    });

    // Convert operand1 and operand2 to f64
    let operand1 = operand1.parse::<f64>().unwrap_or_else(|_| {
        eprintln!("Error: Invalid operand1");
        std::process::exit(1);
    });

    let operand2 = operand2.parse::<f64>().unwrap_or_else(|_| {
        eprintln!("Error: Invalid operand2");
        std::process::exit(1);
    });

   
    match operation {
        "add" | "+" => println!("Result: {}", operand1 + operand2),
        "sub" | "-" => println!("Result: {}", operand1 - operand2),
        "mul" | "*" => println!("Result: {}", operand1 * operand2),
        "div" | "/" => {
            if operand2 == 0.0 {
                eprintln!("Error: Division by zero");
                std::process::exit(1);
            } else {
                println!("Result: {}", operand1 / operand2);
            }
        }
        _ => {
            eprintln!("Error: Unsupported operation");
            std::process::exit(1);
        }
    }
}


#[cfg(test)]
mod tests {
    

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 4.0), 6.0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(3.0, 2.0), 1.0);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(4.0, 4.0), 16.0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(8.0, 2.0), 4.0);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_by_zero() {
        div(8.0, 0.0);
    }

    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    fn sub(a: f64, b: f64) -> f64 {
        a - b
    }

    fn mul(a: f64, b: f64) -> f64 {
        a * b
    }

    fn div(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            panic!("Division by zero");
        }
        a / b
    }
}