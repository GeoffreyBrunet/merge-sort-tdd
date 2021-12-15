pub fn merge(mut slice1: Vec<i32>, mut slice2: Vec<i32>) -> Vec<i32> {
    let height: i32 = (slice1.len() + slice2.len()) as i32;
    let mut result: Vec<i32> = vec![];
    for _ in 0..height {
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
    }
    result
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