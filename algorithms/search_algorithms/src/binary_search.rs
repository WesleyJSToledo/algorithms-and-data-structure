use std::{collections::HashSet, io};

const SIZE: usize = 30;
const MIN: u32 = 0;
const MAX: u32 = 99;

pub fn binary(){
    let mut value = String::new();
    let mut array = generate_unique_random_vector();
    array.sort();
    
    println!("#################################################");
    println!("#\t\tSearch Algorithms\t\t#");
    println!("#################################################");
    println!("Binary Search: ");
    println!("{:?}", array);
    println!("Enter a value: ");
    io::stdin().read_line(&mut value).expect("Error");

    let value: u32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida!");
            return;
        }
    };

    match binary_search(array.clone(), value){
        Some(index) => {
            println!("Value {} is in index {}", value, index);
        }
        None => {
            println!("Element not found");
        }
    }; 

}

fn binary_search(array: Vec<u32>, key:u32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high = array.len();

    while low <= high {
        let mid = (low + high)/ 2;
        if array[mid] == key {return Some(mid)};
        if array[mid] > key {high = mid - 1} else {low = mid + 1}; 
    }
    return None;
}

fn generate_unique_random_vector() -> Vec<u32> {
    let mut unique_values = HashSet::new();

    while unique_values.len() < SIZE {
        let rand_value = rand::random_range(MIN..=MAX);
        unique_values.insert(rand_value);
    }
    return unique_values.into_iter().collect();
}