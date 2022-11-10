use std::cmp::max;
use std::convert::TryFrom;

use crate::suffix_array::sais::make_sa_induce;

/// Relative LZ
pub struct RLZ {
    pub text: Vec<u8>,
    sa: Vec<u32>,
}

impl RLZ {
    pub fn new(text: Vec<u8>) -> Self {
        let sa = make_sa_induce(&text);
        Self { text, sa }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the lowest position in the range [beg, end] that a suffix have
    /// a character greater than or equal to `c` at position `len`.
    /// Note that `end` may be `self.len()` which is the out of the suffix array.
    fn lower_bound(&self, mut beg: usize, mut end: usize, len: usize, c: u8) -> usize {
        let (beg_orig, end_orig) = (beg, end);
        // println!("lower_bound({}, {}, {}, {})", beg, end, len, c);
        while beg < end {
            let mid = (beg + end) / 2;
            let i = self.sa[mid] as usize + len;
            if i >= self.len() || self.text[i] < c {
                beg = mid + 1;
            } else {
                end = mid;
            }
        }

        if 0 < beg && beg < end_orig && beg_orig < beg {
            let b0 = self.sa[beg - 1] as usize + len;
            let b1 = self.sa[beg] as usize + len;
            if b0 < self.len() && b1 < self.len() {
                // println!("c={}, (b0, b1)=({}, {})", c, self.text[b0], self.text[b1]);
                debug_assert!((self.text[b0] < c && c <= self.text[b1]));
            }
        }
        beg
    }

    /// Returns the lowest position in the range [beg, end] that a suffix have
    /// a character greater than `c` at position `len`.
    /// Note that `end` may be `self.len()` which is the out of the suffix array.
    fn upper_bound(&self, mut beg: usize, mut end: usize, len: usize, c: u8) -> usize {
        while beg < end {
            // while beg + 1 < end {
            let mid = (beg + end) / 2;
            let i = self.sa[mid] as usize + len;
            if i >= self.len() || self.text[i] <= c {
                beg = mid + 1;
            } else {
                end = mid;
            }
        }
        beg
    }

    /// Prints the suffixes in the range `[beg, end)`.
    pub fn print_suffix_range(&self, beg: usize, end: usize, len: usize) {
        for i in beg..end {
            let j = self.sa[i] as usize;
            println!("[{}]: {:?}", i, &self.text[j..(j + len)]);
        }
    }

    /// Finds the next suffix interval that the given suffix interval can be extended with a given character.
    fn extend_range(&self, beg: usize, end: usize, len: usize, c: u8) -> (usize, usize) {
        (
            self.lower_bound(beg, end, len, c),
            self.upper_bound(beg, end, len, c),
        )
    }

    /// Finds suffix interval on the suffix array such that the suffixes on the interval have common prefix and they are longest.
    /// Returns a tuple beginning and ending of the interval, and its length.
    pub fn lcp_range(&self, pat: &[u8]) -> (usize, usize, usize) {
        let mut res = 0;
        let mut beg = 0;
        let mut end = self.len();
        for (i, &c) in pat.iter().enumerate() {
            let (beg2, end2) = self.extend_range(beg, end, i, c);
            if beg2 == end2 {
                res = i;
                break;
            }
            debug_assert_eq!(self.text[self.sa[beg2] as usize + i], c);
            debug_assert_eq!(self.text[self.sa[end2 - 1] as usize + i], c);
            (beg, end) = (beg2, end2);
            res += 1;
        }
        if res > 0 {
            let b = self.sa[beg] as usize;
            debug_assert_eq!(pat[..res], self.text[b..b + res]);
        }
        (beg, end, res)
    }

    /// Returns a factor that equals a prefix of a given pattern.
    /// The factor is a pair `(prev_len, prev_occ)`.
    /// If, `prev_len == 0`, the factor represents a character `text[prev_occ]`.
    /// Otherwise, it represents a text `text[prev_occ..prev_occ+prev_len]`.
    pub fn encode_factor(&self, pat: &[u8]) -> (usize, usize) {
        let (beg, _, len) = self.lcp_range(pat);
        match len {
            0 => (0, pat[0] as usize),
            _ => (len, self.sa[beg] as usize),
        }
    }

    /// Returns factors that equal a given pattern.
    pub fn encode_factors(&self, pat: &[u8]) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let mut i = 0;
        while i < pat.len() {
            let factor = self.encode_factor(&pat[i..]);
            if factor.0 > 0 {
                debug_assert_eq!(
                    self.text[factor.1..factor.1 + factor.0],
                    pat[i..i + factor.0]
                );
            }
            res.push(factor);
            i += max(1, factor.0);
        }
        res
    }

    /// Returns the original string of the factor.
    pub fn decode_factor(&self, factor: &(usize, usize)) -> Vec<u8> {
        if factor.1 == 0 {
            [u8::try_from(factor.0).unwrap()].to_vec()
        } else {
            self.text[factor.0..factor.0 + factor.1].to_vec()
        }
    }

    /// Returns the original string of the factors.
    pub fn decode_factors(&self, factors: &[(usize, usize)]) -> Vec<u8> {
        let mut res = vec![];
        for factor in factors {
            res.extend(self.decode_factor(factor));
        }
        res
    }
}

#[test]
fn test_rlz() {
    let text1 = br"abaaab";
    let text2 = br"aaaaab";
    let rlz = RLZ::new(text1.to_vec());
    let factors = rlz.encode_factors(text2);
    assert_eq!(factors, vec![(3, 2), (3, 3)]);
}
