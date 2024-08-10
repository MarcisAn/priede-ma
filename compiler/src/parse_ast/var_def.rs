use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn var_def(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
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
