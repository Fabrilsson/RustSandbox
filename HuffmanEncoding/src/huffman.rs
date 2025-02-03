use std::sync::Mutex;
use crate::tree_node::TreeNode;
use rayon::prelude::*;

const SPECIAL_CHAR: char = '\0';

pub fn get_binary_tree_root_ref(content: &String) -> Option<Box<TreeNode>> {
    let mut tree_nodes = get_tree_nodes_vec(&content);

    let tree_head_node = get_huff_tree_head(&mut tree_nodes);

    let mut tree_head_ref = Some(Box::new(tree_head_node));

    create_tree_binary_codes(&mut tree_head_ref, &mut String::new());

    tree_head_ref
}

pub fn encode_string(head_tree_node: Option<Box<TreeNode>>, str_content: &mut String) -> String {
    let mut ret_vec : Vec<Box<TreeNode>> = Vec::new();

    get_tree_vec(head_tree_node, &mut ret_vec);

    let mut ret_string = str_content.clone();

    ret_vec.iter().for_each(|n| {
        ret_string = ret_string.replacen(n.character, &n.binary_code, n.frequency);
    });

    ret_string
}

fn get_tree_vec(node: Option<Box<TreeNode>>, ret_vec: &mut Vec<Box<TreeNode>>) {
    match node {
        Some(inner) => {
            if inner.character != SPECIAL_CHAR {
                ret_vec.push(inner.clone());
            }

            get_tree_vec(inner.left_node, ret_vec);
            get_tree_vec(inner.right_node, ret_vec);
        }
        None => {
        }
    }
}

fn get_tree_nodes_vec(content: &&String) -> Vec<TreeNode> {
    let mut ret_vec = Mutex::new(Vec::<TreeNode>::new());
    let mut char_vec: Vec<char> = content.chars().collect();

    char_vec.into_par_iter().for_each(|c| {
        let mut vec = ret_vec.lock().unwrap();

        if let Some(node) = vec.iter_mut().find(|n| n.character == c) {
            node.frequency += 1;
        }
        else {
            vec.push(TreeNode::new_default(c, 1));
        }
    });

    let mut ret_vec = ret_vec.into_inner().unwrap();

    ret_vec.sort();

    ret_vec
}

fn get_huff_tree_head(nodes: &mut Vec<TreeNode>) -> TreeNode {
    while nodes.len() > 1{
        let new_freq = nodes[0].frequency + nodes[1].frequency;

        let left_node = Some(Box::new(nodes[0].clone()));
        let right_node = Some(Box::new(nodes[1].clone()));

        let new_node= TreeNode::new_with_child_nodes(SPECIAL_CHAR, new_freq, left_node, right_node);

        nodes.drain(0..2);

        nodes.push(new_node);

        nodes.sort();
    }

    nodes[0].clone()
}

fn create_tree_binary_codes(head_node: &mut Option<Box<TreeNode>>, current_code: &mut String) {
    match head_node {
        Some(inside) => {
            current_code.push('0');
            create_tree_binary_codes(&mut inside.left_node, current_code);
            inside.set_binary_code(current_code.to_string());

            current_code.push('1');
            create_tree_binary_codes(&mut inside.right_node, current_code);
            inside.set_binary_code(current_code.to_string());

            current_code.pop();
        }
        None => {
            current_code.pop();
        }
    }
}