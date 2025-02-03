#[derive(Debug, Clone)]
pub struct TreeNode {
    pub character: char,
    pub frequency: usize,
    pub left_node: Option<Box<TreeNode>>,
    pub right_node: Option<Box<TreeNode>>,
    pub binary_code: String
}

impl TreeNode {
    pub fn new_default(character: char, frequency: usize) -> TreeNode {
        TreeNode {character, frequency, left_node: None, right_node: None, binary_code: String::new()}
    }

    pub fn new_with_child_nodes(character: char, frequency: usize, left_node: Option<Box<TreeNode>>, right_node: Option<Box<TreeNode>>) -> TreeNode {
        TreeNode {character, frequency, left_node, right_node, binary_code: String::new()}
    }

    pub fn set_binary_code(&mut self, binary_code: String) {
        self.binary_code = binary_code
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

impl Eq for TreeNode {

}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}