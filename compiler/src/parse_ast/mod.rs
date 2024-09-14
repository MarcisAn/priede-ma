use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
mod func_call;
use func_call::func_call;
mod comp_s;
use comp_s::comp_s;
mod math;
use math::math;
mod var_def;
use var_def::var_def;
mod id;
use id::id;
mod id_assign;
use id_assign::id_assign;
mod if_stat;
use if_stat::if_stat;
mod w_loop;
use w_loop::w_loop;
mod number;
use number::number;

use crate::{ Compiler, StackValue, OPTCODE };

pub fn parse_ast(node: AstNode, compiler: &mut Compiler, block: &mut Vec<OPTCODE>) {
    let binding = node.get_symbol().to_string();
    let title = binding.as_str();

    if title == "block" {
        for child in node.children() {
            parse_ast(child, compiler, block);
        }
    }
    if title == "array_def" {
        let name = node.child(2).get_value().unwrap().to_string();
        let items = node.child(3).children();
        for (index, item) in items.into_iter().enumerate() {
            parse_ast(item, compiler, block);
            let register = match compiler.stack.pop_back().unwrap() {
                crate::StackValue::NUM { register } => register,
                _ => panic!("addition with non-number"),
            };
            block.push(OPTCODE::DefineVariable {
                name: format!("array_{}_{}", name, index),
                value_reg: register,
            });
        }
    }
    if title == "array" {
        let name = node.child(0).get_value().unwrap();
        let index = node.child(1).get_value().unwrap().parse::<usize>().unwrap();
        block.push(OPTCODE::GetVariable {
            name: format!("array_{}_{}", name, index),
            target_reg: compiler.register_counter,
        });
        compiler.stack.push_back(crate::StackValue::NUM { register: compiler.register_counter });
        compiler.register_counter += 1;
    }
    match title {
        "func_call" => func_call(compiler, node, block),
        "comp_s" => comp_s(compiler, node, block),
        "plus" => math(title, compiler, node, block),
        "minus" => math(title, compiler, node, block),
        "reiz" => math(title, compiler, node, block),
        "dal" => math(title, compiler, node, block),
        "var_def" => var_def(compiler, node, block),
        "ID" => id(compiler, node, block),
        "id_assign" => id_assign(compiler, node, block),
        "if" => if_stat(compiler, node, block),
        "w_loop" => w_loop(compiler, node, block),
        "NUMBER" => number(compiler, node, block),
        "block" => (),
        "array_def" => (),
        "array" => (),
        _ => panic!("Unrecognized symbol: {}", title),
    }
}
