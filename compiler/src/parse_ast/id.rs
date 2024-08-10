use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn id(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    block.push(OPTCODE::GetVariable {
        name: node.get_value().unwrap().to_string(),
        target_reg: compiler.register_counter,
    });
    compiler.stack.push_back(crate::StackValue::NUM { register: compiler.register_counter });
    compiler.register_counter += 1;
}
