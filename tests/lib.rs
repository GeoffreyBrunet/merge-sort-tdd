#[cfg(test)]
mod tests {
    use merge_sort_tdd::sort;

    #[test]
    fn it_works_test() {
        let v: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(sort(&v), &expected);
    }

}
