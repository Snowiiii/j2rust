use std::{env, path::Path};

use parser::{convert::method::convert_method, nodes::method::NodeMethod};

mod java_std;
mod lexer;
mod parser;
mod prelude;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file");
    }
    let input_file = Path::new(&args[1]);
    if !input_file.exists() {
        panic!("Input file does not exist");
    }
    println!("Using Input file {}", input_file.display());

    let tokens = lexer::read_file(input_file);
    // for token in tokens {
    //     println!("{}", token)
    // }
    let nodes = parser::parse_tokens(tokens).unwrap();
    for node in nodes {
        match node {
            parser::Node::Class(node_class) => {},
            parser::Node::Method(node_method) => {
                let header = convert_method(node_method);
                println!("{}", header)
            },
        }
    }
}
