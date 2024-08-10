use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

pub fn number(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
    let number: isize = node.get_value().unwrap().parse().unwrap();
    compiler.stack.push_back(crate::StackValue::NUM {
        register: compiler.register_counter,
    });
    block.push(crate::OPTCODE::LoadNumber {
        register: compiler.register_counter,
        value: number,
    });
    compiler.register_counter += 1;
}
