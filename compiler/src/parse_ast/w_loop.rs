use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn w_loop(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    let mut exp_block: Vec<OPTCODE> = vec![];
    parse_ast(node.child(0), compiler, &mut exp_block);
    let mut body_bytecode: Vec<OPTCODE> = vec![];
    parse_ast(node.child(1), compiler, &mut body_bytecode);
    compiler.while_loop(block, &mut exp_block, &mut body_bytecode, None);
}
