//! Classic sorting algorithms
//!
//! This module contains different sorting algorithms

/// This bubble sort algorithm will order any array that implements that `PartialOrd` trait, i. e.,
/// any data that can be compared using `> < >= <=`
/// The core of this algorithm is as for each item in a list of n element it will iterate the list n**2 times
///  and compare two elements. If the first element is greater the algorithm will swap them.
/// # Examples
///
/// ```
/// use ds_algo::algorithms::sorting::bubble_sort::{simple_bubble_sort};
/// let mut v = vec![4, 6, 1, 8, 11, 12];
/// simple_bubble_sort(&mut v);
/// assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
/// ```
pub fn simple_bubble_sort<T: PartialOrd>(list: &mut [T]) {
    for _ in 0..list.len() {
        for i in 0..list.len() - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1)
            }
        }
    }
}

/// This is a version of bubble sort a bit more performant
/// The worst case scenario is still `O(n^2)`
/// A case is added just to check if in the first run the array is already sorted
/// Or is the array is close to sorted the function will return sooner then running all the iterations for all members
/// And since we move the greater element forward this version also decrease the number of iterations on each member that is swaped
/// # Examples
///
/// ```
/// use ds_algo::algorithms::sorting::bubble_sort::{bubble_sort_more_performant};
/// let mut v = vec![4, 6, 1, 8, 11, 12];
/// bubble_sort_more_performant(&mut v);
/// assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
/// ```
// O(n^2)
pub fn bubble_sort_more_performant<T: PartialOrd>(list: &mut [T]) {
    for p in 0..list.len() {
        let mut sorted = true;
        for i in 0..(list.len() - 1) - p {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                sorted = false;
            }
        }

        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::simple_bubble_sort;

    #[test]
    fn test_bubble_sort_with_letters() {
        let mut v = vec!["a", "c", "b"];
        simple_bubble_sort(&mut v);
        assert_eq!(v, vec!["a", "b", "c"]);
    }
}
