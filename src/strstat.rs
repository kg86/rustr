use std::collections::{HashMap, HashSet};

use crate::tools::*;

use crate::bstr;

pub struct StrStat<'text> {
    pub text: &'text bstr,
    pub n: usize,
    // pub epsilon: &'text bstr,
    sigma: HashSet<&'text bstr>,
    pres: HashSet<&'text bstr>,
    ppres: HashSet<&'text bstr>,
    sufs: HashSet<&'text bstr>,
    psufs: HashSet<&'text bstr>,
    substrs: HashSet<&'text bstr>,
    occ_map: HashMap<&'text bstr, usize>,
    lgmap: HashMap<&'text bstr, HashSet<&'text bstr>>,
    rgmap: HashMap<&'text bstr, HashSet<&'text bstr>>,
}

impl<'text> StrStat<'text> {
    pub fn new(w: &'text bstr) -> Self {
        let epsilon: &bstr = &[];
        let sigma = sigma(w);
        let pres = pref_set(w);
        let mut ppres = pres.clone();
        ppres.remove(epsilon);
        let sufs = suf_set(w);
        let mut psufs = sufs.clone();
        psufs.remove(epsilon);
        let subs = substrs(w);
        let lgmap = leq_groups(&subs, &pres);
        let rgmap = req_groups(&subs, &sufs);
        let suf_trie = suffix_trie(&sigma, &subs);
        Self {
            text: w,
            n: w.len(),
            // epsilon: &[],
            sigma,
            pres: pres,
            ppres: ppres,
            sufs: sufs,
            psufs: psufs,
            substrs: subs,
            occ_map: nocc(w),
            lgmap,
            rgmap,
            // suf_trie: suf_trie,
        }
    }

    pub fn nocc(&self, w: &bstr) -> usize {
        *self.occ_map.get(w).unwrap_or(&0)
    }

    pub fn left_maximal(&self, sub: &bstr) -> &bstr {
        self.rgmap
            .get(sub)
            .expect("has substring.")
            .iter()
            .max_by_key(|k| k.len())
            .expect("has at least one element.")
    }

    pub fn right_maximal(&self, sub: &bstr) -> &bstr {
        self.lgmap
            .get(sub)
            .expect("has substring.")
            .iter()
            .max_by_key(|k| k.len())
            .expect("has at least one element.")
    }

    /// set of characters that appear in the left on the given substring `sub`.
    pub fn left_extention(&self, sub: &bstr) -> HashSet<&bstr> {
        self.sigma
            .iter()
            .filter(|&c| self.substrs.contains([c, sub].concat().as_slice()))
            .map(|k| *k)
            .collect()
    }

    /// set of characters that appear in the right on the given substring `sub`.
    pub fn right_extention(&self, sub: &bstr) -> HashSet<&bstr> {
        self.sigma
            .iter()
            .filter(|&c| self.substrs.contains([sub, c].concat().as_slice()))
            .map(|k| *k)
            .collect()
    }
}
