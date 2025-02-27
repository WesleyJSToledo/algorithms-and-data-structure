
pub fn quick_sort(mut array: Vec<u32>) -> Vec<u32> {
    let len = array.len() - 1;
    quick(&mut array, 0, len);
    return array;
}

fn quick(array: &mut Vec<u32>, left: usize, right:usize){
    let mut i: usize = left;
    let mut j: usize = right;
    let pivot_index: usize = (left+right)/2;
    let pivot: u32 = array[pivot_index];

    loop{
        while array[i] < pivot && i < right{i+=1};
        while array[j] > pivot && j > left {j-=1};

        if i > j {break};

        array.swap(i, j);
        i+=1;
        j-=1;
    } 

    if j > left {quick(array, left, j)};
    if i < right{quick(array, i, right)};
}