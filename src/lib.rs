pub fn merge(mut slice1: Vec<i32>, mut slice2: Vec<i32>) -> Vec<i32> {
    let height: i32 = (slice1.len() + slice2.len()) as i32;
    let mut count: i32 = 0;
    let mut result: Vec<i32> = vec![];
    while count < height {
        if slice1.len() != 0 && slice2.len() != 0 {
            if slice1[0] < slice2[0] {
                result.push(slice1[0]);
                slice1.remove(0);
            } else if slice1[0] > slice2[0] {
                result.push(slice2[0]);
                slice2.remove(0);
            } else if slice1[0] == slice2[0] {
                result.push(slice1[0]);
                result.push(slice2[0]);
                slice1.remove(0);
                slice2.remove(0);
            }
        } else if slice1.len() != 0 && slice2.len() == 0 {
            result.push(slice1[0]);
            slice1.remove(0);
        } else if slice1.len() == 0 && slice2.len() != 0 {
            result.push(slice2[0]);
            slice2.remove(0);
        }
        count += 1;
    }
    result
}

pub fn divide(array: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mid = array.len()/2;
    let last = array.len();
    let mut slice1: Vec<i32> = vec![];
    let mut slice2: Vec<i32> = vec![];
    slice1.append(&mut array[0..mid].to_vec());
    slice2.append(&mut array[mid..last].to_vec());
    (slice1, slice2)
}

pub fn sort(mut array: Vec<i32>) -> Vec<i32>  {
    if array.len() > 1 {
        if array[0] > array[1] {
            let tmp = array[0];
            array[0] = array[1];
            array[1] = tmp;
        }
    }
    array
}

pub fn merge_sort(array: Vec<i32>) -> Vec<i32> {
    if array.len() < 2 {
        array
    } else {
        let size: usize = array.len() / 2;
        let slice1: Vec<i32> = merge_sort(array[0..size].to_vec());
        let slice2: Vec<i32> = merge_sort(array[size..].to_vec());
        let result: Vec<i32> = merge(slice1, slice2);
        result
    }
}