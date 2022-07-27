// use crate::bstr::*;
// use crate::commons::*;
// use crate::stree_json;

// pub fn dawg(text: &bstr) -> stree_json::StreeSerde {
//     let subs = substrs(text);
//     let rgmap = epos_groups(text);
//     let sigma = alphabet_set(text);

//     let rmaxs = rgmap
//         .values()
//         .cloned()
//         .map(|xs| xs.into_iter().max_by_key(|x| x.len()).unwrap())
//         .collect();
//     println!("rmaxs={:?}", bstrs2string_set(&rmaxs));
//     let nodes = rmaxs.iter().cloned().map(|x| bstr2string(&x)).collect();
//     let mut edges = vec![];

//     for v1 in rmaxs {
//         for c in &sigma {
//             // let v1cvec = [v1.clone(), c.clone()].concat();
//             let mut v1cvec = v1.clone();
//             v1cvec.push(*c);
//             let v1c = v1cvec.as_slice();
//             if subs.contains(v1c) {
//                 let v2 = left_maximal(&v1c.to_vec(), &rgmap);
//                 edges.push((bstr2string(&v1), [c], bstr2string(&v2)));
//             }
//         }
//     }
//     stree_json::StreeSerde { nodes, edges }
// }
