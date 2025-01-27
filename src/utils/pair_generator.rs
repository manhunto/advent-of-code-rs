use itertools::Itertools;

pub fn pairs<T>(list: Vec<T>) -> Vec<(T, T)>
where
    T: Copy,
{
    list.into_iter().tuple_combinations().collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::pair_generator::pairs;

    #[test]
    fn generate_pairs_test() {
        let vec: Vec<i32> = vec![1, 2, 3, 4];

        let expected: Vec<(i32, i32)> = vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)];

        assert_eq!(expected, pairs(vec));
    }

    #[test]
    fn generate_pairs_count_test() {
        let vec: Vec<i32> = (1..=9).collect();

        assert_eq!(36, pairs(vec).len());
    }
}
