use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, StackValue };

pub fn parse_ast(node: AstNode, compiler: &mut Compiler) {
    let title = node.get_symbol().to_string();
    if title == "block" {
        for child in node.children() {
            parse_ast(child, compiler);
        }
    }
    if title == "func_call" {
        for arg in node.child(1).children() {
            parse_ast(arg, compiler);
        }
    }
    if title == "plus" {
        parse_ast(node.child(0), compiler);
        parse_ast(node.child(1), compiler);
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

        compiler.bytecode.push(crate::OPTCODE::Add {
            target_register: a_register,
            value_register: b_register,
        });
        compiler.stack.push_back(crate::StackValue::NUM { register: a_register });
    }
    if title == "minus" {
        parse_ast(node.child(0), compiler);
        parse_ast(node.child(1), compiler);
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

        compiler.bytecode.push(crate::OPTCODE::LoadString {
            value: "\"-\"".to_string(),
            register: compiler.register_counter,
        });
        compiler.bytecode.push(crate::OPTCODE::Add {
            target_register: compiler.register_counter,
            value_register: b_register,
        });
        compiler.bytecode.push(crate::OPTCODE::Add {
            target_register: a_register,
            value_register: compiler.register_counter,
        });

        compiler.stack.push_back(crate::StackValue::NUM { register: a_register });
        compiler.register_counter += 1;
    }
    if title == "if" {
        parse_ast(node.child(0), compiler);
        let mut cloned_compiler = compiler.clone();
        cloned_compiler.bytecode = vec![];
        parse_ast(node.child(1), &mut cloned_compiler);
        let to_jump_to = cloned_compiler.bytecode.len() + compiler.bytecode.len();
        let conditional = compiler.stack.pop_back().unwrap();

        let register_to_check = match conditional {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        compiler.bytecode.push(crate::OPTCODE::JumpIfZero { register_to_check, line_to_jump_to: to_jump_to + 2 });
        compiler.bytecode.append(&mut cloned_compiler.bytecode);
        compiler.bytecode.push(crate::OPTCODE::EmptyLine);

    }
    if title == "NUMBER" {
        let number: isize = node.get_value().unwrap().parse().unwrap();
        compiler.stack.push_back(crate::StackValue::NUM {
            register: compiler.register_counter,
        });
        compiler.bytecode.push(crate::OPTCODE::LoadNumber {
            register: compiler.register_counter,
            value: number,
        });
        compiler.register_counter += 1;
    }
}
