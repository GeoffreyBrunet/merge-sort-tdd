#[cfg(test)]
mod tests {
    use merge_sort_tdd::{merge, merge_sort};
    use rstest::rstest;

    #[rstest]
    #[case(vec![92], vec![65], vec![65, 92])]
    #[case(vec![2, 2], vec![2, 2], vec![2, 2, 2, 2])]
    #[case(vec![12, 65], vec![9, 29], vec![9, 12, 29, 65])]
    #[case(vec![2, 5], vec![2, 5], vec![2, 2, 5, 5])]
    fn it_merge_lot_arrays(#[case] input1: Vec<i32>, #[case] input2: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(expected, merge(input1, input2));
    }

    #[rstest]
    #[case(vec![], vec![])]
    #[case(vec![4], vec![4])]
    #[case(vec![8, 7], vec![7, 8])]
    #[case(vec![8, 7, 4], vec![4, 7, 8])]
    #[case(vec![5, 8, 7, 4], vec![4, 5, 7, 8])]
    #[case(vec![5, 8, 7, 4, 6], vec![4, 5, 6, 7, 8])]
    #[case(vec![5, 8, 7, 4, 6, 2, 3], vec![2, 3, 4, 5, 6, 7, 8])]
    fn it_merge_sort_arrays(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(expected, merge_sort(input));
    }
}
