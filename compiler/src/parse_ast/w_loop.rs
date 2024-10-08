use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn w_loop(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    let jump_back_target = block.len();
    parse_ast(node.child(0), compiler, block);
    let conditional = compiler.stack.pop_back().unwrap();
    let starting_length = block.len();

    let mut body: Vec<OPTCODE> = vec![];
    parse_ast(node.child(1), compiler, &mut body);
    let register_to_check = match conditional {
        crate::StackValue::NUM { register } => register,
        _ => panic!("addition with non-number"),
    };
    block.push(crate::OPTCODE::JumpIfZero {
        register_to_check,
        line_to_jump_to: body.len() + 3,
    });
    block.append(&mut body);

    block.push(OPTCODE::Jump { line_to_jump_to: jump_back_target + 1 });
    block.push(crate::OPTCODE::EmptyLine);
}
