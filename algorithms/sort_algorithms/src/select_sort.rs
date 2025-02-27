pub fn select_sort(mut array: Vec<u32>) -> Vec<u32> {
    let mut sort_array= Vec::<u32>::with_capacity(array.len());
    while array.len() > 0 {
        let i = select_min(array.clone());
        sort_array.push(array[i]);
        array.remove(i);
    }
    return sort_array;
}

pub fn select_min(array: Vec<u32>) -> usize{
    let mut small_index: usize = 0;

    for i in  0..array.len(){
        if array[i] < array[small_index] {
            small_index = i;
        }
    } 

    return small_index;
}