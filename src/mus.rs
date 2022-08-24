use std::cmp::max;

use crate::lcp::*;
use crate::suffix_array::sanaive::*;

/// Compute Minimal Unique Substrings (MUSs) array `musa` of a string `text`.
/// for `musa[i]=j`, text[j..i] is MUS.
pub fn minimum_unique_substrs<T>(text: &[T]) -> Vec<Vec<T>>
where
    T: Ord + Clone,
{
    let sa = make_sa(text);
    let lcpa = make_lcpa(text, &sa);
    minimum_unique_substrs_aux(text, &sa, &lcpa)
}

pub fn minimum_unique_substrs_aux<T>(text: &[T], sa: &[usize], lcpa: &[usize]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let n = text.len();
    let mut musa = vec![None; n];
    for i in 0..text.len() {
        let l2 = if i + 1 < n { lcpa[i + 1] } else { 0 };
        let l = max(lcpa[i], l2);
        if sa[i] + l < n {
            musa[sa[i] + l] = match (musa[sa[i] + l], sa[i]) {
                (Some(x), y) => Some(max(x, y)),
                (None, y) => Some(y),
            };
        }
    }
    musa.iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, &x)| text[x.unwrap()..=i].to_vec())
        .collect()
}
