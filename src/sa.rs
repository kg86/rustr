/// Returns suffix array.
pub fn make_sa<T>(text: &[T]) -> Vec<usize>
where
    T: Ord,
{
    make_sa_naive(text)
}

/// Returns suffix array using a naive algorithm.
pub fn make_sa_naive<T>(text: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let n = text.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&x, &y| text[x as usize..].cmp(&text[y as usize..]));
    sa
}

#[test]
fn test_sa() {
    let text = br"bananaba$";
    let sa = make_sa(text);
    let ans = vec![8, 7, 5, 3, 1, 6, 0, 4, 2];
    assert_eq!(ans, sa);
}
