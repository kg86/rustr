use std::hash::Hash;

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
    assert!(is_lyndon_naive(br"a"));
    assert!(is_lyndon_naive(br"b"));
    assert!(!is_lyndon_naive(br"aa"));
    assert!(!is_lyndon_naive(br"bb"));
    assert!(is_lyndon_naive(br"aab"));
    assert!(is_lyndon_naive(br"abb"));
    assert!(!is_lyndon_naive(br"aba"));
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
    let alpha = alphabet_asc(br"ab");
    let ans_ab2 = vec![br"ab".to_vec()];
    let ans_ab12: Vec<Vec<u8>> = vec![br"a".to_vec(), br"b".to_vec(), br"ab".to_vec()];
    let ans_ab3 = [br"aab", br"abb"]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
    assert_eq!(ans_ab2, enum_lyndon_len_eq(&alpha, 2));
    assert_eq!(ans_ab12, enum_lyndon_len_leq(&alpha, 2));
    assert_eq!(ans_ab3, enum_lyndon_len_eq(&alpha, 3));
}
