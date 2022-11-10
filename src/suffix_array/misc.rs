pub fn make_isa(sa: &[usize]) -> Vec<usize> {
    let n = sa.len();
    let mut isa = vec![0; n];
    sa.iter().for_each(|&i| isa[sa[i]] = i);
    isa
}
