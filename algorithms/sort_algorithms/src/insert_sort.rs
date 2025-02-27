pub fn insert_sort(mut array: Vec<u32>) -> Vec<u32> {
    for i in 0..array.len(){
        let temp = array[i];
        let mut j = i;
        while (j>0) && (array[j-1]> temp){
            array[j] = array[j-1];
            j = j-1;
        }
        array[j] = temp;
    }
    return array;
}