//! Fibonacci String
//!
//! Let $x$ and $y$ be the first and second characters.
//! A fibonacci string is defined by
//! - $F(0)=x$
//! - $F(1)=x+y$
//! - $F(i) = F(i-1) + F(i-2)$ for $i>1$.
//!
//! For example, let $x=\mathrm{a}$ and $y=\mathrm{b}$ be the first and second characters.
//! - $F(0) = \mathrm{a}$
//! - $F(1) = \mathrm{ab}$
//! - $F(2) = \mathrm{aba}$
//! - $F(3) = \mathrm{abaab}$, and so on.

use crate::bstr::*;
pub fn fibstr(i: usize, first: &bstr, second: &bstr) -> BString {
    if i == 0 {
        first.to_vec()
    } else if i == 1 {
        [first, second].concat()
    } else {
        let mut prev = first.to_vec();
        let mut cur = [first, second].concat();
        for _ in 2..=i {
            let next = [cur.clone(), prev].concat();
            prev = cur;
            cur = next;
        }
        cur
    }
}

pub fn fibstr_ab(i: usize) -> BString {
    fibstr(i, &"a".as_bytes(), &"b".as_bytes())
}

pub fn fibstr_01(i: usize) -> BString {
    fibstr(i, &"0".as_bytes(), &"1".as_bytes())
}

#[test]
fn test_fib() {
    assert_eq!(str2bstring("a"), fibstr_ab(0));
    assert_eq!(str2bstring("ab"), fibstr_ab(1));
    assert_eq!(str2bstring("aba"), fibstr_ab(2));
    assert_eq!(str2bstring("abaab"), fibstr_ab(3));
}
