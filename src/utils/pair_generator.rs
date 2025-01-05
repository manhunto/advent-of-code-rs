use itertools::Itertools;
use std::hash::Hash;

pub fn pairs<T>(list: Vec<T>) -> Vec<(T, T)>
where
    T: Clone,
{
    list.into_iter().tuple_combinations().collect()
}

pub fn unique_pairs<T>(list: Vec<T>, how_many_pairs: usize) -> Vec<Vec<(T, T)>>
where
    T: Clone + Ord + Hash,
{
    list.into_iter()
        .permutations(how_many_pairs * 2)
        .map(|perm| {
            let mut pairs: Vec<(T, T)> = perm
                .chunks(2)
                .map(|chunk| {
                    let a = chunk[0].clone().min(chunk[1].clone());
                    let b = chunk[0].clone().max(chunk[1].clone());

                    (a, b)
                })
                .collect();
            pairs.sort_unstable();
            pairs
        })
        .unique()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::pair_generator::{pairs, unique_pairs};

    #[test]
    fn pairs_test() {
        let vec: Vec<i32> = vec![1, 2, 3, 4];

        let expected: Vec<(i32, i32)> = vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)];

        assert_eq!(expected, pairs(vec));
    }

    #[test]
    fn pairs_count_test() {
        let vec: Vec<i32> = (1..=9).collect();

        assert_eq!(36, pairs(vec).len());
    }

    #[test]
    fn unique_pairs_test() {
        let vec: Vec<i32> = vec![1, 2, 3, 4];

        let expected: Vec<Vec<(i32, i32)>> = vec![
            vec![(1, 2), (3, 4)],
            vec![(1, 3), (2, 4)],
            vec![(1, 4), (2, 3)],
        ];

        assert_eq!(expected, unique_pairs(vec, 2));
    }

    #[test]
    fn unique_pairs_empty_test() {
        let vec: Vec<i32> = vec![];

        let expected: Vec<Vec<(i32, i32)>> = vec![];

        assert_eq!(expected, unique_pairs(vec, 2));
    }

    #[test]
    fn unique_pairs_single_element_test() {
        let vec: Vec<i32> = vec![1];

        let expected: Vec<Vec<(i32, i32)>> = vec![];

        assert_eq!(expected, unique_pairs(vec, 2));
    }

    #[test]
    fn unique_pairs_large_test() {
        let vec: Vec<i32> = (1..=6).collect();

        let expected: Vec<Vec<(i32, i32)>> = vec![
            vec![(1, 2), (3, 4), (5, 6)],
            vec![(1, 2), (3, 5), (4, 6)],
            vec![(1, 2), (3, 6), (4, 5)],
            vec![(1, 3), (2, 4), (5, 6)],
            vec![(1, 3), (2, 5), (4, 6)],
            vec![(1, 3), (2, 6), (4, 5)],
            vec![(1, 4), (2, 3), (5, 6)],
            vec![(1, 4), (2, 5), (3, 6)],
            vec![(1, 4), (2, 6), (3, 5)],
            vec![(1, 5), (2, 3), (4, 6)],
            vec![(1, 5), (2, 4), (3, 6)],
            vec![(1, 5), (2, 6), (3, 4)],
            vec![(1, 6), (2, 3), (4, 5)],
            vec![(1, 6), (2, 4), (3, 5)],
            vec![(1, 6), (2, 5), (3, 4)],
        ];

        assert_eq!(expected, unique_pairs(vec, 3));
    }
}
