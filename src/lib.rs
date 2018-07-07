pub trait BaseSort {
    fn bubble_sort(&mut self);
    fn inerted_sort(&mut self);
    fn shake_sort(&mut self);
    fn odd_even_sort(&mut self);
    fn comb_sort(&mut self);
}

impl<T: Ord> BaseSort for Vec<T> {
    fn bubble_sort(&mut self) {
        let len = self.len();

        for _ in 0..len {
            for j in 1..len {
                if self[j - 1] > self[j] {
                    self.swap(j - 1, j);
                }
            }
        }
    }

    fn inerted_sort(&mut self) {
        let len = self.len() + 1;

        for i in 0..len {
            for j in (1..i).rev() {
                if self[j] < self[j - 1] {
                    self.swap(j - 1, j);
                }
            }
        }
    }

    fn shake_sort(&mut self) {
        let mut right_index = self.len();
        let mut left_index = 0;

        while left_index < right_index {
            for i in left_index..right_index - 1{
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                }
            }

            right_index = right_index - 1;

            for i in (left_index..right_index).rev() {
                if i > 0 && self[i] < self[i - 1] {
                    self.swap(i, i - 1);
                }
            }

            left_index = left_index + 1;
        }
    }

    fn odd_even_sort(&mut self) {
        let mut flag = false;
        let is_odd = |x: usize| x % 2 != 0;
        let is_even = |x: usize| x % 2 == 0;
        let len = self.len();

        for _ in 0..len {
            {
                let condition = |index: usize| if flag { is_even(index) } else { is_odd(index) };

                for j in 1..len{
                    if condition(j - 1) && self[j - 1] > self[j] {
                        self.swap(j - 1, j);
                    }
                }
            }

            flag = !flag;
        }
    }

    fn comb_sort(&mut self) {
        let reduction_factor: f64 = 1.24;
        let len = self.len() - 1;
        let mut toggle_len = len.clone();
        let mut count = 1;

        for _ in 0..len {
            let step = (toggle_len as f64 / reduction_factor).ceil() as usize;

            for j in 0..count {
                if self[j + step] < self[j] {
                    self.swap(j + step, j);
                }
            }

            toggle_len = toggle_len - 1;
            count = count + 1;
        }
    }
}
