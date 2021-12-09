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
    let last = *(array.last().unwrap()) as usize;
    let slice1: Vec<i32> = array[0..mid].to_vec();
    let slice2: Vec<i32> = array[mid..last].to_vec();
    (slice1, slice2)
}

pub fn sort() -> Vec<i32>  {
    vec![]
}
