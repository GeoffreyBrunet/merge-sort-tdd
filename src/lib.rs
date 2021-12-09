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

pub fn sort(mut array: Vec<i32>) -> Vec<i32> {
    if array.len() >= 2 {
        let (slice1, slice2): (Vec<i32>, Vec<i32>) = array
            .clone()
            .into_iter()
            .partition(|x| x % 2 ==0);
        array[0] = slice1[0];
        array[1] = slice2[0];
        println!("Slice 1: {:?}, Slice 2: {:?}", slice1, slice2);
    }
    array
}
