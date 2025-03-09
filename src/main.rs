use core::num;
use std::io;
use std::collections::HashSet;
use regex::Regex;

struct Matrix {
    data: Vec<Vec<f64>>,
    num_elements: usize, //The number of rows
    num_coefficients: usize // The number of columns
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>, num_elements: usize, num_coefficients: usize) -> Self {
        assert!(data.len() == num_elements && data.iter().all(|c| c.len() == num_coefficients), "The number of elements and coefficients must match the dimensions of data.");
        Self { data, num_elements, num_coefficients }
    }
}


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

fn create_matrix() {

}
