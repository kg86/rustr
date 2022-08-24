use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
};

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashSet<usize>,
    pub edges: HashSet<(usize, usize, String)>,
}

#[derive(PartialEq, Debug)]
enum NodeType {
    Root,
    Internal,
    Leaf,
}
struct Node {
    parent: Option<usize>,
    ebeg: usize,
    elen: usize,
    children: HashMap<u8, usize>,
}

impl Node {
    fn new(parent: Option<usize>, ebeg: usize, elen: usize) -> Self {
        Self {
            parent,
            ebeg,
            elen,
            children: HashMap::new(),
        }
    }

    fn node_type(&self) -> NodeType {
        if self.parent == None {
            NodeType::Root
        } else if self.elen > 0 {
            NodeType::Internal
        } else {
            NodeType::Leaf
        }
    }
}

struct ImplicitNode {
    nid: usize,
    match_len: usize,
    edge_len: usize,
}
impl ImplicitNode {
    const fn root() -> Self {
        Self {
            nid: 0,
            match_len: 0,
            edge_len: 0,
        }
    }
    const fn at_node(&self) -> bool {
        self.edge_len == self.match_len
    }
}

pub struct Tree {
    text: Vec<u8>,
    ap: ImplicitNode,
    nodes: Vec<Node>,
    slink: HashMap<usize, usize>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            text: vec![],
            ap: ImplicitNode::root(),
            nodes: vec![Node::new(None, 0, 0)],
            slink: HashMap::new(),
        }
    }

    fn create_node(&mut self, ebeg: usize, elen: usize) -> usize {
        self.nodes.push(Node::new(None, ebeg, elen));
        self.nodes.len() - 1
    }

    fn set_par_child(&mut self, pid: usize, cid: usize) {
        let k = self.text[self.nodes[cid].ebeg];
        self.nodes[cid].parent = Some(pid);
        self.nodes[pid].children.insert(k, cid);
    }

    fn split_at(&mut self, cid: usize, match_len: usize) -> usize {
        assert_ne!(self.nodes[cid].node_type(), NodeType::Root);
        let pid = self.nodes[cid].parent.unwrap();
        let bid = self.create_node(self.nodes[cid].ebeg, match_len);
        self.nodes[cid].ebeg += match_len;
        if self.nodes[cid].node_type() == NodeType::Internal {
            self.nodes[cid].elen -= match_len;
        }
        self.set_par_child(pid, bid);
        self.set_par_child(bid, cid);
        bid
    }

    // move from `ap` with `c`.
    fn move_ap(&self, ap: &ImplicitNode, c: u8) -> Option<ImplicitNode> {
        if ap.at_node() {
            self.nodes[ap.nid]
                .children
                .get(&c)
                .map(|&cid| ImplicitNode {
                    nid: cid,
                    match_len: 1,
                    edge_len: self.nodes[cid].elen,
                })
        } else if self.text[self.nodes[ap.nid].ebeg + ap.match_len] == c {
            Some(ImplicitNode {
                nid: ap.nid,
                match_len: ap.match_len + 1,
                edge_len: ap.edge_len,
            })
        } else {
            None
        }
    }

    // move from node `nid` with key `key`.
    // it is guaranteed that we can move with key `key`.
    fn move_trust(&self, nid: usize, key: &[u8]) -> ImplicitNode {
        if key.is_empty() {
            assert_ne!(self.nodes[nid].node_type(), NodeType::Leaf);
            ImplicitNode {
                nid,
                match_len: 0,
                edge_len: self.nodes[nid].elen,
            }
        } else {
            match self.nodes[nid].children.get(&key[0]) {
                Some(&cid) => {
                    if self.elen(cid) <= key.len() {
                        self.move_trust(cid, &key[self.nodes[cid].elen..])
                    } else {
                        ImplicitNode {
                            nid: cid,
                            match_len: key.len(),
                            edge_len: self.nodes[cid].elen,
                        }
                    }
                }
                None => panic!(
                    "move_trust must move from node[{:?}] with key[{:?}]",
                    nid, key
                ),
            }
        }
    }

    pub fn add_suffix(&mut self, c: u8) {
        println!("add {}", std::str::from_utf8(&[c]).unwrap());
        self.text.push(c);
        let mut prev_node: Option<usize> = None;
        loop {
            if let Some(ap_next) = self.move_ap(&self.ap, c) {
                if self.ap.at_node() {
                    if let Some(prev_id) = prev_node {
                        self.slink.insert(prev_id, self.ap.nid);
                    }
                }
                self.ap = ap_next;
                break;
            } else {
                // add new leaf from bid to cid
                let bid = if self.ap.at_node() {
                    self.ap.nid
                } else {
                    self.split_at(self.ap.nid, self.ap.match_len)
                };
                let cid = self.create_node(self.text.len() - 1, 0);
                self.set_par_child(bid, cid);
                println!("insert {} to {}", bid, cid);

                // update suffix link
                if let Some(prev_id) = prev_node {
                    self.slink.insert(prev_id, bid);
                }
                prev_node = Some(bid);

                // update ap, where bid is root or internal node.
                if self.nodes[bid].node_type() == NodeType::Root {
                    // there is no suffixes to be inserted.
                    break;
                } else {
                    // node `bid` is an internal node
                    let key = &self.text
                        [self.nodes[bid].ebeg..self.nodes[bid].ebeg + self.nodes[bid].elen];
                    let pid = self.nodes[bid].parent.unwrap();
                    self.ap = if self.nodes[pid].node_type() == NodeType::Root {
                        self.move_trust(pid, &key[1..])
                    } else {
                        self.move_trust(*self.slink.get(&pid).unwrap(), key)
                    }
                }
            }
        }

        println!("graph: {:?}", self.graph());
        // println!("graph: {:?}", self.serialize());
        println!();
    }

    // return edge lentgth of node `nid`.
    fn elen(&self, nid: usize) -> usize {
        let node = &self.nodes[nid];
        if node.node_type() == NodeType::Root {
            0
        } else if node.elen == 0 {
            self.text.len() - node.ebeg
        } else {
            node.elen
        }
    }

    fn estr(&self, nid: usize) -> String {
        let ebeg = self.nodes[nid].ebeg;
        let eend = ebeg + self.elen(nid);
        String::from_utf8(self.text[ebeg..eend].to_vec()).unwrap()
    }

    fn estr_all(&self, nid: usize) -> String {
        match self.nodes[nid].parent {
            Some(pid) => self.estr_all(pid) + &self.estr(nid),
            None => self.estr(nid),
        }
    }

    pub fn graph(&self) -> Graph {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            nodes.insert(i);
            if let Some(pid) = node.parent {
                edges.insert((pid, i, self.estr(i)));
            }
        }
        Graph { nodes, edges }
    }

    pub fn serialize(&self) -> String {
        let mut nodes = vec![];
        let mut edges = vec![];
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            nodes.push(self.estr_all(i));
            if let Some(pid) = node.parent {
                edges.push((self.estr_all(pid), self.estr(i), self.estr_all(i)));
            }
        }
        let s = crate::stree_json::StreeSerde { nodes, edges };
        serde_json::to_string(&s).unwrap()
    }

    pub fn dump(&self, fpath: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(fpath)?;
        file.write(self.serialize().as_bytes())
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}
