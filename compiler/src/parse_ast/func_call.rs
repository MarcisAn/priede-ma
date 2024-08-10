use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use crate::{Compiler, OPTCODE};

use super::parse_ast;

pub fn func_call(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
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
