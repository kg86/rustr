use std::cmp::max;

use crate::suffix_array::sais::make_sa_induce;

/// Make previous/next smaller value arrays.
pub fn make_psv_nsv_texta(sa: &[usize]) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let n = sa.len();
    let mut psva = vec![None; n];
    let mut nsva = vec![None; n];

    let mut previ = None;
    for i in 0..n {
        while previ.is_some() && previ.unwrap() > sa[i] as usize {
            let pi = previ.unwrap();
            nsva[pi] = Some(sa[i] as usize);
            previ = psva[pi];
        }
        psva[sa[i] as usize] = previ;
        previ = Some(sa[i] as usize);
    }
    (psva, nsva)
}

/// Compute longest extention of text at positions `i` and `j`.
fn longest_extention(text: &[u8], i: usize, j: usize) -> usize {
    let mut match_len = 0;
    while i + match_len < text.len() && j + match_len < text.len() {
        if text[i + match_len] != text[j + match_len] {
            break;
            // return match_len;
        }
        match_len += 1;
    }
    match_len
}

/// Compute a longest previous factor of `text[i..]` that starts at j.
/// Returns a pair `(prev_len, prev_occ)`, where
/// `prev_len`: the length of the factor
/// `prev_occ`: If `prev_len==0`, it represents a chracter `text[i]`. Otherwise, it represents the previous occurrence that `text[i..i+prev_len]==text[prev_occ..prev_occ+prev_len]`.
fn calc_factor(text: &[u8], i: usize, j: Option<usize>) -> (usize, usize) {
    let plen = match j {
        Some(pos) => longest_extention(text, i, pos),
        None => 0,
    };
    let pocc = if plen == 0 {
        text[i] as usize
    } else {
        j.unwrap()
    };
    (plen, pocc)
}

/// Compute LZ77 factors.
/// Returns pairs `[(prev_len, prev_occ), ...]`, where
/// `prev_len`: the length of the factor
/// `prev_occ`: If `prev_len==0`, it represents a chracter `text[i]`, where `i` is a position of the factor in the text. Otherwise, it represents the previous occurrence that `text[i..i+prev_len]==text[prev_occ..prev_occ+prev_len]`.
pub fn lz77(text: &[u8]) -> Vec<(usize, usize)> {
    let sa: Vec<usize> = {
        let sa32 = make_sa_induce(text);
        sa32.into_iter().map(|x| x as usize).collect()
    };
    let (psva, nsva) = make_psv_nsv_texta(&sa);
    let mut i = 0;
    let n = text.len();
    let mut res = vec![];
    while i < n {
        let (plen, pocc) = calc_factor(text, i, psva[i]);
        let (nlen, nocc) = calc_factor(text, i, nsva[i]);
        let (prev_len, prev_occ) = max((plen, pocc), (nlen, nocc));
        res.push((prev_len, prev_occ));
        i += max(1, prev_len);
    }
    res
}

#[test]
fn test_lz77() {
    let text = br"ab";
    let res = vec![(0, text[0] as usize), (0, text[1] as usize)];
    assert_eq!(res, lz77(text));
    assert_eq!(lz77(br"aaaaaa"), vec![(0, br"a"[0] as usize), (5, 0)]);
    assert_eq!(
        lz77(br"baaaaaa"),
        vec![(0, br"b"[0] as usize), (0, br"a"[0] as usize), (5, 1)]
    );
    assert_eq!(
        lz77(br"abababab"),
        vec![(0, br"a"[0] as usize), (0, br"b"[0] as usize), (6, 0)]
    );
}
