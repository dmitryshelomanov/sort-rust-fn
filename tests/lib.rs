extern crate base_sort;

use base_sort::BaseSort;

#[cfg(test)]
pub mod tests {
    use super::*;

    thread_local!(static EXAMPLE_RESULT: Vec<i32> = vec![1, 2, 3]);
    thread_local!(static EXAMPLE1_RESULT: Vec<i32> = vec![1, 2, 4, 5, 6, 10]);

    #[test]
    pub fn bubble_sort_test() {
        let mut example = vec![3, 2, 1];
        let mut example1 = vec![10, 5, 4, 1, 2, 6];

        example.bubble_sort();
        example1.bubble_sort();

        EXAMPLE_RESULT.with(|e| {
            assert_eq!(*e, example);
        });

        EXAMPLE1_RESULT.with(|e| {
            assert_eq!(*e, example1);
        });
    }

    #[test]
    pub fn insertion_sort_test() {
        let mut example = vec![3, 2, 1];
        let mut example1 = vec![10, 5, 4, 1, 2, 6];

        example.insertion_sort();
        example1.insertion_sort();

        EXAMPLE_RESULT.with(|e| {
            assert_eq!(*e, example);
        });

        EXAMPLE1_RESULT.with(|e| {
            assert_eq!(*e, example1);
        });
    }

    #[test]
    pub fn shell_sort_test() {
        let mut example = vec![3, 2, 1];
        let mut example1 = vec![10, 5, 4, 1, 2, 6];

        example.shell_sort();
        example1.shell_sort();

        EXAMPLE_RESULT.with(|e| {
            assert_eq!(*e, example);
        });

        EXAMPLE1_RESULT.with(|e| {
            assert_eq!(*e, example1);
        });
    }

    #[test]
    pub fn shake_sort_test() {
        let mut example = vec![3, 2, 1];
        let mut example1 = vec![10, 5, 4, 1, 2, 6];

        example.shake_sort();
        example1.shake_sort();

        EXAMPLE_RESULT.with(|e| {
            assert_eq!(*e, example);
        });

        EXAMPLE1_RESULT.with(|e| {
            assert_eq!(*e, example1);
        });
    }

    #[test]
    pub fn odd_even_sort_test() {
        let mut example = vec![3, 2, 1];
        let mut example1 = vec![10, 5, 4, 1, 2, 6];

        example.odd_even_sort();
        example1.odd_even_sort();

        EXAMPLE_RESULT.with(|e| {
            assert_eq!(*e, example);
        });

        EXAMPLE1_RESULT.with(|e| {
            assert_eq!(*e, example1);
        });
    }

    #[test]
    pub fn comb_sort_test() {
        let mut example = vec![3, 2, 1];
        let mut example1 = vec![10, 5, 4, 1, 2, 6];

        example.comb_sort();
        example1.comb_sort();

        EXAMPLE_RESULT.with(|e| {
            assert_eq!(*e, example);
        });

        EXAMPLE1_RESULT.with(|e| {
            assert_eq!(*e, example1);
        });
    }
}
