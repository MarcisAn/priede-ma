use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, StackValue, OPTCODE };

use super::parse_ast;

pub fn comp_s(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
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
    block.push(OPTCODE::LoadNumber { value: "0".to_string(), register: compiler.register_counter });
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
