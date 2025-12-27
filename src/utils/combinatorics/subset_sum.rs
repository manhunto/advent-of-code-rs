use std::ops::Sub;

pub fn find_subset_sum<T>(candidates: &[T], target: T) -> Vec<Vec<T>>
where
    T: Copy + Default + Ord + Sub<Output = T>,
{
    if target == T::default() {
        return Vec::new();
    }

    let mut sorted = candidates.to_vec();
    sorted.sort_unstable();

    let mut result = Vec::new();
    backtrack(&sorted, target, 0, &mut Vec::new(), &mut result);
    result
}

fn backtrack<T>(
    candidates: &[T],
    remaining: T,
    start: usize,
    current_path: &mut Vec<T>,
    result: &mut Vec<Vec<T>>,
) where
    T: Copy + Default + Ord + Sub<Output = T>,
{
    if remaining == T::default() {
        result.push(current_path.clone());
        return;
    }

    for i in start..candidates.len() {
        let val = candidates[i];
        if val > remaining {
            break;
        }

        current_path.push(val);
        backtrack(candidates, remaining - val, i + 1, current_path, result);
        current_path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_element_separate() {
        let result = find_subset_sum(&[10, 20], 10);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![10]);
    }

    #[test]
    fn one_element_last_separate() {
        let result = find_subset_sum(&[10, 20, 40], 40);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![40]);
    }

    #[test]
    fn two_elements_separate() {
        let result = find_subset_sum(&[10, 10], 10);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![10]);
        assert_eq!(result[1], vec![10]);
    }

    #[test]
    fn more_complex_elements() {
        let result = find_subset_sum(&[20, 15, 10, 5, 5], 25);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], vec![5, 5, 15]);
        assert_eq!(result[1], vec![5, 20]);
        assert_eq!(result[2], vec![5, 20]);
        assert_eq!(result[3], vec![10, 15]);
    }

    #[test]
    fn empty_candidates() {
        let result: Vec<Vec<i32>> = find_subset_sum(&[], 10);

        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn target_zero() {
        let result = find_subset_sum(&[1, 2, 3], 0);

        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn no_solution() {
        let result = find_subset_sum(&[2, 4, 6], 5);

        assert_eq!(result, Vec::<Vec<i32>>::new());
    }
}
