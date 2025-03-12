use std::collections::{HashMap, HashSet};
use regex::Regex;
use std::error::Error;

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize, // Each row represents an element
    cols: usize  // Each column represents a compound
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>, rows: usize, cols: usize) -> Self {
        assert!(data.len() == rows && data.iter().all(|c| c.len() == cols), "The dimensions of the data do not match the number of rows and columns provided.");
        Self { data, rows, cols }
    }
}

// Get the function from the user and return without whitespace
fn get_function_from_user() -> String {
    let mut equation = String::new();
    std::io::stdin().read_line(&mut equation).unwrap();
    String::from(equation.trim())
}

// Ensures that the user equation has the same elements present on both sides of the equation, and returns a list of all unique elements if valid.
fn verify_user_equation(equation_vec: Vec<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
    let regex = Regex::new(r"[A-Za-z]+").unwrap();
    let reactants: HashSet<&str> = regex.find_iter(equation_vec[0]).map(|c| c.as_str()).collect();
    let products: HashSet<&str> = regex.find_iter(equation_vec[1]).map(|c| c.as_str()).collect();
    match reactants.iter().all(|c| products.contains(c)) {
        true => {
            Ok(reactants.into_iter().collect())
        }
        false =>{
            Err("The elements in the reactants and products do not match".into())
        }
    }
}

fn get_element_counts(compound: &str) -> HashMap<String, i32> {
    let mut counts = HashMap::new();
    let regex = Regex::new(r"([A-Z][a-z])(\d*)").unwrap();
    for cap in regex.captures_iter(compound) {
        println!("{:?}", cap);
    }
    counts
}

fn main() {
    println!("Please enter the chemical equation you want to balance.");
    let equation = get_function_from_user();
    let equation_vec: Vec<&str> = equation.split(" = ").collect();
    let elements = verify_user_equation(equation_vec.clone()).unwrap();
    let reactants: Vec<&str> = equation_vec[0].split(" + ").collect();
    let products: Vec<&str> = equation_vec[1].split(" + ").collect();
    println!("Here are your elements: {:?}", elements);
    for reactant in reactants {
        get_element_counts(reactant);
    }
    for product in products {
        get_element_counts(product);
    }
}