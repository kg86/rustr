use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    rc::Rc,
};

pub fn make_sa_mm(text: &[u8]) -> Vec<u32> {
    let n = text.len();
    let mut sa: Vec<u32> = (0..n).into_iter().map(|x| x as u32).collect();
    let rank: Vec<u32> = text.iter().map(|&x| x as u32).collect();
    let rank = Rc::new(RefCell::new(rank));
    let len: Rc<Cell<usize>> = Rc::new(Cell::new(1 as usize));
    let compare_suf = |i: &u32, j: &u32| {
        let (i, j) = (*i as usize, *j as usize);
        let l = len.get();
        let rank = rank.borrow();
        if rank[i] != rank[j] {
            rank[i].cmp(&rank[j])
        } else if i + l >= n {
            Ordering::Less
        } else if j + l >= n {
            Ordering::Greater
        } else {
            rank[i + l].cmp(&rank[j + l])
        }
    };
    while len.get() < n {
        sa.sort_by(compare_suf);
        let mut tmp = vec![0u32; n];
        tmp[sa[0] as usize] = 0;
        for i in 1..n {
            tmp[sa[i] as usize] = tmp[sa[i - 1] as usize]
                + if compare_suf(&sa[i - 1], &sa[i]) == Ordering::Less {
                    1
                } else {
                    0
                };
        }
        (0..n).for_each(|i| rank.borrow_mut()[i] = tmp[i]);

        len.set(len.get() * 2);
    }
    sa
}
