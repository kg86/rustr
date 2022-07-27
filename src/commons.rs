use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::bstr::*;

/// Returns all prefixes of a given string.
pub fn prefs(text: &bstr) -> Vec<BString> {
    prefs_sli(text).into_iter().map(|x| x.to_vec()).collect()
}

/// Returns all prefixes of a given string.
pub fn prefs_sli(text: &bstr) -> Vec<&bstr> {
    (0..=text.len()).into_iter().map(|i| &text[..i]).collect()
}

/// Returns all suffixes of a given string.
pub fn sufs(text: &bstr) -> Vec<BString> {
    sufs_sli(text).into_iter().map(|x| x.to_vec()).collect()
}

/// Returns all suffixes of a given string.
pub fn sufs_sli(text: &bstr) -> Vec<&bstr> {
    (0..=text.len()).into_iter().map(|i| &text[i..]).collect()
}

/// Returns a set of all prefixes of a given string.
pub fn pref_set(text: &bstr) -> HashSet<BString> {
    prefs_sli(text).into_iter().map(|x| x.to_vec()).collect()
}

/// Returns a set of all prefixes of a given string.
pub fn pref_set_sli(text: &bstr) -> HashSet<&bstr> {
    prefs_sli(text).into_iter().collect()
}

/// Returns a set of all suffixes of a given string.
pub fn suf_set(text: &bstr) -> HashSet<BString> {
    sufs_sli(text).into_iter().map(|x| x.to_vec()).collect()
}

/// Returns a set of all suffixes of a given string.
pub fn suf_set_sli(text: &bstr) -> HashSet<&bstr> {
    sufs_sli(text).into_iter().collect()
}

#[test]
fn test_pre_suf() {
    let text = str2bstr("banana");
    let prefs = prefs(text);
    let sufs = sufs(text);
    // prefix
    for i in 0..text.len() {
        assert_eq!(prefs[i], text[..i]);
    }
    // suffix
    for i in 0..text.len() {
        assert_eq!(sufs[i], text[i..]);
    }
}

/// Returns a set of all substrings of a given string.
pub fn substrs(text: &bstr) -> HashSet<BString> {
    substrs_sli(text).into_iter().map(|x| x.to_vec()).collect()
}

/// Returns a set of all substrings of a given string.
pub fn substrs_sli(text: &bstr) -> HashSet<&bstr> {
    (0..text.len())
        .into_iter()
        .map(|i| (i..=text.len()).into_iter().map(move |j| &text[i..j]))
        .flatten()
        .collect()
}

/// Filters strings that begin with a given string,
/// and returns a set of strings whose prefixes are trimmed by the given string.
pub fn trim_pre(ss: &HashSet<BString>, pre: &bstr) -> HashSet<BString> {
    ss.iter()
        .filter(|x| x.starts_with(pre))
        .map(|x| bstr2bstring(&x[pre.len()..]))
        .collect()
}

#[test]
fn test_trim_pre() {
    let ss = HashSet::from(["abab", "acbb", "ccbb", "acc"].map(str2bstring));
    let pre = str2bstring("ac");
    let pre_ss = trim_pre(&ss, &pre);
    let ans = HashSet::from(["bb", "c"].map(str2bstring));
    assert_eq!(pre_ss, ans);
}

/// Filters strings that end with a given string,
/// and returns a set of strings whose suffixes are trimmed by the given string.
pub fn trim_suf(ss: &HashSet<BString>, suf: &bstr) -> HashSet<BString> {
    // assert!(text.starts_with(pre));
    ss.iter()
        .filter(|x| x.ends_with(suf))
        .map(|x| bstr2bstring(&x[..(x.len() - suf.len())]))
        .collect()
}

#[test]
fn test_trim_suf() {
    let ss = HashSet::from(["abab", "acbb", "ccbb", "acc"].map(str2bstring));
    let suf = str2bstring("bb");
    let suf_ss = trim_suf(&ss, &suf);
    let ans = HashSet::from(["ac", "cc"].map(str2bstring));
    assert_eq!(suf_ss, ans);
}

/// Returns a set of strings that start with a given string.
// pub fn filter_pre(ss: &HashSet<BString>, pre: &BString) -> HashSet<BString> {
pub fn filter_pre(ss: &HashSet<BString>, pre: &bstr) -> HashSet<BString> {
    ss.into_iter()
        .cloned()
        // .filter(|s| pre.len() <= s.len() && s.starts_with(pre))
        .filter(|s| pre.len() <= s.len() && *pre == s[..pre.len()])
        // .filter(|s| pre.len() <= s.len())
        .collect()
}

/// Returns a set of strings that end with a given string.
// pub fn filter_suf(ss: &HashSet<BString>, suf: &BString) -> HashSet<BString> {
pub fn filter_suf(ss: &HashSet<BString>, suf: &bstr) -> HashSet<BString> {
    ss.into_iter()
        .cloned()
        .filter(|s| suf.len() <= s.len() && s.ends_with(suf))
        .collect()
}

#[test]
fn test_filter() {
    let text = str2bstr("banana");
    let sufs = suf_set(text);
    let a = str2bstring("a");
    let sufs_a = filter_pre(&sufs, &a);
    let ans: HashSet<_> = ["anana", "ana", "a"]
        .iter()
        .map(|s| str2bstring(s))
        .collect();
    assert_eq!(ans, sufs_a);

    let pres = pref_set(text);
    let an = str2bstring("an");
    let ans_an: HashSet<_> = ["ban", "banan"].iter().map(|s| str2bstring(s)).collect();
    assert_eq!(ans_an, filter_suf(&pres, &an));
}

/// Returns a set of characters that appear in a given string.
pub fn alphabet_set<T>(text: &[T]) -> HashSet<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    text.iter().cloned().collect()
}

/// Returns a set of characters that appear in a given string.
pub fn alphabet_asc<T>(text: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash + Ord,
{
    let alpha_set = alphabet_set(text);
    let mut alph: Vec<T> = alpha_set.into_iter().collect();
    alph.sort();
    alph
}

/// Returns a set of characters that appear in a given string.
pub fn alphabet_desc<T>(text: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash + Ord,
{
    let mut alph = alphabet_asc(text);
    alph.reverse();
    alph
}

#[test]
fn test_alphabet() {
    let text = str2bstr("banana");
    let ans = br"abn".to_vec();
    // let ans: Vec<_> = ["a", "b", "n"].iter().map(|s| str2bstring(s)).collect();
    // let ans_rev: Vec<_> = ["n", "b", "a"].iter().map(|s| str2bstring(s)).collect();
    let ans_rev = br"nba".to_vec();
    assert_eq!(ans, alphabet_asc(&text));
    assert_eq!(ans_rev, alphabet_desc(&text));
}

/// Returns a string that a given string repeats `k` times.
pub fn repeat<T>(text: &[T], k: usize) -> Vec<T>
where
    T: Clone,
{
    let mut res = vec![];
    for _ in 0..k {
        res.extend_from_slice(text)
    }
    res
}

#[test]
fn test_repeat() {
    let text = str2bstring("abc");
    let ans = str2bstring("abcabcabc");
    assert_eq!(ans, repeat(&text, 3));
}

/// Checks whether a given string is primitive or not.
///
/// A string is primitive if it is not the power of any other string.
/// Namely, a string of the form $x=u^k$ for $k>1$ is not a primitive.
pub fn is_primitive<T>(text: &[T]) -> bool
where
    T: Clone + PartialEq,
{
    is_primitive_naive(text)
}

/// Checks whether a given string is primitive or not by naive way.
pub fn is_primitive_naive<T>(text: &[T]) -> bool
where
    T: Clone + PartialEq,
{
    if text.is_empty() {
        return false;
    }
    for i in 1..text.len() {
        if text.len() % i == 0 {
            let k = text.len() / i;
            if repeat(&text[0..i], k) == text {
                return false;
            }
        }
    }
    true
}

/// Returns the exponent of a given string.
///
/// The exponent is the minimum $k$ such that $x=u^k$.
pub fn exponent(text: &bstr) -> usize {
    for i in 1..text.len() {
        let k = text.len() / i;
        if text.len() % i == 0 && text == repeat(&text[..i], k) {
            return k;
        }
    }
    return 1;
}

pub fn primitive_root(text: &bstr) -> BString {
    let k = exponent(text);
    let len = text.len() / k;
    bstr2bstring(&text[..len])
}

#[test]
fn test_primitive() {
    assert!(!is_primitive(str2bstr("abab")));
    assert!(is_primitive(str2bstr("aba")));
    assert!(is_primitive(str2bstr("ab")));

    assert_eq!(3, exponent(str2bstr("ababab")));
    assert_eq!(str2bstr("ab"), primitive_root(str2bstr("ababab")));
}

/// Rotates a given string `i` position to the left.
pub fn rotate_left<T>(text: &[T], i: usize) -> Vec<T>
where
    T: Clone,
{
    [&text[i..], &text[0..i]].concat()
}

/// Enumerates all rotations of a given string.
pub fn enum_rotate_left<T>(text: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..text.len())
        .into_iter()
        .map(|i| rotate_left(text, i))
        .collect()
}

/// Returns a set of all rotations of a given string.
pub fn enum_rotate_set<T>(text: &[T]) -> HashSet<Vec<T>>
where
    T: Clone + Hash + Eq,
{
    enum_rotate_left(text).into_iter().collect()
}

/// Checks whether a rotation of a given string equals the other string.
pub fn eq_rotate(text1: &bstr, text2: &bstr) -> bool {
    if text1.len() != text2.len() {
        return false;
    }
    for text3 in enum_rotate_left(text2) {
        if text1 == text3 {
            return true;
        }
    }
    return false;
}

#[test]
fn test_rotate() {
    // let text = "aaab".as_bytes();
    let text = str2bstr("aaab");
    assert_eq!(str2bstring("aaab"), rotate_left(text, 0));
    assert_eq!(str2bstring("aaba"), rotate_left(text, 1));
    assert_eq!(str2bstring("abaa"), rotate_left(text, 2));
    assert_eq!(str2bstring("baaa"), rotate_left(text, 3));
    assert_eq!(str2bstring("aaab"), rotate_left(text, 4));

    assert!(eq_rotate(text, str2bstr("abaa")));
    assert!(!eq_rotate(text, str2bstr("bbaa")));
}

/// Enumerates strings of length `len`.
pub fn enum_strs_len_eq<T>(alpha: &[T], len: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if len == 0 {
        return vec![vec![]];
    }
    let prevs = enum_strs_len_eq(alpha, len - 1);
    let mut res = Vec::new();
    for c in alpha {
        for prev in prevs.iter() {
            let mut next = vec![c.clone()];
            next.extend(prev.to_vec());
            // next.push(c.clone());
            // let mut next = prev.to_vec();
            res.push(next);
        }
    }
    assert_eq!(res.len(), alpha.len().pow(len as u32));
    res
}

/// Enumerates strings whose lengths are less than or equal to `len`.
pub fn enum_strs_len_leq<T>(alphabet: &[T], len: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut res = vec![];
    for i in 0..=len {
        res.extend(enum_strs_len_eq(alphabet, i));
    }
    res
}

#[test]
fn test_enum_strs_len() {
    // let alpha: Vec<BString> = alphabet(str2bstr("ab")).into_iter().collect();
    let alpha: Vec<u8> = alphabet_asc(str2bstr("ab"));
    let ss1 = enum_strs_len_eq(&alpha, 1);
    let ss2 = enum_strs_len_eq(&alpha, 2);
    let ss3 = enum_strs_len_eq(&alpha, 3);
    let ans1: Vec<BString> = ["a", "b"].iter().map(|x| str2bstring(x)).collect();
    let ans2: Vec<Vec<u8>> = ["aa", "ab", "ba", "bb"]
        .iter()
        .map(|x| str2bstring(x))
        .collect();
    let ans3: Vec<Vec<u8>> = ["aaa", "aab", "aba", "abb", "baa", "bab", "bba", "bbb"]
        .iter()
        .map(|x| str2bstring(x))
        .collect();
    println!("ans1 {:?}", ans1);
    println!("ss1 {:?}", ss1);
    println!("ans2 {:?}", ans2);
    println!("ss2 {:?}", ss2);
    println!("ans3 {:?}", ans3);
    println!("ss3 {:?}", ss3);
    assert_eq!(ans1, ss1);
    assert_eq!(ans2, ss2);
    assert_eq!(ans3, ss3);
}

/// Returns an index that, for a key of substring,
/// stores the number of occurrences of the key.
pub fn nocc(text: &bstr) -> HashMap<BString, usize> {
    let mut count: HashMap<BString, usize> = HashMap::new();
    for i in 0..text.len() {
        for j in i + 1..=text.len() {
            let entry = count.entry(bstr2bstring(&text[i..j])).or_insert(0);
            *entry += 1;
        }
    }
    count
}

#[test]
fn test_nocc() {
    let text = str2bstr("banana");
    let nocc = nocc(text);
    assert_eq!(1, *nocc.get(&str2bstring("b")).unwrap());
    assert_eq!(3, *nocc.get(&str2bstring("a")).unwrap());
    assert_eq!(2, *nocc.get(&str2bstring("an")).unwrap());
    assert_eq!(2, *nocc.get(&str2bstring("ana")).unwrap());
}

/// Returns a set of the beginning positions of a given string `x` in `w`.
pub fn beg_pos(w: &bstr, x: &bstr) -> HashSet<usize> {
    (0..w.len()).filter(|i| w[*i..].starts_with(x)).collect()
}

#[test]
fn test_beg_pos() {
    let text = str2bstr("cocoaccao");
    let pat = str2bstr("co");
    let bpos = beg_pos(text, pat);
    assert_eq!(bpos, HashSet::from([0, 2]));
}

/// Checks whether beginning positions of substrings `x` and `y` of `w` is equal.
pub fn eq_bpos_pres(x: &bstr, y: &bstr, pres: &HashSet<BString>) -> bool {
    trim_suf(pres, x) == trim_suf(pres, y)
}

/// Checks whether beginning positions of substrings `x` and `y` of `w` is equal.
pub fn eq_bpos(w: &bstr, x: &bstr, y: &bstr) -> bool {
    beg_pos(w, x) == beg_pos(w, y)
}

#[test]
fn test_eq_bpos() {
    let text = str2bstr("acocoaao");
    let x = str2bstr("co");
    let y = str2bstr("c");
    let z = str2bstr("aco");
    assert!(eq_bpos(text, x, y));
    assert!(!eq_bpos(text, x, z));
}

/// Returns a set of the ending positions of a given string `x` in `w`.
pub fn end_pos(w: &bstr, x: &bstr) -> HashSet<usize> {
    (0..w.len()).filter(|i| w[*i..].ends_with(x)).collect()
}

/// Checks whether ending positions of substrings `x` and `y` of `w` is equal.
pub fn eq_epos_suf(x: &bstr, y: &bstr, sufs: &HashSet<BString>) -> bool {
    trim_pre(sufs, x) == trim_pre(sufs, y)
}

/// Checks whether ending positions of substrings `x` and `y` of `w` is equal.
pub fn eq_epos(w: &bstr, x: &bstr, y: &bstr) -> bool {
    let ss = suf_set(w);
    eq_epos_suf(x, y, &ss)
}

#[test]
fn test_lr_eq() {
    let text = str2bstr("banana");
    let pres = pref_set(text);
    let sufs = suf_set(text);
    let ana = str2bstr("ana");
    let na = str2bstr("na");
    let an = str2bstr("an");
    let a = str2bstr("a");
    let b = str2bstr("b");
    let ban = str2bstr("ban");
    assert!(eq_bpos_pres(ana, an, &pres));
    assert!(!eq_bpos_pres(ana, a, &pres));
    assert!(!eq_bpos_pres(na, a, &pres));
    assert!(eq_bpos_pres(b, ban, &pres));
    // requal
    assert!(eq_epos_suf(ana, na, &sufs));
    assert!(!eq_epos_suf(ana, an, &sufs));
    assert!(!eq_epos_suf(ana, a, &sufs));
}

fn bpos_groups_(
    subs: &HashSet<BString>,
    pres: &HashSet<BString>,
) -> HashMap<BString, HashSet<BString>> {
    let mut beg_groups: HashMap<BString, HashSet<BString>> = HashMap::new();
    for x in subs {
        beg_groups.insert(x.clone(), [x.clone()].iter().cloned().collect());
    }
    for x in subs.iter() {
        for y in subs.iter() {
            if x != y && eq_bpos_pres(&x, &y, pres) {
                beg_groups.get_mut(x).unwrap().insert(y.clone());
                beg_groups.get_mut(y).unwrap().insert(x.clone());
            }
        }
    }
    beg_groups
}

/// Returns an index to beginning position groups
/// For a key of substring, the index stores a set of strings
/// whose beginning positions are the same to ones of the key.
pub fn bpos_groups(text: &bstr) -> HashMap<BString, HashSet<BString>> {
    let subs = substrs(text);
    let prefs = pref_set(text);
    bpos_groups_(&subs, &prefs)
}

#[test]
fn test_bpos_groups() {
    let text = str2bstr("cocoa");
    let bgroup = bpos_groups(text);
    assert_eq!(
        *bgroup.get(str2bstr("a")).unwrap(),
        HashSet::from(["a"].map(str2bstring))
    );
    assert_eq!(
        *bgroup.get(str2bstr("oco")).unwrap(),
        HashSet::from(["oc", "oco", "ocoa"].map(str2bstring))
    );
    assert_eq!(
        *bgroup.get(str2bstr("c")).unwrap(),
        HashSet::from(["c", "co"].map(str2bstring))
    );
    // for (k, v) in bgroup {
    //     println!("[{}]={}", bstr2string(&k), bstrs2string_set(&v));
    // }
}

fn epos_groups_(
    subs: &HashSet<BString>,
    sufs: &HashSet<BString>,
) -> HashMap<BString, HashSet<BString>> {
    let mut req_groups: HashMap<BString, HashSet<BString>> = HashMap::new();
    for x in subs.iter() {
        req_groups.insert(x.clone(), [x.clone()].iter().cloned().collect());
    }
    for x in subs.iter() {
        for y in subs.iter() {
            if x != y && eq_epos_suf(x, y, sufs) {
                req_groups.get_mut(x).unwrap().insert(y.clone());
                req_groups.get_mut(y).unwrap().insert(x.clone());
            }
        }
    }
    req_groups
}

/// Returns an index to ending position groups
/// For a key of substring, the index stores a set of strings
/// whose ending positions are the same to ones of the key.
pub fn epos_groups(text: &bstr) -> HashMap<BString, HashSet<BString>> {
    let subs = substrs(text);
    let sufs = suf_set(text);
    epos_groups_(&subs, &sufs)
}

#[test]
fn test_lr_eq_groups() {
    let text = str2bstr("banana");
    println!("{:?} {}", text, bstr2string(text));
    let egroup = epos_groups(text);
    // println!("{:?}", rg);
    // for (k, v) in rg.iter() {
    //     let ve: Vec<_> = v.iter().cloned().collect();
    //     println!("k={}, v={}", bstr2string(&k), bstrs2string(&ve));
    // }
    let ans_an: HashSet<_> = ["na", "ana"].iter().cloned().map(str2bstring).collect();
    let res_an = egroup.get(&str2bstring("na")).unwrap();
    assert_eq!(ans_an, *res_an);

    let bgroup = bpos_groups(text);
    let ans_na: HashSet<_> = ["an", "ana"].iter().cloned().map(str2bstring).collect();
    let res_na = bgroup.get(&str2bstring("an")).unwrap();
    assert_eq!(ans_na, *res_na);
}

/// Returns the longest substring in a set of left equal substrings for a given substring.
pub fn left_maximal(text: &bstr, egmap: &HashMap<BString, HashSet<BString>>) -> BString {
    egmap
        .get(text)
        .expect("has key")
        .iter()
        .max_by_key(|k| k.len())
        .unwrap()
        .clone()
}

/// Returns the longest substring in a set of right equal substrings for a given substring.
pub fn right_maximal(text: &bstr, bgmap: &HashMap<BString, HashSet<BString>>) -> BString {
    bgmap
        .get(text)
        .expect("has key")
        .iter()
        .max_by_key(|k| k.len())
        .unwrap()
        .clone()
}

#[test]
fn test_lr_maximal() {
    let text = str2bstr("banana$");
    let bgmap = bpos_groups(text);
    let egmap = epos_groups(text);
    let an = str2bstring("an");
    let na = str2bstring("na");
    let ans_an = str2bstring("ana");
    // for (k, v) in lgmap.iter() {
    for v in bgmap.get(&an).unwrap().iter() {
        println!("v={}", bstr2string(v));
    }
    // println!("{:?}", lgmap.get(&an).unwrap());
    let res_right_an = right_maximal(&an, &bgmap);
    assert_eq!(ans_an, res_right_an);
    // left
    let res_left_na = left_maximal(&na, &egmap);
    assert_eq!(ans_an, res_left_na);
}

// /// Returns an index that, for a key of substring,
// /// stores the longest substring of its right equal substrings.
// pub fn rmax_map(text: &bstr) -> HashMap<BString, BString> {
//     let subs = &substrs(text);
//     let rgmap = epos_groups_(subs, &suf_set(text));
//     subs.into_iter()
//         .map(|sub| (sub.clone(), right_maximal(&sub, &rgmap)))
//         .collect()
// }

/// Returns all qgrams which appears in a given string,
///
/// A qgram is a string of length q.
pub fn qgrams(text: &BString, q: usize) -> HashSet<BString> {
    (0..=(text.len() - q))
        .map(|i| text[i..i + q].to_vec())
        .collect()
}
