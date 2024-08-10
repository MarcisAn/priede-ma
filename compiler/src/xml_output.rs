use crate::OPTCODE;

pub fn format_xml_file(bytecode: Vec<OPTCODE>) -> String {
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
pub fn format_macro_line(optcode: OPTCODE, lineid: usize) -> String {
    let command = match optcode {
        OPTCODE::LoadNumber { value, register } =>
            format!("SetVar ${} = {}", "reg_".to_string() + &register.to_string(), value),
        OPTCODE::AreVarsEqual { target_reg, a_reg, b_reg } =>
            format!(
                "[${} == ${}] SetVar ${} = 1",
                "reg_".to_string() + &a_reg.to_string(),
                "reg_".to_string() + &b_reg.to_string(),
                "reg_".to_string() + &target_reg.to_string()
            ),
        OPTCODE::LargerThan { target_reg, a_reg, b_reg } =>
            format!(
                "[${} > ${}] SetVar ${} = 1",
                "reg_".to_string() + &a_reg.to_string(),
                "reg_".to_string() + &b_reg.to_string(),
                "reg_".to_string() + &target_reg.to_string()
            ),
        OPTCODE::LargerEq { target_reg, a_reg, b_reg } =>
            format!(
                "[${} >= ${}] SetVar ${} = 1",
                "reg_".to_string() + &a_reg.to_string(),
                "reg_".to_string() + &b_reg.to_string(),
                "reg_".to_string() + &target_reg.to_string()
            ),
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
        OPTCODE::Jump { line_to_jump_to } =>
            format!(
                "Macro 1.1.{}",
                line_to_jump_to
            ),
        OPTCODE::SelectFixture { id_register } =>
            format!("Fixture ${}", "reg_".to_string() + &id_register.to_string()),
        OPTCODE::DefineVariable { name, value_reg } => format!("SetVar $priedevar_{} = $reg_{}", name, value_reg),
        OPTCODE::GetVariable { name, target_reg } => format!("SetVar $reg_{} = $priedevar_{}", target_reg, name)
    };
    format!("<Macroline index=\"{}\" delay=\"0\">
			<text>{}</text>
		</Macroline>", lineid, command)
}