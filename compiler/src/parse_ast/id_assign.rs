use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, OPTCODE };

use super::parse_ast;

pub fn id_assign(compiler: &mut Compiler, node: AstNode, block: &mut Vec<OPTCODE>) {
parse_ast(node.child(2), compiler, block);
        let value = compiler.stack.pop_back().unwrap();

        let value_register = match value {
            crate::StackValue::NUM { register } => register,
            _ => panic!("addition with non-number"),
        };
        let varname = node.child(0).get_value().unwrap().to_string();
        if node.child(1).get_symbol().name == ":" {
            block.push(OPTCODE::DefineVariable {
                name: varname,
                value_reg: value_register,
            });
        } else if node.child(1).get_symbol().name == "+:" {
            block.push(OPTCODE::GetVariable {
                name: varname,
                target_reg: compiler.register_counter,
            });
            block.push(OPTCODE::Add {
                target_register: compiler.register_counter,
                value_register: value_register,
            });
            block.push(OPTCODE::DefineVariable {
                name: node.child(0).get_value().unwrap().to_string(),
                value_reg: compiler.register_counter,
            });
        }
}
