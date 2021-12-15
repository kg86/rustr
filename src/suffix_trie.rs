use std::collections::HashSet;

use crate::bstr::*;
use crate::stree_json;

pub fn suffix_trie(sigma: &HashSet<&bstr>, subs: &HashSet<&bstr>) -> stree_json::StreeSerde {
    let nodes: Vec<String> = subs.iter().map(|&s| bstr2string(s)).collect();
    let mut edges: Vec<(String, String, String)> = Vec::new();
    for &v in subs.iter() {
        let vstr = String::from_utf8(v.to_vec()).expect("can be parsed.");
        for &c in sigma.iter() {
            let cstr = String::from_utf8(c.to_vec()).expect("can be parsed.");
            let vcstr = String::from_utf8([v, c].concat()).expect("can be parsed.");
            let child = vcstr.as_bytes();
            if subs.contains(child) {
                edges.push((vstr.clone(), cstr, vcstr));
            }
        }
    }
    stree_json::StreeSerde { nodes, edges }
}
