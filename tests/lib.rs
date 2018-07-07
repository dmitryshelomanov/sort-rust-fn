extern crate base_sort;

#[cfg(test)]
pub mod tests {
    use base_sort::BaseSort;

    #[test]
    pub fn bubble_sort_test() {
        let mut data = vec![3, 2, 1];

        data.bubble_sort();

        assert_eq!(vec![1, 2, 3], data);
    }

    #[test]
    pub fn inerted_sort_test() {
        let mut data = vec![3, 2, 1];

        data.inerted_sort();

        assert_eq!(vec![1, 2, 3], data);
    }

    #[test]
    pub fn shake_sort_test() {
        let mut data = vec![3, 2, 1];

        data.shake_sort();

        assert_eq!(vec![1, 2, 3], data);
    }

    #[test]
    pub fn odd_even_sort_test() {
        let mut data = vec![3, 2, 1];

        data.odd_even_sort();

        assert_eq!(vec![1, 2, 3], data);
    }

    #[test]
    pub fn comb_sort_test() {
        let mut data = vec![3, 2, 1];

        data.bubble_sort();

        assert_eq!(vec![1, 2, 3], data);
    }
}
