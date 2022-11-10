// next smaller suffix
// nss[i]=min of text.len() or j s.t. text[j..] < text[i..] for j > i

pub fn nss<T>(text: &[T]) -> Vec<usize>
where
    T: Ord,
{
    nss_naive(text)
}

pub fn nss_naive<T>(text: &[T]) -> Vec<usize>
where
    T: Ord,
{
    (0..text.len())
        .into_iter()
        .map(|i| nssi_naive(text, i))
        .collect()
}

pub fn nssi_naive<T>(text: &[T], i: usize) -> usize
where
    T: Ord,
{
    for j in i + 1..text.len() {
        if text[j..] < text[i..] {
            return j;
        }
    }
    text.len()
}
