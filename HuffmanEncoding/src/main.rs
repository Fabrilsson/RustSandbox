use std::env;
mod file_reader;
mod huffman;
mod tree_node;
mod binary_tree;

use std::fs::File;
use std::io::{Error, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let flag = &args[1];

    match flag.as_str() {
        "-e" => {
            let input_file_name = "files/war_and_peace.txt";
            let output_file_name = "files/war_and_peace.bin";

            let mut str_content = file_reader::read_file_to_string(input_file_name).unwrap();

            let tree_root = huffman::get_binary_tree_root_ref(&str_content);

            let str_encoded = huffman::encode_string(tree_root, &mut str_content);

            file_reader::write_bits_to_file(&str_encoded, output_file_name).unwrap()
        }
        "-d" => {}
        _ => {
            println!("Unknown argument: {}", flag);
            println!("Usage: {} <-e/-d> <input> <output>", args[0]);
        }
    }
}
