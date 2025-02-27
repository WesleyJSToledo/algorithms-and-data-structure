pub fn bubble_sort(mut array: Vec<u32>) -> Vec<u32> {
    for _ in  1 ..array.len() {
        for j in 0 ..array.len() - 1{
            if array[j] > array[j+1]{
                array.swap(j, j+1);
            }
        }
    }
    return array;
}