use std::collections::HashSet;
use std::hash::Hash;

use crate::commons::*;
use crate::lyndon::*;

/// Checks whether a given string is a de Bruijn word.
// pub fn is_debruijn(text: &bstr, order: usize) -> bool {
pub fn is_debruijn<T>(text: &[T], order: usize) -> bool
where
    T: Clone + Eq + Hash,
{
    let alphabet = alphabet_set(text);
    if alphabet.len() == 1 {
        return text.len() == 1;
    }
    // println!("alphabet size={}, order={}", alphabet.len(), order);
    let expect_len = alphabet.len().pow(order as u32) + order - 1;
    if expect_len != text.len() {
        // dbg!(alphabet, order, expect_len, text.len());
        return false;
    }
    let mut substrs = HashSet::new();
    for i in 0..(text.len() - order) {
        if !alphabet.contains(&text[i]) || substrs.contains(&text[i..(i + order)]) {
            return false;
        }
        substrs.insert(&text[i..(i + order)]);
    }
    true
}

#[test]
fn test_is_debruijn() {
    // assert!(is_debruijn(str2bstr("a"), 1));
    assert!(is_debruijn(br"a", 1));
    assert!(is_debruijn(br"b", 1));
    assert!(is_debruijn(br"ab", 1));
    assert!(is_debruijn(br"ba", 1));
    assert!(!is_debruijn(br"aa", 1));
    assert!(is_debruijn(br"aabba", 2));
    assert!(!is_debruijn(br"aabb", 2));
    assert!(is_debruijn(br"aaababbbaa", 3));
}

/// Computes a de Bruijn Word.
///
/// Let $A$ be alphabet, and $k$ be a integer.
/// A string $x$ is a de Bruijn word if each string of $A^k$ occurs
/// only once in $x$.
pub fn debruijn<T>(alphabet: &[T], order: usize) -> Vec<T>
where
    T: Clone + Ord + Eq + Hash,
{
    debruijn_lex_smallest(alphabet, order)
}

/// Compute the divisors of a given integer.
fn divisors(n: usize) -> Vec<usize> {
    let high = (n as f64).sqrt().floor() as usize;
    let mut res = (1..=high).filter(|&i| n % i == 0).collect::<Vec<usize>>();
    res.push(n);
    res
}

/// Compute the lexicographically smallest de Bruijn word.
pub fn debruijn_lex_smallest<T>(alphabet: &[T], order: usize) -> Vec<T>
where
    T: Clone + Ord + Eq + Hash,
{
    let mut lyndons: Vec<Vec<T>> = divisors(order)
        .into_iter()
        .flat_map(|len| enum_lyndon_len_eq(alphabet, len))
        .collect();
    lyndons.sort();
    fn index2<T>(alphabet: &[T], key: &T) -> usize
    where
        T: Clone + Ord + Eq + Hash,
    {
        let pos = alphabet.iter().position(|x| x == key);
        pos.unwrap()
    }
    // println!("lyndons={}", bstrs2string(&lyndons));
    let mut debu = lyndons.concat();
    debu.extend(debu[..(order - 1)].to_vec());
    // ordering
    let mut alpha_asc: Vec<T> = alphabet.to_vec();
    alpha_asc.sort();
    let is_asc = {
        let mut res = true;
        for i in 0..alphabet.len() {
            if alphabet[i] != alpha_asc[i] {
                res = false;
                break;
            }
        }
        res
    };
    // println!(
    //     "alphabet={:?}, alpha_asc={:?}, is_asc={}",
    //     alphabet, alpha_asc, is_asc
    // );
    if !is_asc {
        (0..debu.len()).for_each(|i| {
            let j = index2(&alpha_asc, &debu[i]);
            debu[i] = alphabet[j].clone();
        });
    }
    debu
}

#[test]
fn test_debruijn() {
    let alpha = alphabet_asc(br"ab");
    let deb2 = debruijn_lex_smallest(&alpha, 2);
    let deb3 = debruijn_lex_smallest(&alpha, 3);
    assert_eq!(br"aabba".to_vec(), deb2);
    assert_eq!(br"aaababbbaa".to_vec(), deb3);
    let alpha_desc = alphabet_desc(br"ab");
    let deb4 = debruijn_lex_smallest(&alpha_desc, 2);
    let deb5 = debruijn_lex_smallest(&alpha_desc, 3);
    assert_eq!(br"bbaab".to_vec(), deb4);
    assert_eq!(br"bbbabaaabb".to_vec(), deb5);
}
