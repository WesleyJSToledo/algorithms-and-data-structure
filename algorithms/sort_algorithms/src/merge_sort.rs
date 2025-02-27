pub fn merge_sort(mut array: Vec<u32>) -> Vec<u32> {
    let len = array.len() - 1;
    merge(&mut array,0, len);
    return array;
}
fn merge(array: &mut Vec<u32>, begin:usize, end:usize){
    if begin == end {return};
    let mid = (begin + end) / 2;
    merge(array, begin, mid);
    merge(array,mid+1, end);
    intercalation(array, begin, mid, end);
}

fn intercalation(array: &mut Vec<u32>, begin:usize, mid:usize, end:usize){
    let mut aux = array.clone();
    let mut i: usize = begin;
    let mut j: usize = mid +1;
    let mut k: usize = 0;
    while  i <= mid && j <= end{
        if array[i] <= array[j]{
            aux[k] = array[i];
            i+=1;
        } else {
            aux[k] = array[j];
            j+=1;
        }
        k+=1;
    }
    while i <= mid {
        aux[k] = array[i];
        i+=1;
        k+=1;
    }
    while j <= end {
        aux[k] = array[j];
        j+=1;
        k+=1;
    }
    for i in 0..(end-begin+1){
        let j = begin + i;
        array[j] = aux[i];
    }
}