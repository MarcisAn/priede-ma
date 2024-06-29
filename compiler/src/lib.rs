use std::{ collections::LinkedList, fs, process };

use hime_redist::ast::AstNode;
mod parse_ast;
mod hime;

#[derive(Debug, Clone)]
pub enum StackValue {
    NUM {
        register: usize,
    },
    STRING {
        register: usize,
    },
}

#[derive(Debug, Clone)]
pub enum OPTCODE {
    LoadNumber {
        value: isize,
        register: usize,
    },
    LoadString {
        value: String,
        register: usize,
    },
    Add {
        target_register: usize,
        value_register: usize,
    },
    PrintFunction {
        register: usize,
    },
    JumpIfZero {
        register_to_check: usize,
        line_to_jump_to: usize,
    },
    EmptyLine,
}

#[derive(Debug, Clone)]
pub struct Compiler {
    register_counter: usize,
    stack: LinkedList<StackValue>,
    bytecode: Vec<OPTCODE>,
}

impl Compiler {
    pub fn new() -> Compiler {
        return Compiler {
            register_counter: 0,
            stack: LinkedList::new(),
            bytecode: vec![],
        };
    }
}

pub fn compile(path: String) {
    let file_content = read_file(path.clone());

    let parse_res = hime::priede::parse_string(file_content.clone());
    let ast = parse_res.get_ast();
    let root = ast.get_root();
    print_ast(root);
    let mut compiler = Compiler::new();
    parse_ast::parse_ast(root, &mut compiler);

    for optcode in &compiler.bytecode {
        println!("{:?}", optcode);
    }

    let xmlfile = format_xml_file(compiler.bytecode);
    println!("{}", xmlfile);
    let _ = fs::write(
        "C:/ProgramData/MA Lighting Technologies/grandma/gma2_V_3.9.60/macros/priede.xml",
        xmlfile
    );
    //C:\ProgramData\MA Lighting Technologies\grandma\gma2_V_3.9.60\macros
}

fn format_xml_file(bytecode: Vec<OPTCODE>) -> String {
    let mut macro_lines = "".to_string();

    let mut counter = 0;
    for optcode in bytecode {
        macro_lines += &format_macro_line(optcode, counter);
        macro_lines += "\n";
        counter += 1;
    }

    format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>
<MA xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns=\"http://schemas.malighting.de/grandma2/xml/MA\" xsi:schemaLocation=\"http://schemas.malighting.de/grandma2/xml/MA http://schemas.malighting.de/grandma2/xml/3.1.58/MA.xsd\" major_vers=\"3\" minor_vers=\"1\" stream_vers=\"58\">
	<Info datetime=\"2015-05-22T14:08:42\" showfile=\"new show 2015-05-22\" />
	<Macro index=\"0'\" name=\"macrotest\">
    {}
	</Macro>

</MA>
    ", macro_lines)
}
//<MacroLine Guid="A5 21 19 8F ED 18 04 09 69 4A 63 3E 9D 49 BD 7C" Command="Group 1" />
fn format_macro_line(optcode: OPTCODE, lineid: usize) -> String {
    let command = match optcode {
        OPTCODE::LoadNumber { value, register } =>
            format!("SetVar ${} = {}", "reg_".to_string() + &register.to_string(), value),
        OPTCODE::EmptyLine => "".to_string(),
        OPTCODE::LoadString { value, register } =>
            format!("SetVar ${} = {}", "reg_".to_string() + &register.to_string(), value),
        OPTCODE::Add { target_register, value_register } =>
            format!(
                "AddVar ${} = ${}",
                "reg_".to_string() + &target_register.to_string(),
                "reg_".to_string() + &value_register.to_string()
            ),
        OPTCODE::PrintFunction { register } => todo!(),
        OPTCODE::JumpIfZero { register_to_check, line_to_jump_to } =>
            format!(
                "[${} == 0]  Macro 1.1.{}",
                "reg_".to_string() + &register_to_check.to_string(),
                line_to_jump_to
            ),
    };
    format!("<Macroline index=\"{}\" delay=\"0\">
			<text>{}</text>
		</Macroline>", lineid, command)
}

pub fn read_file(path: String) -> String {
    let file_read = fs::read_to_string(&path);
    if file_read.is_err() {
        println!("{}", file_read.err().unwrap());
        println!("Neizdevās nolasīt failu {} \nPārlicinies, ka faila adrese ir pareiza!", path);
        process::exit(1);
    }
    file_read.unwrap()
}
fn print<'a>(node: AstNode, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{:}", node);
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), child_crossings);
        i += 1;
    }
}
pub fn print_ast(node: AstNode) {
    print(node, Vec::<bool>::new());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        compile("../examples/test.pr".to_string());
    }
}
