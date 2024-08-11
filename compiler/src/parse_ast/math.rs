
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, StackValue, OPTCODE };

use super::parse_ast;

pub fn math(title: &str, compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    if title == "plus" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        compiler.add(block);
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

        block.push(OPTCODE::LoadNumber { value: 0, register: compiler.register_counter });
        compiler.register_counter += 1;

        block.push(OPTCODE::DefineVariable {
            name: "is_negative".to_string(),
            value_reg: compiler.register_counter - 1,
        });

        let mut exp_block: Vec<OPTCODE> = vec![];
        let mut body_block: Vec<OPTCODE> = vec![];

        exp_block.push(OPTCODE::LargerEq {
            target_reg: compiler.register_counter,
            a_reg: compiler.register_counter - 1,
            b_reg: b_register,
        });
        compiler.register_counter += 1;
        body_block.push(OPTCODE::LoadNumber { value: 1, register: compiler.register_counter });
        compiler.register_counter += 1;

        body_block.push(OPTCODE::DefineVariable {
            name: "is_negative".to_string(),
            value_reg: compiler.register_counter - 1,
        });
        compiler.stack.push_back(crate::StackValue::NUM {
            register: compiler.register_counter,
        });
        let zero_register = compiler.register_counter;
        block.push(crate::OPTCODE::LoadNumber {
            register: compiler.register_counter,
            value: 0,
        });
        compiler.register_counter += 1;
        parse_ast(node.child(1), compiler, block);
        compiler.subtract(&mut body_block);

        compiler.if_stat(&mut body_block, &mut exp_block, block);

        let result_reg = compiler.register_counter;
        
        let counter_reg = compiler.register_counter;
        block.push(OPTCODE::LoadNumber { value: 0, register: compiler.register_counter });
        compiler.register_counter += 1;

        let mut exp_block: Vec<OPTCODE> = vec![];
        let mut body_block: Vec<OPTCODE> = vec![];

        exp_block.push(OPTCODE::LargerEq {
            target_reg: compiler.register_counter,
            a_reg: counter_reg,
            b_reg: b_register,
        });
        let exp_reg = compiler.register_counter;
        compiler.register_counter += 1;
        body_block.push(OPTCODE::LoadNumber { value: 1, register: compiler.register_counter });
        compiler.register_counter += 1;
        body_block.push(OPTCODE::Add { target_register: result_reg, value_register: a_register });
        body_block.push(OPTCODE::Add { target_register: counter_reg, value_register: compiler.register_counter - 1 });
        compiler.while_loop(block, &mut exp_block, &mut body_block, Some(exp_reg));
        compiler.stack.push_back(StackValue::NUM { register: result_reg });
    }
    if title == "minus" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        compiler.subtract(block);
    }
}
