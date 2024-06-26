use std::{ collections::LinkedList, fs, process };

use hime_redist::ast::AstNode;
mod parse_ast;
mod hime;

pub enum StackValue {
    NUM {
        register: usize,
    },
    STRING {
        register: usize,
    },
}

#[derive(Debug)]
pub enum OPTCODE {
    LoadNumber {
        value: isize,
        register: usize,
    },
    LoadString {
        value: String,
        register: usize,
    },
    Add {
        target_register: usize,
        a_register: usize,
        b_register: usize,
    },
    PrintFunction {
        register: usize,
    },
}

pub struct Compiler {
    register_counter: usize,
    stack: LinkedList<StackValue>,
    bytecode: Vec<OPTCODE>,
}

impl Compiler {
    pub fn new() -> Compiler {
        return Compiler {
            register_counter: 0,
            stack: LinkedList::new(),
            bytecode: vec![],
        };
    }
}

pub fn compile(path: String) {
    let file_content = read_file(path.clone());

    let parse_res = hime::priede::parse_string(file_content.clone());
    let ast = parse_res.get_ast();
    let root = ast.get_root();
    print_ast(root);
    let mut compiler = Compiler::new();
    parse_ast::parse_ast(root, &mut compiler);

    for optcode in compiler.bytecode {
        println!("{:?}", optcode);
    }
}

pub fn read_file(path: String) -> String {
    let file_read = fs::read_to_string(&path);
    if file_read.is_err() {
        println!("{}", file_read.err().unwrap());
        println!("Neizdevās nolasīt failu {} \nPārlicinies, ka faila adrese ir pareiza!", path);
        process::exit(1);
    }
    file_read.unwrap()
}
fn print<'a>(node: AstNode, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{:}", node);
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), child_crossings);
        i += 1;
    }
}
pub fn print_ast(node: AstNode) {
    print(node, Vec::<bool>::new());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        compile("../examples/test.pr".to_string());
    }
}
