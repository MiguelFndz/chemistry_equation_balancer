use std::{collections::{HashMap, HashSet}, hash::Hasher, io::Read};
use regex::Regex;
use std::hash::Hash;

struct Element {
    name: String,
    subscript: u32,
}

impl Element {
    fn new(name: String, subscript: u32) -> Self {
        Self {name, subscript}
    }
}

impl Eq for Element {}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.subscript == other.subscript
    }
}

impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.subscript.hash(state);
    }
}
struct Compound {
    name: String,
    coefficient: u32,
    elements: HashMap<Element, u32>,
}

impl Compound {
    fn new(name: String) -> Self {
        let mut elements: HashMap<Element, u32> = HashMap::new();
        let mut coefficient: u32 = 1;
    
        let regex = Regex::new(r"([A-Z][a-z]?)(\d*)").unwrap();
        let c_regex = Regex::new(r"^(\d*)").unwrap();

        if let Some(cap) = c_regex.captures(&name) {
            coefficient = cap[1].parse::<u32>().unwrap_or(1);
        }

        for cap in regex.captures_iter(&name) {
            let element_name = cap[1].to_string();
            let subscript = cap[2].parse::<u32>().unwrap_or(1);
            let element = Element::new(element_name, subscript);
            *elements.entry(element).or_insert(0) += subscript
        };

        Self { name, coefficient, elements }

    }
}
struct EquationHalf {
    name: String,
    compounds: Vec<Compound>,
    elements: HashMap<String, u32>,
}

impl EquationHalf {
    fn new(half: String) -> Self {
        let compounds: Vec<Compound> = half.split(" + ").map(|s| Compound::new(s.trim().to_string())).collect();
        let mut elements: HashMap<String, u32> = HashMap::new();
        for compound in compounds.iter() {
            for (element, count) in &compound.elements {
                let entry = elements.entry(element.name.clone()).or_insert(0);
                *entry += count * compound.coefficient;
            }
        }
        Self { name: half, compounds, elements }
    }

    fn get_element_names(&self) -> HashSet<String> {
        let mut element_names: HashSet<String> = HashSet::new();
        for compound in self.compounds.iter() {
            for element in compound.elements.keys() {
                element_names.insert(element.name.clone());
            }
        }
        element_names
    }
}
struct Equation {
    name: String,
    reactants: EquationHalf,
    products: EquationHalf,
}

impl Equation {
    fn new(user_equation: String) -> Self {
        let equation_parts: Vec<String> = user_equation.split(" = ").map(|s| s.to_string()).collect();
        let reactants = EquationHalf::new(equation_parts[0].clone());
        let products = EquationHalf::new(equation_parts[1].clone());
        Self { name: user_equation, reactants, products }
    }
}

fn get_equation() -> String {
    let mut equation = String::new();
    std::io::stdin().read_line(&mut equation).unwrap();
    String::from(equation.trim())
}

fn verify_elements(products: EquationHalf, reactants: EquationHalf) -> bool {
    true
}

fn main() {
    // Get equation from user
    println!("Welcome to Chemical Equation Balncer! Please type in the chemical equation you wish to balance.");
    let user_equation = get_equation();
    println!("You have entered this equation: {:?}", user_equation);

    // create equation
    let equation = Equation::new(user_equation);

    // print reactants and products
    println!("reactants:");
    for reactant in equation.reactants.compounds {
        println!("{:?}", reactant.name);
    }

    println!("products:");
    for product in equation.products.compounds {
        println!("{:?}", product.name);
    }

}