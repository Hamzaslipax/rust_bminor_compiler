use frontend::parser::ProgramParser;
use middle::ir::{generate_ir, Opcode};

#[test]
fn try_catch_generates_ir() {
    let src = "main: function void () = { try { print \"t\"; } catch { print \"c\"; } }";
    let ast = ProgramParser::new().parse(src).expect("parse try-catch");
    let ir = generate_ir(&ast);
    let try_pos = ir.iter().position(|i| matches!(i.opcode, Opcode::Try));
    let catch_pos = ir.iter().position(|i| matches!(i.opcode, Opcode::Catch));
    assert!(try_pos.is_some(), "Try opcode missing");
    assert!(catch_pos.is_some(), "Catch opcode missing");
    assert!(try_pos.unwrap() < catch_pos.unwrap(), "Catch occurs before Try");
}
