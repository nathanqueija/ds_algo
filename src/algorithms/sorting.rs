//! Classic sorting algorithms
//!
//! This module contains different sortin algorithms

/// This bubble sort algorithm will order any array that implements that `PartialOrd` trait, i. e.,
/// any data that can be compared using `> < >= <=`
/// The core of this algorithm is as for each item in a list of n element it will iterate the list n-1 times
///  and compare two elements. If the first element is greater the algorithm will swap them.
/// # Example
///
/// ```
/// use data_structures_and_algorithms::algorithms::sorting::bubble_sort;
/// let mut v = vec![4, 6, 1, 8, 11, 12];
/// bubble_sort(&mut v);
/// assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
/// ```
pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    for _ in 0..list.len() {
        for i in 0..list.len() - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1)
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//
//     use super::bubble_sort;
//
//     #[test]
//     fn test_bubble_sort() {
//         let mut v = vec![4, 6, 1, 8, 11, 12];
//         bubble_sort(&mut v);
//         assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
//     }
// }
