use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn if_stat(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
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

        let conditional = compiler.stack.pop_back().unwrap();
        let mut if_bytecode: Vec<OPTCODE> = vec![];

        parse_ast(node.child(1), compiler, &mut if_bytecode);
        let to_jump_to = if_bytecode.len();
        let register_to_check = match conditional {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        block.push(crate::OPTCODE::JumpIfZero {
            register_to_check,
            line_to_jump_to: to_jump_to + 2,
        });
        block.append(&mut if_bytecode.clone());
        println!("{:?}\n{:?}", if_bytecode, block);
        block.push(crate::OPTCODE::EmptyLine);
    }
}

/*
 */
