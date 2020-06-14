//! # Quick Sort
//! Quick sort is also a recursive algorithm like Merge Sort and it works with the idea of a pivot
//! You pick a random number in the list and then you move all the numbers that are lesser than the pivot to the left
//! And you move all the numbers that are greater than the pivot to the right
//! Then you split the list around the pivot and then you execute quick sort in the next halves
//! Basically you do the same process until all the pivots are sorted which means that the whole list is sorted
//! [Wanna have fun?](https://www.youtube.com/watch?v=ywWBy6J5gz8)

use crate::utils::rand::generate_random_number;
use std::fmt::Debug;

// Move first element to the correct place
// Everything lower should be before it
// Everything higher should be after it
// Return pivot's position to the list can be split around the pivot
fn pivot<T: PartialOrd>(list: &mut [T]) -> usize {
    let mut pivot = generate_random_number(list.len());
    list.swap(pivot, 0);
    pivot = 0;

    for i in 1..list.len() {
        if list[i] < list[pivot] {
            // Move our pivot forward 1 and put this element before it
            list.swap(pivot + 1, i);
            list.swap(pivot, pivot + 1);
            pivot += 1;
        }
    }

    pivot
}

/// # Examples
///
/// ```
/// use ds_algo::algorithms::sorting::quick_sort::{quick_sort};
/// let mut v = vec![4, 6, 1, 8, 11, 12];
/// quick_sort(&mut v);
/// assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
/// ```
pub fn quick_sort<T: PartialOrd + Debug>(list: &mut [T]) {
    if list.len() <= 1 {
        return;
    }

    let pivot = pivot(list);

    let (a, b) = list.split_at_mut(pivot);
    quick_sort(a);
    //Middle element already sorted
    quick_sort(&mut b[1..]);
}

#[cfg(test)]
mod tests {
    use super::{pivot, quick_sort};

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert_eq!(v[x] < v[p], x < p);
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 12];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
    }
}
