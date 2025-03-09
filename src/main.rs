use std::io;
use std::collections::HashSet;
use regex::Regex;

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
    let regex = Regex::new(r"[A-Za-z]+").unwrap();
    let reactants: Vec<&str> = regex.find_iter(tokens[0]).map(|element| element.as_str()).collect();
    let products: Vec<&str> = regex.find_iter(tokens[1]).map(|element| element.as_str()).collect();
    println!("Here are the reacting elements: {:?}", reactants);
    println!("Here are the product elements: {:?}", products);
    println!("{:?}", verify_elements_match(reactants.into_iter().collect(), products.into_iter().collect()));
    tokens
}

fn verify_elements_match(reactants: HashSet<&str>, products: HashSet<&str>) -> bool{
    reactants.iter().all(|element| products.contains(element))
}
