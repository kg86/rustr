// use std::collections::HashMap;

// #[derive(Debug, PartialEq, Eq)]
// pub enum BinaryTree<T> {
//     Nil,
//     Node {
//         val: T,
//         left: Box<BinaryTree<T>>,
//         right: Box<BinaryTree<T>>,
//     },
// }

// impl<T> BinaryTree<T> {
//     pub fn replace(&mut self, to: Self) {
//         *self = to;
//     }
// }

// #[macro_export]
// macro_rules! bin_tree {
//     (val: $val:expr, left: $left:expr, right: $right:expr $(,)?) => {
//         BinaryTree::Node {
//             val: $val,
//             left: Box::new($left),
//             right: Box::new($right),
//         }
//     };
//     (val: $val:expr,right: $right:expr $(,)?) => {
//         bin_tree! {
//             val:$val,
//             left:bin_tree!(),
//             right: $right,
//         }
//     };
//     (val:$val:expr, left: $left:expr $(,)?) => {
//         bin_tree! {
//             val:$val,
//             left:$left,
//             right : bin_tree!()
//         }
//     };
//     (val:$val:expr $(,)?) => {
//         bin_tree!(val: $val, left: bin_tree!(), right: bin_tree!(),)
//     };
//     () => {
//         BinaryTree::Nil
//     };
// }

// struct Node2 {
//     in_edge: Vec<u8>,
//     children: HashMap<u8, usize>,
// }
// impl Node2 {
//     fn new(in_edge: Vec<u8>) -> Node2 {
//         Node2 {
//             in_edge: in_edge,
//             children: HashMap::new(),
//         }
//     }
//     fn copy(self: &Self) -> Self {
//         Node2 {
//             in_edge: self.in_edge.clone(),
//             children: HashMap::new(),
//         }
//     }
// }
// struct Trie2 {
//     // root: Node,
//     nodes: Vec<Node2>,
// }
// impl Trie2 {
//     fn new() -> Self {
//         let mut nodes = vec![Node2 {
//             // in_edge: "hoge".to_string().into_bytes(),
//             in_edge: "hoge".as_bytes().to_vec(),
//             children: HashMap::new(),
//         }];
//         Trie2 { nodes: nodes }
//     }
//     fn create_node(&mut self, in_edge: Vec<u8>) -> &Node2 {
//         self.nodes.push(Node2::new(in_edge));
//         &self.nodes[self.nodes.len() - 1]
//     }
//     // fn set_parent(&mut self, par: &mut Node, child: &Node) {
//     //     par.children
//     //         .insert(child.in_edge[0], Box::new(child.copy()));
//     // }
//     fn set_parent(&mut self, par_id: usize, child_id: usize) {
//         // let child = &self.nodes[child_id];
//         let key = self.nodes[child_id].in_edge[0];
//         let par = &mut self.nodes[par_id];
//         par.children.insert(key, child_id);
//     }
//     fn set_parent2(&self, par: &mut Node2, child_id: usize) {
//         let child = &self.nodes[child_id];
//         let key = self.nodes[child_id].in_edge[0];
//         // let par = &mut self.nodes[par_id];
//         par.children.insert(key, child_id);
//     }
//     fn add(&mut self, key: &[u8]) {
//         let leaf = Node2::new("".as_bytes().to_vec());
//         self.set_parent(0, self.nodes.len());
//         // self.nodes[0].children.insert(key[0], Box::new(leaf));
//     }
//     // fn estr(&self, node: &Node) -> String {
//     //     let s = "hoge";
//     //     String::from_utf8(node.in_edge.clone()).unwrap()
//     // }
//     // fn estr(&self, node: &Node) -> &Vec<u8> {
//     //     &node.in_edge
//     // }
//     fn estr(&self, nid: usize) -> &Vec<u8> {
//         &self.nodes[nid].in_edge
//     }
// }
// struct Node {
//     in_edge: Vec<u8>,
//     children: HashMap<u8, Box<Node>>,
// }
// impl Node {
//     fn new(in_edge: Vec<u8>) -> Node {
//         Node {
//             in_edge: in_edge,
//             children: HashMap::new(),
//         }
//     }
//     fn copy(self: &Self) -> Self {
//         Node {
//             in_edge: self.in_edge.clone(),
//             children: HashMap::new(),
//         }
//     }
// }
// struct Trie {
//     // root: Node,
//     nodes: Vec<Node>,
// }
// impl Trie {
//     fn new() -> Trie {
//         let mut nodes = vec![Node {
//             // in_edge: "hoge".to_string().into_bytes(),
//             in_edge: "hoge".as_bytes().to_vec(),
//             children: HashMap::new(),
//         }];
//         Trie { nodes: nodes }
//     }
//     fn create_node(&mut self, in_edge: Vec<u8>) -> &Node {
//         self.nodes.push(Node::new(in_edge));
//         &self.nodes[self.nodes.len() - 1]
//     }
//     // fn set_parent(&mut self, par: &mut Node, child: &Node) {
//     //     par.children
//     //         .insert(child.in_edge[0], Box::new(child.copy()));
//     // }
//     fn set_parent(&mut self, par: &mut Node, child: Box<Node>) {
//         par.children.insert(child.in_edge[0], child);
//     }
//     fn add(&mut self, key: &[u8]) {
//         let leaf = Node::new("".as_bytes().to_vec());
//         self.nodes[0].children.insert(key[0], Box::new(leaf));
//     }
//     // fn estr(&self, node: &Node) -> String {
//     //     let s = "hoge";
//     //     String::from_utf8(node.in_edge.clone()).unwrap()
//     // }
//     // fn estr(&self, node: &Node) -> &Vec<u8> {
//     //     &node.in_edge
//     // }
//     fn estr2<'a>(&self, node: &'a Node) -> &'a Vec<u8> {
//         &node.in_edge
//     }
// }
// #[derive(Default)]
// struct SomeOptions {
//     foo: i32,
//     bar: f32,
// }

// fn main() {
//     println!("Hellow, world!");
//     // let s = "aiueo".bytes();
//     let ss = "banana$".as_bytes();
//     let mut trie = Trie::new();
//     trie.add(ss);
//     let options: SomeOptions = Default::default();
//     println!("{}", options.bar);
// }
