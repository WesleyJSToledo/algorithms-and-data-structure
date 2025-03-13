mod bubble_sort;
mod select_sort;
mod insert_sort;
mod merge_sort;
mod quick_sort;

use bubble_sort::bubble_sort; 
use select_sort::select_sort;
use insert_sort::insert_sort;
use merge_sort::merge_sort;
use quick_sort::quick_sort;

use std::{self, collections::HashSet, time::Instant};

const SIZE: usize = 99_999;
const MIN: u32 = 0;
const MAX: u32 = 9_999_999;

fn main() {
    let array:Vec<u32> = generate_unique_random_vector();

    println!("#########################################\n#\t\tStart Sort\t\t#\n#########################################");

    let mut start =  Instant::now();
    bubble_sort(array.clone());
    let mut time: std::time::Duration = start.elapsed();
    println!("Bubble Sort: \t{:.5} segundos", time.as_secs_f64());

    start =  Instant::now();
    select_sort(array.clone());
    time = start.elapsed();
    println!("Select Sort: \t{:.5} segundos", time.as_secs_f64());
    
    start =  Instant::now();
    insert_sort(array.clone());
    time = start.elapsed();
    println!("Insert Sort: \t{:.5} segundos", time.as_secs_f64());
    
    start =  Instant::now();
    merge_sort(array.clone());
    time = start.elapsed();
    println!("Merge Sort: \t{:.5} segundos", time.as_secs_f64());
    
    start =  Instant::now();
    quick_sort(array.clone());
    time = start.elapsed();
    println!("Quick Sort: \t{:.5} segundos", time.as_secs_f64());
}


fn generate_unique_random_vector() -> Vec<u32> {
    let mut unique_values = HashSet::new();

    while unique_values.len() < SIZE {
        let rand_value = rand::random_range(MIN..=MAX);
        unique_values.insert(rand_value);
    }
    unique_values.into_iter().collect()
}