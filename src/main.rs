use std::{env, fs::File, io::Write, path::Path, process::Command};

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
    let nodes = parser::parse_tokens(&tokens).unwrap();
    let mut final_code = vec![];
    for node in nodes {
        match node {
            parser::Node::Class(node_class) => {}
            parser::Node::Method(node_method) => {
                let code = node_method.get_full_code();
                println!("{}", code);
                final_code.push(code);
            }
        }
    }
    let output_file = input_file.with_extension("rs");
    let mut file = File::create(&output_file).expect("Failed to create output file");
    for line in final_code {
        file.write(line.as_bytes()).unwrap();
    }
    let _ = Command::new("rustfmt").arg(output_file).output();
}
