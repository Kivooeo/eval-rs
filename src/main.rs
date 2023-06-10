use std::io::{stdin, stdout, Write};

use eval::f;

fn main() {
    loop {
        print!(">>> ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input);
        input.replace("\n", "");
        if input.contains("|") {
            let split = input.split("|").into_iter().collect::<Vec<&str>>();
            let vars = split[1]
                .trim()
                .split_ascii_whitespace()
                .into_iter()
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();
            let expression = split[0].trim();
            let answer = eval::math::eval(&eval::math::fill(expression, &vars));
            println!("answer is {answer}");
        } else {
            println!("answer is {}", f!(&input));
        }
        input.clear();
    }
}
