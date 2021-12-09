#[cfg(test)]
mod tests {
    use merge_sort_tdd::merge;

    #[test]
    fn it_merge_two_vec_with_one_member() {
        let slice1: Vec<i32> = vec![6];
        let slice2: Vec<i32> = vec![2];
        let expected: Vec<i32> = vec![2, 6];
        assert_eq!(expected, merge(slice1, slice2));
    }

    #[test]
    fn it_merge_two_vec_with_one_same_member() {
        let slice1: Vec<i32> = vec![2];
        let slice2: Vec<i32> = vec![2];
        let expected: Vec<i32> = vec![2, 2];
        assert_eq!(expected, merge(slice1, slice2));
    }

    #[test]
    fn it_merge_two_vec_with_two_members() {
        let slice1: Vec<i32> = vec![5, 8];
        let slice2: Vec<i32> = vec![2, 6];
        let expected: Vec<i32> = vec![2, 5, 6, 8];
        assert_eq!(expected, merge(slice1, slice2));
    }

    #[test]
    fn it_merge_two_vec_with_two_same_numbers() {
        let slice1: Vec<i32> = vec![2, 5];
        let slice2: Vec<i32> = vec![2, 5];
        let expected: Vec<i32> = vec![2, 2, 5, 5];
        assert_eq!(expected, merge(slice1, slice2));
    }

    /*#[test]
    fn it_sort_empty() {
        let v: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, sort(v));
    }

    #[test]
    fn it_sort_one_item() {
        let v: Vec<i32> = vec![1];
        let expected: Vec<i32> = vec![1];
        assert_eq!(expected, sort(v));
    }

    #[test]
    fn it_sort_two_identical_items() {
        let v: Vec<i32> = vec![2, 5];
        let expected: Vec<i32> = vec![2, 5];
        assert_eq!(expected, sort(v));
    }

    #[test]
    fn it_sort_two_items() {
        let v: Vec<i32> = vec![5, 2];
        let expected: Vec<i32> = vec![2, 5];
        assert_eq!(expected, sort(v));
    }

    #[test]
    fn it_sort_three_items() {
        let v: Vec<i32> = vec![1, 5, 2];
        let expected: Vec<i32> = vec![1, 2, 5];
        assert_eq!(expected, merge(v));
    }

    #[test]
    fn it_sort_three_others_items() {
        let v: Vec<i32> = vec![5, 1, 2];
        let expected: Vec<i32> = vec![1, 2, 5];
        assert_eq!(expected, merge(v));
    }*/
}
