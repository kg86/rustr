use std::collections::HashSet;

use crate::bstr::*;
use crate::commons::*;
use crate::lyndon::*;

/// Checks whether a given string is a de Bruijn word.
pub fn is_debruijn(text: &bstr, order: usize) -> bool {
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
        if !alphabet.contains(&text[i..i + 1]) || substrs.contains(&text[i..(i + order)]) {
            return false;
        }
        substrs.insert(&text[i..(i + order)]);
    }
    return true;
}

#[test]
fn test_is_debruijn() {
    assert!(is_debruijn(str2bstr("a"), 1));
    assert!(is_debruijn(str2bstr("b"), 1));
    assert!(is_debruijn(str2bstr("ab"), 1));
    assert!(is_debruijn(str2bstr("ba"), 1));
    assert!(!is_debruijn(str2bstr("aa"), 1));
    assert!(is_debruijn(str2bstr("aabba"), 2));
    assert!(!is_debruijn(str2bstr("aabb"), 2));
    assert!(is_debruijn(str2bstr("aaababbbaa"), 3));
}

/// Computes a de Bruijn Word.
///
/// Let $A$ be alphabet, and $k$ be a integer.
/// A string $x$ is a de Bruijn word if each string of $A^k$ occurs
/// only once in $x$.
pub fn debruijn(alphabet: &Vec<BString>, order: usize) -> BString {
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
pub fn debruijn_lex_smallest(alphabet: &Vec<BString>, order: usize) -> BString {
    let mut lyndons: Vec<BString> = divisors(order)
        .into_iter()
        .map(|len| enum_lyndon_len_eq(alphabet, len))
        .flatten()
        .collect();
    lyndons.sort();
    fn index2(alphabet: &Vec<BString>, key: &bstr) -> usize {
        let pos = alphabet.iter().position(|x| x == key);
        // println!("key={}, pos={:?}", bstr2string(key), pos);
        pos.unwrap()
    }
    // println!("lyndons={}", bstrs2string(&lyndons));
    let mut debu = lyndons.concat();
    debu.extend(debu[..(order - 1)].to_vec());
    // ordering
    let mut alpha_asc = alphabet.clone();
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
        for i in 0..debu.len() {
            let j = index2(&alpha_asc, &debu[i..i + 1]);
            debu[i] = alphabet[j][0];
        }
    }
    debu
}

#[test]
fn test_debruijn() {
    let alpha = alphabet_asc(str2bstr("ab"));
    let deb2 = debruijn_lex_smallest(&alpha, 2);
    let deb3 = debruijn_lex_smallest(&alpha, 3);
    assert_eq!(str2bstring("aabba"), deb2);
    assert_eq!(str2bstring("aaababbbaa"), deb3);
    let alpha_desc = alphabet_desc(str2bstr("ab"));
    let deb4 = debruijn_lex_smallest(&alpha_desc, 2);
    let deb5 = debruijn_lex_smallest(&alpha_desc, 3);
    assert_eq!(str2bstring("bbaab"), deb4);
    assert_eq!(str2bstring("bbbabaaabb"), deb5);
}
