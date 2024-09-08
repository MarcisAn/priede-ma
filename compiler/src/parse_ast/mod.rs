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
        _ => panic!("Unrecognized symbol: {}", title)
    }
}
