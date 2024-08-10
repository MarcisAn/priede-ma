use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, StackValue, OPTCODE };

pub fn parse_ast(node: AstNode, compiler: &mut Compiler, block: &mut Vec<OPTCODE>) {
    let title = node.get_symbol().to_string();
    if title == "block" {
        for child in node.children() {
            parse_ast(child, compiler, block);
        }
    }
    if title == "func_call" {
        for arg in node.child(1).children() {
            parse_ast(arg, compiler, block);
        }
        let func_name = node.child(0).get_value().unwrap();
        if func_name == "fixture" {
            let a = compiler.stack.pop_back().unwrap();
            let id_register = match a {
                crate::StackValue::NUM { register } => register,
                _ => panic!("addition with non-number"),
            };
            block.push(crate::OPTCODE::SelectFixture { id_register });
        }
    }
    if title == "comp_s" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(2), compiler, block);
        let a = compiler.stack.pop_back().unwrap();
        let b = compiler.stack.pop_back().unwrap();

        let a_register = match a {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let b_register = match b {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(OPTCODE::LoadNumber { value: 0, register: compiler.register_counter });
        println!("{}", node.child(1).get_symbol().name);
        if node.child(1).get_symbol().name == "=" {
            block.push(crate::OPTCODE::AreVarsEqual {
                target_reg: compiler.register_counter,
                a_reg: a_register,
                b_reg: b_register,
            });
        } else if node.child(1).get_symbol().name == ">" {
            block.push(crate::OPTCODE::LargerThan {
                target_reg: compiler.register_counter,
                a_reg: b_register,
                b_reg: a_register,
            });
        } else if node.child(1).get_symbol().name == "<" {
            block.push(crate::OPTCODE::LargerThan {
                target_reg: compiler.register_counter,
                a_reg: a_register,
                b_reg: b_register,
            });
        } else if node.child(1).get_symbol().name == ">=" {
            block.push(crate::OPTCODE::LargerEq {
                target_reg: compiler.register_counter,
                a_reg: b_register,
                b_reg: a_register,
            });
        } else if node.child(1).get_symbol().name == "<=" {
            block.push(crate::OPTCODE::LargerEq {
                target_reg: compiler.register_counter,
                a_reg: a_register,
                b_reg: b_register,
            });
        }
        compiler.stack.push_back(StackValue::NUM { register: compiler.register_counter });
        compiler.register_counter += 1;
    }
    if title == "plus" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        let a = compiler.stack.pop_back().unwrap();
        let b = compiler.stack.pop_back().unwrap();

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
        compiler.stack.push_back(crate::StackValue::NUM { register: a_register });
    }
    if title == "reiz" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        let a = compiler.stack.pop_back().unwrap();
        let b = compiler.stack.pop_back().unwrap();

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
        compiler.stack.push_back(crate::StackValue::NUM { register: a_register });
    }
    if title == "minus" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        let b = compiler.stack.pop_back().unwrap();
        let a = compiler.stack.pop_back().unwrap();

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
            register: compiler.register_counter,
        });
        block.push(crate::OPTCODE::Add {
            target_register: compiler.register_counter,
            value_register: b_register,
        });
        block.push(crate::OPTCODE::Add {
            target_register: a_register,
            value_register: compiler.register_counter,
        });

        compiler.stack.push_back(crate::StackValue::NUM { register: a_register });
        compiler.register_counter += 1;
    }

    if title == "var_def" {
        if node.child(0).get_value().unwrap() != "sk" {
            panic!("Not supported");
        }
        parse_ast(node.child(2), compiler, block);
        let value = compiler.stack.pop_back().unwrap();

        let value_register = match value {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(OPTCODE::DefineVariable {
            name: node.child(1).get_value().unwrap().to_string(),
            value_reg: value_register,
        });
    }
    if title == "ID" {
        block.push(OPTCODE::GetVariable {
            name: node.get_value().unwrap().to_string(),
            target_reg: compiler.register_counter,
        });
        compiler.stack.push_back(crate::StackValue::NUM { register: compiler.register_counter });
        compiler.register_counter += 1;
    }
    if title == "id_assign" {
        parse_ast(node.child(2), compiler, block);
        let value = compiler.stack.pop_back().unwrap();

        let value_register = match value {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let varname = node.child(0).get_value().unwrap().to_string();
        if node.child(1).get_symbol().name == ":" {
            block.push(OPTCODE::DefineVariable {
                name: varname,
                value_reg: value_register,
            });
        } else if node.child(1).get_symbol().name == "+:" {
            block.push(OPTCODE::GetVariable {
                name: varname,
                target_reg: compiler.register_counter,
            });
            block.push(OPTCODE::Add {
                target_register: compiler.register_counter,
                value_register: value_register,
            });
            block.push(OPTCODE::DefineVariable {
                name: node.child(0).get_value().unwrap().to_string(),
                value_reg: compiler.register_counter,
            });
        }
    }
    if title == "if" {
        let is_ifelse = node.children_count() > 2;
        if is_ifelse {
            parse_ast(node.child(0), compiler, block);
            let mut if_bytecode: Vec<OPTCODE> = vec![];
            let mut else_bytecode: Vec<OPTCODE> = vec![];

            parse_ast(node.child(1), compiler, &mut if_bytecode);
            parse_ast(node.child(3), compiler, &mut else_bytecode);

            let to_jump_to = if_bytecode.len() + block.len() + 1;
            let conditional = compiler.stack.pop_back().unwrap();
            let register_to_check = match conditional {
                crate::StackValue::NUM { register } => register,
                _ => panic!("addition with non-number"),
            };
            block.push(crate::OPTCODE::JumpIfZero {
                register_to_check,
                line_to_jump_to: to_jump_to + 2,
            });

            let to_jump_to_else = if_bytecode.len() + block.len() + else_bytecode.len() + 1;

            if_bytecode.push(crate::OPTCODE::Jump {
                line_to_jump_to: to_jump_to_else + 1,
            });
            block.append(&mut if_bytecode);
            block.append(&mut else_bytecode);
            block.push(crate::OPTCODE::EmptyLine);
        } else {
            parse_ast(node.child(0), compiler, block);
            let mut if_bytecode: Vec<OPTCODE> = vec![];

            parse_ast(node.child(1), compiler, &mut if_bytecode);
            let to_jump_to = block.len() + if_bytecode.len();
            let conditional = compiler.stack.pop_back().unwrap();

            let register_to_check = match conditional {
                crate::StackValue::NUM { register } => register,
                _ => panic!("addition with non-number"),
            };
            block.push(crate::OPTCODE::JumpIfZero {
                register_to_check,
                line_to_jump_to: to_jump_to + 2,
            });
            block.append(&mut if_bytecode.clone());
            block.push(crate::OPTCODE::EmptyLine);
        }
    }
    if title == "w_loop" {
        let jump_back_target = block.len();
        parse_ast(node.child(0), compiler, block);
        let mut if_bytecode: Vec<OPTCODE> = vec![];

        parse_ast(node.child(1), compiler, &mut if_bytecode);
        let to_jump_to = block.len() + if_bytecode.len();
        let conditional = compiler.stack.pop_back().unwrap();

        let register_to_check = match conditional {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(crate::OPTCODE::JumpIfZero {
            register_to_check,
            line_to_jump_to: to_jump_to + 3,
        });
        block.append(&mut if_bytecode.clone());
        block.push(OPTCODE::Jump { line_to_jump_to: jump_back_target + 1 });
        block.push(crate::OPTCODE::EmptyLine);
    }
    if title == "NUMBER" {
        let number: isize = node.get_value().unwrap().parse().unwrap();
        compiler.stack.push_back(crate::StackValue::NUM {
            register: compiler.register_counter,
        });
        block.push(crate::OPTCODE::LoadNumber {
            register: compiler.register_counter,
            value: number,
        });
        compiler.register_counter += 1;
    }
}
