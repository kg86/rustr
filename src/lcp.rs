/// Retruns LCP array.
pub fn make_lcpa<T>(text: &[T], sa: &[usize]) -> Vec<usize>
where
    T: Ord,
{
    make_lcpa_kasai(text, sa)
}

/// Retruns LCP array using Kasai algorithm.
pub fn make_lcpa_kasai<T>(text: &[T], sa: &[usize]) -> Vec<usize>
where
    T: PartialEq,
{
    let n = text.len();
    let mut rank = vec![0; text.len()];
    let mut lcp = vec![0; text.len()];
    for (i, &x) in sa.iter().enumerate() {
        rank[x] = i;
    }
    let mut l = 0;
    for i in 0..n - 1 {
        let j = sa[rank[i] - 1];
        while (i + l < n) & (j + l < n) & (text[i + l] == text[j + l]) {
            l += 1;
        }
        lcp[rank[i]] = l;
        l = if l > 0 { l - 1 } else { 0 };
    }
    lcp
}

#[test]
fn test_lcp() {
    use crate::sa::*;
    let text = br"bananaba$";
    let sa = make_sa(text);
    let lcpa = make_lcpa(text, &sa);
    let ans = vec![0, 0, 1, 1, 3, 0, 2, 0, 2];
    assert_eq!(ans, lcpa);
}
