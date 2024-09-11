use std::{ collections::LinkedList, fs, process };
mod xml_output;
use xml_output::format_xml_file;
use hime_redist::ast::AstNode;
mod parse_ast;
use parse_ast::parse_ast;
mod hime;

#[derive(Debug, Clone)]
pub enum StackValue {
    NUM {
        register: usize,
    },
    STRING {
        register: usize,
    },
}

#[derive(Debug, Clone)]
pub enum OPTCODE {
    LoadNumber {
        value: String,
        register: usize,
    },
    LoadString {
        value: String,
        register: usize,
    },
    Add {
        target_register: usize,
        value_register: usize,
    },
    PrintFunction {
        register: usize,
    },
    JumpIfZero {
        register_to_check: usize,
        line_to_jump_to: usize,
    },
    Jump {
        line_to_jump_to: usize,
    },
    EmptyLine,
    SelectFixture {
        id_register: usize,
    },
    AreVarsEqual {
        target_reg: usize,
        a_reg: usize,
        b_reg: usize,
    },
    LargerThan {
        target_reg: usize,
        a_reg: usize,
        b_reg: usize,
    },
    LargerEq {
        target_reg: usize,
        a_reg: usize,
        b_reg: usize,
    },
    DefineVariable {
        name: String,
        value_reg: usize,
    },
    GetVariable {
        name: String,
        target_reg: usize,
    },
    CallMacro {
        number: usize
    },
    DimmerFull,
    DimmerZero,
    ColorPreset{number: usize}
}

#[derive(Debug, Clone)]
pub struct Compiler {
    register_counter: usize,
    stack: LinkedList<StackValue>,
}

impl Compiler {
    pub fn new() -> Compiler {
        return Compiler {
            register_counter: 0,
            stack: LinkedList::new(),
        };
    }
    pub fn add(&mut self, block: &mut Vec<OPTCODE>) {
        let a = self.stack.pop_back().unwrap();
        let b = self.stack.pop_back().unwrap();

        let a_register = match a {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let b_register = match b {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };

        block.push(crate::OPTCODE::Add {
            target_register: a_register,
            value_register: b_register,
        });
        self.stack.push_back(crate::StackValue::NUM { register: a_register });
    }
    pub fn subtract(&mut self, block: &mut Vec<OPTCODE>) {
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();

        let a_register = match a {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let b_register = match b {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };

        block.push(crate::OPTCODE::LoadString {
            value: "\"-\"".to_string(),
            register: self.register_counter,
        });
        block.push(crate::OPTCODE::Add {
            target_register: self.register_counter,
            value_register: b_register,
        });
        block.push(crate::OPTCODE::Add {
            target_register: a_register,
            value_register: self.register_counter,
        });

        self.stack.push_back(crate::StackValue::NUM { register: a_register });
        self.register_counter += 1;
    }
}

pub fn compile(path: String) {
    let file_content = read_file(path.clone());

    let parse_res = hime::priede::parse_string(file_content.clone());
    let ast = parse_res.get_ast();
    print!("{:?}", parse_res.errors.errors);
    let root = ast.get_root();
    print_ast(root);
    let mut compiler = Compiler::new();
    let mut main_block: Vec<OPTCODE> = vec![];
    parse_ast(root, &mut compiler, &mut main_block);

    for optcode in &main_block {
        println!("{:?}", optcode);
    }

    let xmlfile = format_xml_file(main_block);
    //println!("{}", xmlfile);
    let _ = fs::write(
        "C:/ProgramData/MA Lighting Technologies/grandma/gma2_V_3.9.60/macros/priede.xml",
        xmlfile
    );
    //C:\ProgramData\MA Lighting Technologies\grandma\gma2_V_3.9.60\macros
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
