// use std::collections::HashSet;

// use crate::bstr::*;
// use crate::commons::*;
// use crate::stree_json;

// pub fn suffix_tree(text: &bstr) -> stree_json::StreeSerde {
//     let subs = substrs(text);
//     let lgmap = bpos_groups(text);
//     let rmaxs: HashSet<BString> = subs
//         .iter()
//         .cloned()
//         .map(|sub| right_maximal(&sub, &lgmap))
//         .collect();
//     let nodes = rmaxs.iter().cloned().map(|s| bstr2string(&s)).collect();
//     let alph = alphabet_set(text);
//     let mut edges = vec![];

//     for v1 in rmaxs {
//         let v1str = bstr2string(&v1);
//         for c in alph.iter() {
//             let v2str =
//                 String::from_utf8([v1.clone(), c.clone()].concat()).expect("can be parsed.");
//             if subs.contains(v2str.as_bytes()) {
//                 let cstr = bstr2string(&c);
//                 // let child = right_maximal(v2str.as_bytes(), &lgmap);
//                 let child = right_maximal(&str2bstring(&v2str), &lgmap);
//                 let v3str = String::from_utf8(child.to_vec()).unwrap();
//                 edges.push((v1str.clone(), cstr, v3str));
//             }
//         }
//     }
//     stree_json::StreeSerde { nodes, edges }
// }
