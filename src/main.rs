use std::io;

fn main() {
    loop {
        let input1 = read_f64();
        let operator = read_operator();
        let input2 = read_f64();

        match calc(input1, operator, input2) {
            Some(result) => println!("result: {}", result),
            None => println!("Fuck"),
        }

        println!("\n")
    }
}

enum Operator {
    Add, Sub, Mul, Div
}

fn read_f64() -> f64 {
    let num: f64;
    loop {
        println!("Please input number:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        num = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input number of f64");
                continue;
            },
        };
        break;
    }
    num
}


fn read_operator() -> Operator {
    let ope: Operator;
    loop {
        println!("Please input operator:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        ope = match input.trim().to_lowercase().as_str() {
            "+" | "add" => Operator::Add,
            "-" | "sub" => Operator::Sub,
            "*" | "mul" => Operator::Mul,
            "/" | "div" => Operator::Div,
            _ => {
                println!("Input number of f64");
                continue;
            },
        };
        break;
    }
    ope
}

fn calc(left: f64, ope: Operator, right: f64) -> Option<f64> {
    let result: Option<f64> = match (ope, right != 0.0) {
        (Operator::Add, _) => Some(left + right),
        (Operator::Sub, _) => Some(left - right),
        (Operator::Mul, _) => Some(left * right),
        (Operator::Div, true) => Some(left / right),
        _ => {
            println!("Can not div by 0");
            None
        },
    };
    result
}