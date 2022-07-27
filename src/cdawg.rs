// use std::collections::HashSet;

// use crate::bstr::*;
// use crate::commons::*;
// use crate::stree_json;

// pub fn cdawg(text: &bstr) -> stree_json::StreeSerde {
//     let subs = substrs(text);
//     let lgmap = bpos_groups(text);
//     let rgmap = epos_groups(text);
//     let rm_req = |x: &Vec<_>| left_maximal(&right_maximal(&x, &lgmap), &rgmap);
//     let sigma = alphabet_set(text);

//     let vs: HashSet<BString> = subs.iter().cloned().map(|x| rm_req(&x)).collect();
//     let nodes = vs
//         .iter()
//         .map(|x| String::from_utf8(x.to_vec()).unwrap())
//         .collect();
//     let mut edges = vec![];

//     for v1 in vs {
//         let v1str = bstr2string(&v1);
//         for c in sigma.iter() {
//             // let v1cvec = [v1.to_vec(), c.to_vec()].concat();
//             let v1cvec = [v1.clone(), c.clone()].concat();
//             // let v1c = v1cvec.as_slice();
//             if subs.contains(&v1cvec) {
//                 let label = bstr2string(&right_maximal(&v1cvec, &lgmap)[v1.len()..]);
//                 let v2str = bstr2string(&left_maximal(&right_maximal(&v1cvec, &lgmap), &rgmap));
//                 edges.push((v1str.clone(), label, v2str));
//             }
//         }
//     }
//     stree_json::StreeSerde { nodes, edges }
// }
