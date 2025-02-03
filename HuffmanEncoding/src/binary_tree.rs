use std::collections::BTreeMap;

#[derive(Clone)]
pub struct BinaryTree{
    pub data_map: BTreeMap<char,String>
}

impl BinaryTree {
    pub fn new_default() -> BinaryTree{
        let tree: BTreeMap<char, String> = BTreeMap::new();
        BinaryTree { data_map: tree }
    }
}