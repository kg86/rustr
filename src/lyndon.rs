use std::hash::Hash;

use crate::bstr::*;
use crate::commons::*;

/// Note that an empty string is not a lyndon word.

/// Checks whether a given string is a lyndon word.
pub fn is_lyndon<T>(text: &[T]) -> bool
where
    T: Clone + PartialOrd + Eq + Hash,
{
    is_lyndon_naive(text)
}

/// Checks whether a given string is a lyndon word in a naive way.
pub fn is_lyndon_naive<T>(text: &[T]) -> bool
where
    T: Clone + std::hash::Hash + PartialOrd + Eq,
{
    if !is_primitive(text) {
        return false;
    }
    let cyclics = enum_rotate_set(text);
    for x in cyclics {
        if text > &x {
            return false;
        }
    }
    true
}

#[test]
fn test_is_lyndon() {
    assert!(is_lyndon_naive(str2bstr("a")));
    assert!(is_lyndon_naive(str2bstr("b")));
    assert!(!is_lyndon_naive(str2bstr("aa")));
    assert!(!is_lyndon_naive(str2bstr("bb")));
    assert!(is_lyndon_naive(str2bstr("aab")));
    assert!(is_lyndon_naive(str2bstr("abb")));
    assert!(!is_lyndon_naive(str2bstr("aba")));
}

/// Enumerates lyndon words of length `len`.
pub fn enum_lyndon_len_eq<T>(alph: &[T], len: usize) -> Vec<Vec<T>>
where
    T: Clone + PartialOrd + Eq + Hash,
{
    let xs = enum_strs_len_eq(alph, len);
    xs.into_iter().filter(|x| is_lyndon(x)).collect()
}

/// Enumerates lyndon words whose lengths are less than or equal to `len`.
pub fn enum_lyndon_len_leq<T>(alpha: &[T], len: usize) -> Vec<Vec<T>>
where
    T: PartialOrd + Clone + Eq + Hash,
{
    let mut res = Vec::new();
    for i in 0..=len {
        res.extend(enum_lyndon_len_eq(alpha, i));
    }
    println!("lyndon debug");
    // println!("{}, len={}", bstrs2string(&res), res.len());
    res
}

#[test]
fn test_enum_lyndon_eq() {
    let alpha = alphabet_asc(str2bstr("ab"));
    let ans_ab2 = vec![str2bstring("ab")];
    let ans_ab12 = vec![str2bstring("a"), str2bstring("b"), str2bstring("ab")];
    let ans_ab3 = ["aab", "abb"]
        .iter()
        .copied()
        .map(str2bstring)
        .collect::<Vec<_>>();
    assert_eq!(ans_ab2, enum_lyndon_len_eq(&alpha, 2));
    assert_eq!(ans_ab12, enum_lyndon_len_leq(&alpha, 2));
    assert_eq!(ans_ab3, enum_lyndon_len_eq(&alpha, 3));
}
