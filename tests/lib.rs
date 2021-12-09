#[cfg(test)]
mod tests {
    use merge_sort_tdd::merge;
    use rstest::rstest;

    #[rstest]
    #[case(vec![92], vec![65], vec![65, 92])]
    #[case(vec![2, 2], vec![2, 2], vec![2, 2, 2, 2])]
    #[case(vec![9, 29], vec![12, 65], vec![9, 12, 29, 65])]
    #[case(vec![2, 5], vec![2, 5], vec![2, 2, 5, 5])]
    fn it_merge_lot_arrays(#[case] input1: Vec<i32>, #[case] input2: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(expected, merge(input1, input2));
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
