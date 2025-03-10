use std::{collections::HashSet};
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

// Create matrix rows
fn create_matrix_data(elements: Vec<&str>, reactants: Vec<&str>, products: Vec<&str>) -> Vec<Vec<f64>> {
    let mut data: Vec<Vec<f64>> = Vec::new();
    for element in elements {
        let mut element_row: Vec<f64> = Vec::new();
        println!("Processing {:?}", element);
        for reactant in &reactants {
            if reactant.contains(element) {
                println!("{:?} is in {:?}", element, reactant);
                let mut index = reactant.find(element).unwrap();
                println!("{:?} appears at position {:?} inside {:?}", element, index, reactant);
            }
            else {
                println!("{:?}, is not in {:?}", element, reactant);
            }
        }
        for product in &products {
            if product.contains(element) {
                println!("{:?} is in {:?}", element, product);
            }
            else {
                println!("{:?} is not in {:?}", element, product);
            }
        }
    }
    data
}
fn main() {
    println!("Please enter the chemical equation you want to balance.");
    let equation = get_function_from_user();
    let equation_vec: Vec<&str> = equation.split(" = ").collect();
    let elements = verify_user_equation(equation_vec.clone()).unwrap();
    let reactants = equation_vec[0].split(" + ").collect();
    let products = equation_vec[1].split(" + ").collect();
    println!("Here are your elements: {:?}", elements);
    create_matrix_data(elements, reactants, products);
}