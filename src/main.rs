use std::io;
use std::collections::HashSet;

fn main() {
    println!("Enter the chemical equation you want to balance.");
    let user_equation: String = get_equation();
    println!("{:?}", &user_equation);
    let tokens = validate_equation(&user_equation);
    println!("{:?}", tokens);
}

fn get_equation() -> String{
    let mut user_equation: String = String::new();
    std::io::stdin().read_line(&mut user_equation).unwrap();
    String::from(user_equation.trim())
}

fn validate_equation(equation: &str) -> Vec<&str>{
    let tokens: Vec<&str> = equation.split("=").collect();
    tokens
}

fn verify_elements(reactants: HashSet<&str>, products: HashSet<&str>) -> bool{
    reactants.iter().all(|element| products.contains(element))
}
