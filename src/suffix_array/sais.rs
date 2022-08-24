// sa-is

use std::{cmp::max, fmt::Debug};

const EMPTY: u32 = u32::MAX;

pub trait IntoUsize {
    #[allow(clippy::wrong_self_convention)]
    fn into_usize(&self) -> usize;
}

macro_rules! into_usize {
    ($($type:ident),*) => {
        $(impl IntoUsize for $type {
            #[inline(always)]
            fn into_usize(&self) -> usize {
                *self as usize
            }
        })*
    };
}

into_usize!(u8, u16, u32, u64);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Type {
    TypeL,
    TypeS,
}

#[inline(always)]
fn is_lms(types: &[Type], i: usize) -> bool {
    types[i] == Type::TypeS && (i == 0 || types[i - 1] == Type::TypeL)
}

/// Make a type array `types`.
/// `types[i]` stores the suffix type of `text[i..]`.
fn make_typea<T>(text: &[T], types: &mut [Type])
where
    T: Ord + IntoUsize,
{
    debug_assert!(!text.is_empty());
    types[text.len() - 1] = Type::TypeL;
    for i in (1..text.len()).rev() {
        types[i - 1] = match text[i - 1].cmp(&text[i]) {
            std::cmp::Ordering::Equal => types[i],
            std::cmp::Ordering::Less => Type::TypeS,
            std::cmp::Ordering::Greater => Type::TypeL,
        };
    }
}

/// Make a bucket array `bkt`.
/// `bkt[c]` indicates the interval of suffixes prefixed by `c` in the suffix array.
/// If `head=true/false`, `bkt[c]` indicates the head/tail of the interval.
/// More precisely, if `head=true`, `bkt[c]` is the number of characters less than `c`,
/// if `head=false`, it is the number of characters less than or equal to `c`.
fn make_bkta<T>(text: &[T], bkt: &mut [u32], head: bool)
where
    T: Ord + IntoUsize,
{
    bkt.iter_mut().for_each(|x| *x = 0);
    text.iter().for_each(|c| bkt[c.into_usize()] += 1);
    if head {
        let mut sum = 0;
        (0..bkt.len()).for_each(|i| {
            (sum, bkt[i]) = (sum + bkt[i], sum);
        });
    } else {
        for i in 1..bkt.len() {
            bkt[i] += bkt[i - 1];
        }
    }
}

/// Induce and sort L-type suffixes.
fn induce_lsuf<T>(text: &[T], sa: &mut [u32], bkt: &mut [u32], types: &mut [Type])
where
    T: Ord + IntoUsize,
{
    make_bkta(text, bkt, true);
    {
        // The last suffix must be L-type suffix and it is not induced by any suffixes.
        // We put it in the head of its corresponding suffix interval.
        let last_c = text[text.len() - 1].into_usize();
        sa[bkt[last_c] as usize] = (text.len() - 1) as u32;
        bkt[last_c] += 1;
    }

    for i in 0..sa.len() {
        let sufi = sa[i];
        if sa[i] != EMPTY && sufi > 0 && types[(sufi - 1) as usize] == Type::TypeL {
            let c = text[(sufi - 1) as usize].into_usize();
            sa[bkt[c] as usize] = sufi - 1;
            bkt[c] += 1;
        }
    }
}

/// Induce and sort S-type suffixes.
fn induce_ssuf<T>(text: &[T], sa: &mut [u32], bkt: &mut [u32], types: &mut [Type])
where
    T: Ord + IntoUsize,
{
    make_bkta(text, bkt, false);
    for i in (0..sa.len()).rev() {
        let sufi = sa[i];
        if sa[i] != EMPTY && sufi > 0 && types[(sufi - 1) as usize] == Type::TypeS {
            let c = text[(sufi - 1) as usize].into_usize();
            bkt[c] -= 1;
            sa[bkt[c] as usize] = sufi - 1;
        }
    }
}

/// Make a suffix array.
pub fn make_sa_induce<T>(text: &[T]) -> Vec<u32>
where
    T: Ord + IntoUsize,
{
    let n = text.len();
    let mut sa = vec![0u32; n];
    let mut bkt = vec![0u32; max(256, n)];
    let mut types = vec![Type::TypeL; n];
    sais(text, &mut sa, &mut bkt, &mut types);
    sa
}

/// Make a suffix array by induced sorting algorithm.
fn sais<T>(text: &[T], sa: &mut [u32], bkt: &mut [u32], types: &mut [Type])
where
    T: Ord + IntoUsize,
{
    let n = text.len();
    make_typea(text, types);
    sa.iter_mut().for_each(|x| *x = EMPTY);

    // Store all LMS-substrings in the tail of each suffix interval.
    make_bkta(text, bkt, false);
    (0..n).for_each(|i| {
        if is_lms(types, i) {
            let c = text[i].into_usize();
            bkt[c] -= 1;
            sa[bkt[c] as usize] = i as u32;
        }
    });

    // Sort LMS-substrings
    induce_lsuf(text, sa, bkt, types);
    induce_ssuf(text, sa, bkt, types);

    // Store LMS-substrings in the head of SA[0..nLms]
    let mut num_lms: u32 = 0;
    for i in 0..n {
        let sufi = sa[i];
        sa[i] = EMPTY;
        // assert!(sufi != EMPTY);
        if is_lms(types, sufi as usize) {
            sa[num_lms as usize] = sufi;
            num_lms += 1;
        }
    }

    // Here, sa[0..num_lms] stores LMS-substring in sorted, and the rest contains `EMPTY`.
    // For an LMS-substring beginning at `i`, we compute its rank and store it in `sa[num_lms+sa[i]/2]`.

    let mut num_distinct_lms: u32 = 0;
    if num_lms > 0 {
        num_distinct_lms = 1;
        sa[(num_lms + sa[0] / 2) as usize] = 0;
    }
    for i in 1..(num_lms as usize) {
        let mut sufi = sa[i] as usize;
        let mut prev = sa[i - 1] as usize;
        if text[sufi] != text[prev] {
            num_distinct_lms += 1;
        } else {
            let len = n - max(sufi, prev);
            for _ in 1..len {
                sufi += 1;
                prev += 1;
                // comparison of types is not necessary, but it may accelerate the comparison.
                // if text[sufi] != text[prev]  || types[sufi] != types[prev]{
                if text[sufi] != text[prev] {
                    num_distinct_lms += 1;
                    break;
                } else if is_lms(types, sufi) && is_lms(types, prev) {
                    break;
                }
            }
        }

        sa[(num_lms + sa[i] / 2) as usize] = num_distinct_lms - 1;
    }

    // Make a new text substituting the text with LMS-substrings by their ranks.
    // We store it in sa[num_lms..2*num_lms].
    let mut text_end = num_lms as usize;
    for i in num_lms as usize..n {
        if sa[i] != EMPTY {
            let rank = sa[i];
            sa[i] = EMPTY;
            sa[text_end] = rank;
            text_end += 1;
        }
    }

    // Make a suffix array of the new text.
    // Namely, we sort LMS-suffixes not LMS-substrings.
    let (front, _) = sa.split_at_mut(text_end);
    let (sa2, text2) = front.split_at_mut(num_lms as usize);
    assert_eq!(text2.len(), sa2.len());
    if num_distinct_lms != num_lms {
        sais(
            text2,
            sa2,
            &mut bkt[..num_lms as usize],
            &mut types[..num_lms as usize],
        );
        make_typea(text, types);
    } else {
        for (i, &c) in text2.iter().enumerate() {
            sa2[c as usize] = i as u32;
        }
    }

    // Here, `sa2` is the suffix array of `text2`.
    // We convert the indexes of `text2` to ones of `text`,
    // that is the indexes of their corresponding LMS-suffixes.
    let mut j = 0;
    for i in 0..n {
        if is_lms(types, i) {
            text2[j] = i as u32;
            j += 1;
        }
    }
    for i in 0..sa2.len() {
        sa2[i] = text2[sa2[i] as usize];
    }

    // Clean up `sa[num_lms..]`.
    // for i in num_lms as usize..sa.len() {
    (num_lms as usize..(2 * num_lms as usize)).for_each(|i| {
        sa[i] = EMPTY;
    });

    // Store sorted LMS-suffixes to the tail of each suffix interval with preserving the order.
    make_bkta(text, bkt, false);
    for i in (0..num_lms as usize).rev() {
        let suf = sa[i];
        sa[i] = EMPTY;
        let c = text[suf as usize].into_usize();
        bkt[c] -= 1;
        sa[bkt[c] as usize] = suf;
    }

    // Sort L-suffixes
    induce_lsuf(text, sa, bkt, types);

    // Sort S-suffixes
    induce_ssuf(text, sa, bkt, types);
}

// fn show<T>(text: &[T], sa: &[u32], types: &[Type], bkt: &[u32])
// where
//     T: Ord + IntoUsize,
// {
//     let text: Vec<usize> = text.iter().map(|x| x.into_usize()).collect();
//     println!("text={:?}", text);
//     println!("sa={:?}", sa);
//     println!("types={:?}", types);
//     println!("bkt={:?}", bkt);
// }
