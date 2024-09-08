use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

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


        block.push(crate::OPTCODE::DefineVariable { name: "multA".to_string(), value_reg: a_register });
        block.push(crate::OPTCODE::DefineVariable { name: "multB".to_string(), value_reg: b_register });
    }
    if title == "minus" {
        parse_ast(node.child(0), compiler, block);
        parse_ast(node.child(1), compiler, block);
        compiler.subtract(block);
    }
}
