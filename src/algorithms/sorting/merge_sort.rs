//! # Merge Sort
//! This module have functions to show how merge sort works
//! What is the difference from the other sorting methods?
//! It is a faster sorting algorithm than bubble sort, but is has its own costs
//! The first difference is that we divide the list, sort each half and bring the sorted halves together
//! It's the divide and conquer method. A recursive function that will divide the list in halves
//! until there is no more place for division, i. e., length = 1
//! Then the merge function is called with both halves and order them
//! ## What is the runtime complexity of a Merge Sort?
//! `O(n * log(n))` because the number of iterations will decrease as the number of halves grow because
//!  we would have less items to sort. So this is more performant than O(n^2)

use std::fmt::Debug;

/// # Examples
///
/// ```
/// use ds_algo::algorithms::sorting::merge_sort::{merge_sort};
/// let v = vec![4, 6, 1, 8, 11, 12];
/// let v = merge_sort(v);
/// assert_eq!(v, vec![1, 4, 6, 8, 11, 12]);
/// ```
pub fn merge_sort<T: PartialOrd + Debug>(mut list: Vec<T>) -> Vec<T> {
    if list.len() == 1 {
        return list;
    }

    let second_half = list.split_off(list.len() / 2);
    let first_half = merge_sort(list);
    let second_half = merge_sort(second_half);

    merge(first_half, second_half)
}

fn merge<T: PartialOrd>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    //Creating the final vector
    let mut result = Vec::with_capacity(left.len() + right.len());

    //Now we should join the halves, but the half with the lowest values should come in front

    let mut first_half_iterator = left.into_iter();
    let mut second_half_iterator = right.into_iter();

    let mut first_half_peek = first_half_iterator.next();
    let mut second_half_peek = second_half_iterator.next();

    loop {
        match first_half_peek {
            //If there is anything in the left side
            Some(ref a_val) => match second_half_peek {
                //If there is also something in the right side we compare to see which value is lower
                Some(ref b_val) => {
                    if b_val < a_val {
                        result.push(second_half_peek.take().unwrap());
                        second_half_peek = second_half_iterator.next();
                    } else {
                        result.push(first_half_peek.take().unwrap());
                        first_half_peek = first_half_iterator.next();
                    }
                }
                //If there is nothing in the right side we extend the left side and return the result
                None => {
                    result.push(first_half_peek.take().unwrap());
                    result.extend(first_half_iterator);
                    return result;
                }
            },
            //If there is nothing in the left side
            //1. We check if there is anything in the right side and push it to the result
            //2. If not, we extend the right side and return
            None => {
                if let Some(b_val) = second_half_peek {
                    result.push(b_val);
                }

                result.extend(second_half_iterator);
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::merge;

    #[test]
    fn test_merge_function() {
        let left = vec![6];
        let right = vec![1];
        let result = merge(left, right);
        assert_eq!(result, vec![1, 6]);
    }
}
