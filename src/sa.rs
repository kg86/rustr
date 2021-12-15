use crate::bstr::*;

/// Returns suffix array.
pub fn make_sa(text: &bstr) -> Vec<usize> {
    make_sa_naive(text)
}

/// Returns suffix array using a naive algorithm.
pub fn make_sa_naive(text: &bstr) -> Vec<usize> {
    let n = text.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&x, &y| text[x as usize..].cmp(&text[y as usize..]));
    sa
}

#[test]
fn test_sa() {
    let text = str2bstr("bananaba$");
    let sa = make_sa(text);
    let ans = vec![8, 7, 5, 3, 1, 6, 0, 4, 2];
    assert_eq!(ans, sa);
}
