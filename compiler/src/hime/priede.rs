#[allow(unused_variables)]
#[warn(dead_code)]

use std::io::Read;

use hime_redist::ast::AstNode;
use hime_redist::errors::ParseErrors;
use hime_redist::lexers::automaton::Automaton;
use hime_redist::lexers::impls::ContextFreeLexer;
use hime_redist::lexers::Lexer;
use hime_redist::parsers::lrk::LRkAutomaton;
use hime_redist::parsers::lrk::LRkParser;
use hime_redist::parsers::Parser;
use hime_redist::result::ParseResult;
use hime_redist::symbols::SemanticBody;
use hime_redist::symbols::SemanticElementTrait;
use hime_redist::symbols::Symbol;
use hime_redist::text::Text;
use hime_redist::tokens::TokenRepository;

/// Static resource for the serialized lexer automaton
const LEXER_AUTOMATON: &[u8] = include_bytes!("priede_lexer.bin");

/// The unique identifier for terminal `SEPARATOR`
pub const ID_TERMINAL_SEPARATOR: u32 = 0x0007;
/// The unique identifier for terminal `ID`
pub const ID_TERMINAL_ID: u32 = 0x0008;
/// The unique identifier for terminal `QUOTE`
pub const ID_TERMINAL_QUOTE: u32 = 0x0009;
/// The unique identifier for terminal `NUMBER`
pub const ID_TERMINAL_NUMBER: u32 = 0x000B;
/// The unique identifier for terminal `String`
pub const ID_TERMINAL_String: u32 = 0x000C;
/// The unique identifier for terminal `CITADI`
pub const ID_TERMINAL_CITADI: u32 = 0x000D;
/// The unique identifier for terminal `ARRAY`
pub const ID_TERMINAL_ARRAY: u32 = 0x000E;
/// The unique identifier for terminal `RETURN`
pub const ID_TERMINAL_RETURN: u32 = 0x000F;
/// The unique identifier for terminal `INCLUDE`
pub const ID_TERMINAL_INCLUDE: u32 = 0x0010;
/// The unique identifier for terminal `FROM`
pub const ID_TERMINAL_FROM: u32 = 0x0011;
/// The unique identifier for terminal `EXPORT`
pub const ID_TERMINAL_EXPORT: u32 = 0x0012;
/// The unique identifier for terminal `Object`
pub const ID_TERMINAL_Object: u32 = 0x0013;

/// The unique identifier for the default context
pub const CONTEXT_DEFAULT: u16 = 0;

/// The collection of terminals matched by this lexer
/// The terminals are in an order consistent with the automaton,
/// so that terminal indices in the automaton can be used to retrieve the terminals in this table
pub const TERMINALS: &[Symbol] = &[
    Symbol {
        id: 0x0001,
        name: "ε"
    },
    Symbol {
        id: 0x0002,
        name: "$"
    },
    Symbol {
        id: 0x0007,
        name: "SEPARATOR"
    },
    Symbol {
        id: 0x0008,
        name: "ID"
    },
    Symbol {
        id: 0x0009,
        name: "QUOTE"
    },
    Symbol {
        id: 0x000B,
        name: "NUMBER"
    },
    Symbol {
        id: 0x000C,
        name: "String"
    },
    Symbol {
        id: 0x000D,
        name: "CITADI"
    },
    Symbol {
        id: 0x000E,
        name: "ARRAY"
    },
    Symbol {
        id: 0x000F,
        name: "RETURN"
    },
    Symbol {
        id: 0x0010,
        name: "INCLUDE"
    },
    Symbol {
        id: 0x0011,
        name: "FROM"
    },
    Symbol {
        id: 0x0012,
        name: "EXPORT"
    },
    Symbol {
        id: 0x0013,
        name: "Object"
    },
    Symbol {
        id: 0x004B,
        name: "PAT"
    },
    Symbol {
        id: 0x004C,
        name: "PATIESS"
    },
    Symbol {
        id: 0x004D,
        name: "NEPAT"
    },
    Symbol {
        id: 0x004E,
        name: "NEPATIESS"
    },
    Symbol {
        id: 0x004F,
        name: "["
    },
    Symbol {
        id: 0x0050,
        name: "]"
    },
    Symbol {
        id: 0x0051,
        name: "{"
    },
    Symbol {
        id: 0x0053,
        name: "}"
    },
    Symbol {
        id: 0x0054,
        name: ":"
    },
    Symbol {
        id: 0x0055,
        name: ";"
    },
    Symbol {
        id: 0x0057,
        name: "+:"
    },
    Symbol {
        id: 0x0058,
        name: "-:"
    },
    Symbol {
        id: 0x0059,
        name: "*:"
    },
    Symbol {
        id: 0x005A,
        name: "/:"
    },
    Symbol {
        id: 0x005B,
        name: "++"
    },
    Symbol {
        id: 0x005C,
        name: "--"
    },
    Symbol {
        id: 0x005D,
        name: "("
    },
    Symbol {
        id: 0x005E,
        name: ")"
    },
    Symbol {
        id: 0x005F,
        name: "()"
    },
    Symbol {
        id: 0x0061,
        name: "="
    },
    Symbol {
        id: 0x0062,
        name: ">"
    },
    Symbol {
        id: 0x0063,
        name: ">="
    },
    Symbol {
        id: 0x0064,
        name: "<"
    },
    Symbol {
        id: 0x0065,
        name: "<="
    },
    Symbol {
        id: 0x0066,
        name: "!="
    },
    Symbol {
        id: 0x0067,
        name: "ja"
    },
    Symbol {
        id: 0x0068,
        name: "atkārtot"
    },
    Symbol {
        id: 0x0069,
        name: "kamēr"
    },
    Symbol {
        id: 0x006B,
        name: "funkc"
    },
    Symbol {
        id: 0x006C,
        name: "->"
    },
    Symbol {
        id: 0x006D,
        name: "."
    },
    Symbol {
        id: 0x006E,
        name: "*"
    },
    Symbol {
        id: 0x006F,
        name: "/"
    },
    Symbol {
        id: 0x0070,
        name: "%"
    },
    Symbol {
        id: 0x0071,
        name: "+"
    },
    Symbol {
        id: 0x0072,
        name: "-"
    },
    Symbol {
        id: 0x0073,
        name: "xvai"
    },
    Symbol {
        id: 0x0074,
        name: "vai"
    },
    Symbol {
        id: 0x0076,
        name: "un"
    }
];

/// Creates a new lexer
fn new_lexer<'a: 'b, 'b, 'c>(
    repository: TokenRepository<'a, 'b, 'c>,
    errors: &'c mut ParseErrors<'a>
) -> Lexer<'a, 'b, 'c> {
    let automaton = Automaton::new(LEXER_AUTOMATON);
    Lexer::ContextFree(ContextFreeLexer::new(repository, errors, automaton, 0x0007))
}

/// Static resource for the serialized parser automaton
const PARSER_AUTOMATON: &[u8] = include_bytes!("priede_parser.bin");

/// The unique identifier for variable `TRUE`
pub const ID_VARIABLE_TRUE: u32 = 0x0014;
/// The unique identifier for variable `FALSE`
pub const ID_VARIABLE_FALSE: u32 = 0x0015;
/// The unique identifier for variable `Bool`
pub const ID_VARIABLE_Bool: u32 = 0x0016;
/// The unique identifier for variable `array`
pub const ID_VARIABLE_ARRAY: u32 = 0x0017;
/// The unique identifier for variable `include_list`
pub const ID_VARIABLE_INCLUDE_LIST: u32 = 0x0018;
/// The unique identifier for variable `include`
pub const ID_VARIABLE_INCLUDE: u32 = 0x0019;
/// The unique identifier for variable `array_def`
pub const ID_VARIABLE_ARRAY_DEF: u32 = 0x001A;
/// The unique identifier for variable `var_def`
pub const ID_VARIABLE_VAR_DEF: u32 = 0x001B;
/// The unique identifier for variable `array_def_value`
pub const ID_VARIABLE_ARRAY_DEF_VALUE: u32 = 0x001C;
/// The unique identifier for variable `var_def_value`
pub const ID_VARIABLE_VAR_DEF_VALUE: u32 = 0x001D;
/// The unique identifier for variable `asignable_exp`
pub const ID_VARIABLE_ASIGNABLE_EXP: u32 = 0x001E;
/// The unique identifier for variable `assign_op`
pub const ID_VARIABLE_ASSIGN_OP: u32 = 0x001F;
/// The unique identifier for variable `id_assign`
pub const ID_VARIABLE_ID_ASSIGN: u32 = 0x0020;
/// The unique identifier for variable `array_assign`
pub const ID_VARIABLE_ARRAY_ASSIGN: u32 = 0x0021;
/// The unique identifier for variable `func_name`
pub const ID_VARIABLE_FUNC_NAME: u32 = 0x0022;
/// The unique identifier for variable `func_call`
pub const ID_VARIABLE_FUNC_CALL: u32 = 0x0023;
/// The unique identifier for variable `funcarg`
pub const ID_VARIABLE_FUNCARG: u32 = 0x0024;
/// The unique identifier for variable `funcargs`
pub const ID_VARIABLE_FUNCARGS: u32 = 0x0025;
/// The unique identifier for variable `comp_s`
pub const ID_VARIABLE_COMP_S: u32 = 0x0026;
/// The unique identifier for variable `if`
pub const ID_VARIABLE_IF: u32 = 0x0027;
/// The unique identifier for variable `s_loop`
pub const ID_VARIABLE_S_LOOP: u32 = 0x0028;
/// The unique identifier for variable `w_loop`
pub const ID_VARIABLE_W_LOOP: u32 = 0x0029;
/// The unique identifier for variable `arg`
pub const ID_VARIABLE_ARG: u32 = 0x002A;
/// The unique identifier for variable `func_def_args`
pub const ID_VARIABLE_FUNC_DEF_ARGS: u32 = 0x002B;
/// The unique identifier for variable `func_return_type`
pub const ID_VARIABLE_FUNC_RETURN_TYPE: u32 = 0x002C;
/// The unique identifier for variable `func_def`
pub const ID_VARIABLE_FUNC_DEF: u32 = 0x002D;
/// The unique identifier for variable `return_st`
pub const ID_VARIABLE_RETURN_ST: u32 = 0x002E;
/// The unique identifier for variable `dotable`
pub const ID_VARIABLE_DOTABLE: u32 = 0x002F;
/// The unique identifier for variable `exp_atom_no_dot`
pub const ID_VARIABLE_EXP_ATOM_NO_DOT: u32 = 0x0030;
/// The unique identifier for variable `exp_atom`
pub const ID_VARIABLE_EXP_ATOM: u32 = 0x0031;
/// The unique identifier for variable `dot_call`
pub const ID_VARIABLE_DOT_CALL: u32 = 0x0032;
/// The unique identifier for variable `dot_call_fn`
pub const ID_VARIABLE_DOT_CALL_FN: u32 = 0x0033;
/// The unique identifier for variable `exp_reizdal`
pub const ID_VARIABLE_EXP_REIZDAL: u32 = 0x0034;
/// The unique identifier for variable `reiz_dal_atl`
pub const ID_VARIABLE_REIZ_DAL_ATL: u32 = 0x0035;
/// The unique identifier for variable `reiz`
pub const ID_VARIABLE_REIZ: u32 = 0x0036;
/// The unique identifier for variable `dal`
pub const ID_VARIABLE_DAL: u32 = 0x0037;
/// The unique identifier for variable `atlik`
pub const ID_VARIABLE_ATLIK: u32 = 0x0038;
/// The unique identifier for variable `exp_plusmin`
pub const ID_VARIABLE_EXP_PLUSMIN: u32 = 0x0039;
/// The unique identifier for variable `plus`
pub const ID_VARIABLE_PLUS: u32 = 0x003A;
/// The unique identifier for variable `minus`
pub const ID_VARIABLE_MINUS: u32 = 0x003B;
/// The unique identifier for variable `exp_t`
pub const ID_VARIABLE_EXP_T: u32 = 0x003C;
/// The unique identifier for variable `exp_a`
pub const ID_VARIABLE_EXP_A: u32 = 0x003D;
/// The unique identifier for variable `xvai`
pub const ID_VARIABLE_XVAI: u32 = 0x003E;
/// The unique identifier for variable `vai`
pub const ID_VARIABLE_VAI: u32 = 0x003F;
/// The unique identifier for variable `un`
pub const ID_VARIABLE_UN: u32 = 0x0040;
/// The unique identifier for variable `exp`
pub const ID_VARIABLE_EXP: u32 = 0x0041;
/// The unique identifier for variable `multiple_ids`
pub const ID_VARIABLE_MULTIPLE_IDS: u32 = 0x0042;
/// The unique identifier for variable `multiple_id_define`
pub const ID_VARIABLE_MULTIPLE_ID_DEFINE: u32 = 0x0043;
/// The unique identifier for variable `object_field`
pub const ID_VARIABLE_Object_FIELD: u32 = 0x0044;
/// The unique identifier for variable `object_def_field`
pub const ID_VARIABLE_Object_DEF_FIELD: u32 = 0x0045;
/// The unique identifier for variable `object`
pub const ID_VARIABLE_Object: u32 = 0x0046;
/// The unique identifier for variable `object_def`
pub const ID_VARIABLE_Object_DEF: u32 = 0x0047;
/// The unique identifier for variable `stat`
pub const ID_VARIABLE_STAT: u32 = 0x0048;
/// The unique identifier for variable `block`
pub const ID_VARIABLE_BLOCK: u32 = 0x0049;
/// The unique identifier for variable `root`
pub const ID_VARIABLE_ROOT: u32 = 0x004A;


/// The collection of variables matched by this parser
/// The variables are in an order consistent with the automaton,
/// so that variable indices in the automaton can be used to retrieve the variables in this table
pub const VARIABLES: &[Symbol] = &[
    Symbol {
        id: 0x0014,
        name: "TRUE"
    },
    Symbol {
        id: 0x0015,
        name: "FALSE"
    },
    Symbol {
        id: 0x0016,
        name: "Bool"
    },
    Symbol {
        id: 0x0017,
        name: "array"
    },
    Symbol {
        id: 0x0018,
        name: "include_list"
    },
    Symbol {
        id: 0x0019,
        name: "include"
    },
    Symbol {
        id: 0x001A,
        name: "array_def"
    },
    Symbol {
        id: 0x001B,
        name: "var_def"
    },
    Symbol {
        id: 0x001C,
        name: "array_def_value"
    },
    Symbol {
        id: 0x001D,
        name: "var_def_value"
    },
    Symbol {
        id: 0x001E,
        name: "asignable_exp"
    },
    Symbol {
        id: 0x001F,
        name: "assign_op"
    },
    Symbol {
        id: 0x0020,
        name: "id_assign"
    },
    Symbol {
        id: 0x0021,
        name: "array_assign"
    },
    Symbol {
        id: 0x0022,
        name: "func_name"
    },
    Symbol {
        id: 0x0023,
        name: "func_call"
    },
    Symbol {
        id: 0x0024,
        name: "funcarg"
    },
    Symbol {
        id: 0x0025,
        name: "funcargs"
    },
    Symbol {
        id: 0x0026,
        name: "comp_s"
    },
    Symbol {
        id: 0x0027,
        name: "if"
    },
    Symbol {
        id: 0x0028,
        name: "s_loop"
    },
    Symbol {
        id: 0x0029,
        name: "w_loop"
    },
    Symbol {
        id: 0x002A,
        name: "arg"
    },
    Symbol {
        id: 0x002B,
        name: "func_def_args"
    },
    Symbol {
        id: 0x002C,
        name: "func_return_type"
    },
    Symbol {
        id: 0x002D,
        name: "func_def"
    },
    Symbol {
        id: 0x002E,
        name: "return_st"
    },
    Symbol {
        id: 0x002F,
        name: "dotable"
    },
    Symbol {
        id: 0x0030,
        name: "exp_atom_no_dot"
    },
    Symbol {
        id: 0x0031,
        name: "exp_atom"
    },
    Symbol {
        id: 0x0032,
        name: "dot_call"
    },
    Symbol {
        id: 0x0033,
        name: "dot_call_fn"
    },
    Symbol {
        id: 0x0034,
        name: "exp_reizdal"
    },
    Symbol {
        id: 0x0035,
        name: "reiz_dal_atl"
    },
    Symbol {
        id: 0x0036,
        name: "reiz"
    },
    Symbol {
        id: 0x0037,
        name: "dal"
    },
    Symbol {
        id: 0x0038,
        name: "atlik"
    },
    Symbol {
        id: 0x0039,
        name: "exp_plusmin"
    },
    Symbol {
        id: 0x003A,
        name: "plus"
    },
    Symbol {
        id: 0x003B,
        name: "minus"
    },
    Symbol {
        id: 0x003C,
        name: "exp_t"
    },
    Symbol {
        id: 0x003D,
        name: "exp_a"
    },
    Symbol {
        id: 0x003E,
        name: "xvai"
    },
    Symbol {
        id: 0x003F,
        name: "vai"
    },
    Symbol {
        id: 0x0040,
        name: "un"
    },
    Symbol {
        id: 0x0041,
        name: "exp"
    },
    Symbol {
        id: 0x0042,
        name: "multiple_ids"
    },
    Symbol {
        id: 0x0043,
        name: "multiple_id_define"
    },
    Symbol {
        id: 0x0044,
        name: "object_field"
    },
    Symbol {
        id: 0x0045,
        name: "object_def_field"
    },
    Symbol {
        id: 0x0046,
        name: "object"
    },
    Symbol {
        id: 0x0047,
        name: "object_def"
    },
    Symbol {
        id: 0x0048,
        name: "stat"
    },
    Symbol {
        id: 0x0049,
        name: "block"
    },
    Symbol {
        id: 0x004A,
        name: "root"
    },
    Symbol {
        id: 0x0052,
        name: "__V82"
    },
    Symbol {
        id: 0x0056,
        name: "__V86"
    },
    Symbol {
        id: 0x0060,
        name: "__V96"
    },
    Symbol {
        id: 0x006A,
        name: "__V106"
    },
    Symbol {
        id: 0x0075,
        name: "__V117"
    },
    Symbol {
        id: 0x0077,
        name: "__V119"
    },
    Symbol {
        id: 0x0078,
        name: "__V120"
    },
    Symbol {
        id: 0x0079,
        name: "__V121"
    },
    Symbol {
        id: 0x007A,
        name: "__V122"
    },
    Symbol {
        id: 0x007B,
        name: "__V123"
    },
    Symbol {
        id: 0x007C,
        name: "__VAxiom"
    }
];

/// The collection of virtuals matched by this parser
/// The virtuals are in an order consistent with the automaton,
/// so that virtual indices in the automaton can be used to retrieve the virtuals in this table
pub const VIRTUALS: &[Symbol] = &[

];

/// Parses the specified string with this parser
#[must_use]
pub fn parse_str(input: &str) -> ParseResult<'static, '_, 'static> {
    let text = Text::from_str(input);
    parse_text(text)
}

/// Parses the specified string with this parser
#[must_use]
pub fn parse_string(input: String) -> ParseResult<'static, 'static, 'static> {
    let text = Text::from_string(input);
    parse_text(text)
}

/// Parses the specified stream of UTF-8 with this parser
pub fn parse_utf8_stream(input: &mut dyn Read) -> ParseResult<'static, 'static, 'static> {
    let text = Text::from_utf8_stream(input).unwrap();
    parse_text(text)
}

/// Parses the specified text with this parser
fn parse_text(text: Text) -> ParseResult<'static, '_, 'static> {
    parse_text_with(text, TERMINALS, VARIABLES, VIRTUALS)
}

/// Parses the specified text with this parser
fn parse_text_with<'s, 't, 'a>(
    text: Text<'t>,
    terminals: &'a [Symbol<'s>],
    variables: &'a [Symbol<'s>],
    virtuals: &'a [Symbol<'s>]
) -> ParseResult<'s, 't, 'a> {
    let mut my_actions = |_index: usize, _head: Symbol, _body: &dyn SemanticBody| ();
    let mut result = ParseResult::new(terminals, variables, virtuals, text);
    {
        let data = result.get_parsing_data();
        let mut lexer = new_lexer(data.0, data.1);
        let automaton = LRkAutomaton::new(PARSER_AUTOMATON);
        let mut parser = LRkParser::new(&mut lexer, automaton, data.2, &mut my_actions);
        parser.parse();
    }
    result
}

/// Visitor interface
pub trait Visitor {
    fn on_terminal_separator(&self, _node: &AstNode) {}
    fn on_terminal_id(&self, _node: &AstNode) {}
    fn on_terminal_quote(&self, _node: &AstNode) {}
    fn on_terminal_number(&self, _node: &AstNode) {}
    fn on_terminal_string(&self, _node: &AstNode) {}
    fn on_terminal_citadi(&self, _node: &AstNode) {}
    fn on_terminal_array(&self, _node: &AstNode) {}
    fn on_terminal_return(&self, _node: &AstNode) {}
    fn on_terminal_include(&self, _node: &AstNode) {}
    fn on_terminal_from(&self, _node: &AstNode) {}
    fn on_terminal_export(&self, _node: &AstNode) {}
    fn on_terminal_object(&self, _node: &AstNode) {}
    fn on_variable_true(&self, _node: &AstNode) {}
    fn on_variable_false(&self, _node: &AstNode) {}
    fn on_variable_bool(&self, _node: &AstNode) {}
    fn on_variable_array(&self, _node: &AstNode) {}
    fn on_variable_include_list(&self, _node: &AstNode) {}
    fn on_variable_include(&self, _node: &AstNode) {}
    fn on_variable_array_def(&self, _node: &AstNode) {}
    fn on_variable_var_def(&self, _node: &AstNode) {}
    fn on_variable_array_def_value(&self, _node: &AstNode) {}
    fn on_variable_var_def_value(&self, _node: &AstNode) {}
    fn on_variable_asignable_exp(&self, _node: &AstNode) {}
    fn on_variable_assign_op(&self, _node: &AstNode) {}
    fn on_variable_id_assign(&self, _node: &AstNode) {}
    fn on_variable_array_assign(&self, _node: &AstNode) {}
    fn on_variable_func_name(&self, _node: &AstNode) {}
    fn on_variable_func_call(&self, _node: &AstNode) {}
    fn on_variable_funcarg(&self, _node: &AstNode) {}
    fn on_variable_funcargs(&self, _node: &AstNode) {}
    fn on_variable_comp_s(&self, _node: &AstNode) {}
    fn on_variable_if(&self, _node: &AstNode) {}
    fn on_variable_s_loop(&self, _node: &AstNode) {}
    fn on_variable_w_loop(&self, _node: &AstNode) {}
    fn on_variable_arg(&self, _node: &AstNode) {}
    fn on_variable_func_def_args(&self, _node: &AstNode) {}
    fn on_variable_func_return_type(&self, _node: &AstNode) {}
    fn on_variable_func_def(&self, _node: &AstNode) {}
    fn on_variable_return_st(&self, _node: &AstNode) {}
    fn on_variable_dotable(&self, _node: &AstNode) {}
    fn on_variable_exp_atom_no_dot(&self, _node: &AstNode) {}
    fn on_variable_exp_atom(&self, _node: &AstNode) {}
    fn on_variable_dot_call(&self, _node: &AstNode) {}
    fn on_variable_dot_call_fn(&self, _node: &AstNode) {}
    fn on_variable_exp_reizdal(&self, _node: &AstNode) {}
    fn on_variable_reiz_dal_atl(&self, _node: &AstNode) {}
    fn on_variable_reiz(&self, _node: &AstNode) {}
    fn on_variable_dal(&self, _node: &AstNode) {}
    fn on_variable_atlik(&self, _node: &AstNode) {}
    fn on_variable_exp_plusmin(&self, _node: &AstNode) {}
    fn on_variable_plus(&self, _node: &AstNode) {}
    fn on_variable_minus(&self, _node: &AstNode) {}
    fn on_variable_exp_t(&self, _node: &AstNode) {}
    fn on_variable_exp_a(&self, _node: &AstNode) {}
    fn on_variable_xvai(&self, _node: &AstNode) {}
    fn on_variable_vai(&self, _node: &AstNode) {}
    fn on_variable_un(&self, _node: &AstNode) {}
    fn on_variable_exp(&self, _node: &AstNode) {}
    fn on_variable_multiple_ids(&self, _node: &AstNode) {}
    fn on_variable_multiple_id_define(&self, _node: &AstNode) {}
    fn on_variable_object_field(&self, _node: &AstNode) {}
    fn on_variable_object_def_field(&self, _node: &AstNode) {}
    fn on_variable_object(&self, _node: &AstNode) {}
    fn on_variable_object_def(&self, _node: &AstNode) {}
    fn on_variable_stat(&self, _node: &AstNode) {}
    fn on_variable_block(&self, _node: &AstNode) {}
    fn on_variable_root(&self, _node: &AstNode) {}
}

/// Walk the AST of a result using a visitor
pub fn visit(result: &ParseResult, visitor: &dyn Visitor) {
    let ast = result.get_ast();
    let root = ast.get_root();
    visit_ast_node(root, visitor);
}

/// Walk the sub-AST from the specified node using a visitor
pub fn visit_ast_node(node: AstNode, visitor: &dyn Visitor) {
    let children = node.children();
    for child in children.iter() {
        visit_ast_node(child, visitor);
    }
    match node.get_symbol().id {
        0x0007 => visitor.on_terminal_separator(&node),
        0x0008 => visitor.on_terminal_id(&node),
        0x0009 => visitor.on_terminal_quote(&node),
        0x000B => visitor.on_terminal_number(&node),
        0x000C => visitor.on_terminal_string(&node),
        0x000D => visitor.on_terminal_citadi(&node),
        0x000E => visitor.on_terminal_array(&node),
        0x000F => visitor.on_terminal_return(&node),
        0x0010 => visitor.on_terminal_include(&node),
        0x0011 => visitor.on_terminal_from(&node),
        0x0012 => visitor.on_terminal_export(&node),
        0x0013 => visitor.on_terminal_object(&node),
        0x0014 => visitor.on_variable_true(&node),
        0x0015 => visitor.on_variable_false(&node),
        0x0016 => visitor.on_variable_bool(&node),
        0x0017 => visitor.on_variable_array(&node),
        0x0018 => visitor.on_variable_include_list(&node),
        0x0019 => visitor.on_variable_include(&node),
        0x001A => visitor.on_variable_array_def(&node),
        0x001B => visitor.on_variable_var_def(&node),
        0x001C => visitor.on_variable_array_def_value(&node),
        0x001D => visitor.on_variable_var_def_value(&node),
        0x001E => visitor.on_variable_asignable_exp(&node),
        0x001F => visitor.on_variable_assign_op(&node),
        0x0020 => visitor.on_variable_id_assign(&node),
        0x0021 => visitor.on_variable_array_assign(&node),
        0x0022 => visitor.on_variable_func_name(&node),
        0x0023 => visitor.on_variable_func_call(&node),
        0x0024 => visitor.on_variable_funcarg(&node),
        0x0025 => visitor.on_variable_funcargs(&node),
        0x0026 => visitor.on_variable_comp_s(&node),
        0x0027 => visitor.on_variable_if(&node),
        0x0028 => visitor.on_variable_s_loop(&node),
        0x0029 => visitor.on_variable_w_loop(&node),
        0x002A => visitor.on_variable_arg(&node),
        0x002B => visitor.on_variable_func_def_args(&node),
        0x002C => visitor.on_variable_func_return_type(&node),
        0x002D => visitor.on_variable_func_def(&node),
        0x002E => visitor.on_variable_return_st(&node),
        0x002F => visitor.on_variable_dotable(&node),
        0x0030 => visitor.on_variable_exp_atom_no_dot(&node),
        0x0031 => visitor.on_variable_exp_atom(&node),
        0x0032 => visitor.on_variable_dot_call(&node),
        0x0033 => visitor.on_variable_dot_call_fn(&node),
        0x0034 => visitor.on_variable_exp_reizdal(&node),
        0x0035 => visitor.on_variable_reiz_dal_atl(&node),
        0x0036 => visitor.on_variable_reiz(&node),
        0x0037 => visitor.on_variable_dal(&node),
        0x0038 => visitor.on_variable_atlik(&node),
        0x0039 => visitor.on_variable_exp_plusmin(&node),
        0x003A => visitor.on_variable_plus(&node),
        0x003B => visitor.on_variable_minus(&node),
        0x003C => visitor.on_variable_exp_t(&node),
        0x003D => visitor.on_variable_exp_a(&node),
        0x003E => visitor.on_variable_xvai(&node),
        0x003F => visitor.on_variable_vai(&node),
        0x0040 => visitor.on_variable_un(&node),
        0x0041 => visitor.on_variable_exp(&node),
        0x0042 => visitor.on_variable_multiple_ids(&node),
        0x0043 => visitor.on_variable_multiple_id_define(&node),
        0x0044 => visitor.on_variable_object_field(&node),
        0x0045 => visitor.on_variable_object_def_field(&node),
        0x0046 => visitor.on_variable_object(&node),
        0x0047 => visitor.on_variable_object_def(&node),
        0x0048 => visitor.on_variable_stat(&node),
        0x0049 => visitor.on_variable_block(&node),
        0x004A => visitor.on_variable_root(&node),
        _ => ()
    };
}