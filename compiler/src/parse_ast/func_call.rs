use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, StackValue, OPTCODE };

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
    } else if func_name == "mint" {
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
        block.push(crate::OPTCODE::DefineVariable {
            name: "multA".to_string(),
            value_reg: a_register,
        });
        block.push(crate::OPTCODE::DefineVariable {
            name: "multB".to_string(),
            value_reg: b_register,
        });
        block.push(crate::OPTCODE::CallMacro { number: 2 });

        block.push(OPTCODE::GetVariable {
            name: "multres".to_string(),
            target_reg: compiler.register_counter,
        });
        compiler.stack.push_back(crate::StackValue::NUM { register: compiler.register_counter });
        compiler.register_counter += 1;
    }
    else if func_name == "fulldim" {
        block.push(OPTCODE::DimmerFull)
    }
    else if func_name == "zerodim" {
        block.push(OPTCODE::DimmerZero)
    }
    else if func_name == "color"{
        block.push(OPTCODE::ColorPreset { number: node.child(1).child(0).get_value().unwrap().parse::<usize>().unwrap() })
    }
}
