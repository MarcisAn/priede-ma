extern crate celsium;
use std::{ collections::LinkedList, fs, process::{ self, Output } };

use celsium::{ bytecode::OPTCODE, bytecode_parser::parse_bytecode };

#[derive(Debug, Clone)]
pub enum MAOptcode {
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
        relative_target: isize,
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
    JumpTarget
}

pub struct StackValue {
    register: usize,
}

pub fn transpile(path: String) {
    let bytecode_str = read_file(path);
    let bytecode = parse_bytecode(bytecode_str);
    let mut ma_bytecode: Vec<MAOptcode> = vec![];
    let mut stack: LinkedList<StackValue> = LinkedList::new();
    let mut register_counter: usize = 0;
    for optcode in bytecode {
        process_optcode(optcode, &mut stack, &mut ma_bytecode, &mut register_counter);
    }
    for macode in ma_bytecode {
        println!("{:?}", macode);
    }
}

pub fn process_optcode(
    optcode: OPTCODE,
    stack: &mut LinkedList<StackValue>,
    output: &mut Vec<MAOptcode>,
    register_counter: &mut usize
) {
    match optcode {
        OPTCODE::LoadInt { value } => {
            output.push(MAOptcode::LoadNumber {
                value: value.to_string(),
                register: *register_counter,
            });
            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        }
        OPTCODE::LoadBool { value } => {
            output.push(MAOptcode::LoadNumber {
                value: match value {
                    true => "1".to_string(),
                    false => "0".to_string(),
                },
                register: *register_counter,
            });

            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        }
        OPTCODE::LoadString { value } => {
            output.push(MAOptcode::LoadString { value, register: *register_counter });
            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        }
        OPTCODE::LoadFloat { value } => {
            output.push(MAOptcode::LoadNumber {
                register: *register_counter,
                value: value.to_string(),
            });
            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        }
        OPTCODE::LoadVar { id } => {
            output.push(MAOptcode::GetVariable {
                name: id.to_string(),
                target_reg: *register_counter,
            });
            *register_counter += 1;
        }
        OPTCODE::CallFunction { name } => {
            output.push(match name.as_str() {
                "fixture" =>
                    MAOptcode::SelectFixture { id_register: stack.pop_back().unwrap().register },
                _ => panic!("Unexpected name of a function"),
            })
        }
        OPTCODE::CallFunctionWithBytecode { bytecode } => todo!(),
        OPTCODE::ReturnFromFunction => todo!(),
        OPTCODE::Add => {
            let a = stack.pop_back().unwrap().register;
            let b = stack.pop_back().unwrap().register;
            output.push(MAOptcode::Add { target_register: a, value_register: b });
            stack.push_back(StackValue { register: a });
        }
        OPTCODE::Subtract => {
            let b = stack.pop_back().unwrap();
            let a = stack.pop_back().unwrap();

            let a_register = match a {
                StackValue { register } => register,
                _ => panic!("addition with non-number"),
            };
            let b_register = match b {
                StackValue { register } => register,
                _ => panic!("addition with non-number"),
            };

            output.push(MAOptcode::LoadString {
                value: "\"-\"".to_string(),
                register: *register_counter,
            });
            output.push(MAOptcode::Add {
                target_register: *register_counter,
                value_register: b_register,
            });
            output.push(MAOptcode::Add {
                target_register: a_register,
                value_register: *register_counter,
            });

            stack.push_back(StackValue { register: a_register });
            *register_counter += 1;
        }
        OPTCODE::Multiply => todo!(),
        OPTCODE::Divide => todo!(),
        OPTCODE::Remainder => todo!(),
        OPTCODE::LessThan => {
            let a = stack.pop_back().unwrap().register;
            let b = stack.pop_back().unwrap().register;
            output.push(MAOptcode::LargerThan { target_reg: *register_counter, a_reg: a, b_reg: b });
            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        },
        OPTCODE::LargerThan => {
            let a = stack.pop_back().unwrap().register;
            let b = stack.pop_back().unwrap().register;
            output.push(MAOptcode::LargerThan { target_reg: *register_counter, a_reg: b, b_reg: a });
            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        },
        OPTCODE::LessOrEq =>{
            let a = stack.pop_back().unwrap().register;
            let b = stack.pop_back().unwrap().register;
            output.push(MAOptcode::LargerEq { target_reg: *register_counter, a_reg: b, b_reg: a });
            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        } ,
        OPTCODE::LargerOrEq => {
            let a = stack.pop_back().unwrap().register;
            let b = stack.pop_back().unwrap().register;
            output.push(MAOptcode::LargerEq { target_reg: *register_counter, a_reg: b, b_reg: a });
            stack.push_back(StackValue { register: *register_counter });
            *register_counter += 1;
        },
        OPTCODE::NotEq => todo!(),
        OPTCODE::Eq => todo!(),
        OPTCODE::Or => todo!(),
        OPTCODE::And => todo!(),
        OPTCODE::Xor => todo!(),
        OPTCODE::JumpIfFalse { steps } => {
            let register_to_check = stack.pop_back().unwrap().register;
            output.push(MAOptcode::JumpIfZero { register_to_check, line_to_jump_to: steps });
        }
        OPTCODE::Jump { steps } => {
            output.push(MAOptcode::Jump { relative_target: steps as isize });
        },
        OPTCODE::JumpBack { steps } => {
            output.push(MAOptcode::Jump { relative_target: -(steps as isize) });
        },
        OPTCODE::DefineVar { id } => {
            let value_reg = stack.pop_back().unwrap().register;
            output.push(MAOptcode::DefineVariable { name: id.to_string(), value_reg });
        },
        OPTCODE::DefineObject { id } => todo!(),
        OPTCODE::CreateObject { field_names } => todo!(),
        OPTCODE::GetObjectField { field_name } => todo!(),
        OPTCODE::DefineArray { id, init_values_count } => todo!(),
        OPTCODE::GetFromArray { id } => todo!(),
        OPTCODE::AssignAtArrayIndex { id } => todo!(),
        OPTCODE::PushToArray { id } => todo!(),
        OPTCODE::GettArrayLength { id } => todo!(),
        OPTCODE::AssignVar { id } => {
            let value_reg = stack.pop_back().unwrap().register;
            output.push(MAOptcode::DefineVariable { name: id.to_string(), value_reg });
        },
        OPTCODE::CallSpecialFunction { function } => {
            match function {
                celsium::SpecialFunctions::Print { newline } =>
                    output.push(MAOptcode::SelectFixture {
                        id_register: stack.pop_back().unwrap().register,
                    }),
                celsium::SpecialFunctions::Input => todo!(),
                celsium::SpecialFunctions::Random => todo!(),
            }
        }
        OPTCODE::SimpleLoop { body_block } => todo!(),
        OPTCODE::PushToTestingStack { duplicate_stackvalue } => {}
    }

    output.push(MAOptcode::JumpTarget);
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

/*use std::{ collections::LinkedList, f32::consts::E, fs, process };
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
        value: isize,
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
    pub fn while_loop(
        &mut self,
        main_block: &mut Vec<OPTCODE>,
        exp_block: &mut Vec<OPTCODE>,
        body_block: &mut Vec<OPTCODE>,
        exp_reg: Option<usize>
    ) {
        let jump_back_target = main_block.len();

        let to_jump_to = main_block.len() + body_block.len();
        let register_to_check = if exp_reg.is_none() {
            let conditional = self.stack.pop_back().unwrap();

            match conditional {
                crate::StackValue::NUM { register } => register,
                _ => panic!("addition with non-number"),
            }
        } else {
            exp_reg.unwrap()
        };
        main_block.append(exp_block);
        main_block.push(crate::OPTCODE::JumpIfZero {
            register_to_check,
            line_to_jump_to: to_jump_to + 3,
        });
        main_block.append(&mut body_block.clone());
        main_block.push(OPTCODE::Jump { line_to_jump_to: jump_back_target + 1 });
        main_block.push(crate::OPTCODE::EmptyLine);
    }
    pub fn if_stat(
        &mut self,
        body_block: &mut Vec<OPTCODE>,
        exp_block: &mut Vec<OPTCODE>,
        block: &mut Vec<OPTCODE>
    ) {
        let to_jump_to = block.len() + body_block.len();
        let conditional = self.stack.pop_back().unwrap();

        let register_to_check = match conditional {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(crate::OPTCODE::JumpIfZero {
            register_to_check,
            line_to_jump_to: to_jump_to + 2,
        });
        block.append(&mut body_block.clone());
        block.push(crate::OPTCODE::EmptyLine);
    }
}

pub fn compile(path: String) {
    let file_content = read_file(path.clone());

    let parse_res = hime::priede::parse_string(file_content.clone());
    let ast = parse_res.get_ast();
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
*/
