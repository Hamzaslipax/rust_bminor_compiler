use frontend::parser::ProgramParser;
use backend::x86_assembler_generator::{execute_x86, generate_assembly};
use middle::ir::{generate_ir, Opcode};

#[test]
fn try_catch_generates_ir() {
    let src = "main: function void () = { try { print \"t\"; } catch { print \"c\"; } }";
    let ast = ProgramParser::new().parse(src).expect("parse try-catch");
    let ir = generate_ir(&ast);
    let try_pos = ir.iter().position(|i| matches!(i.opcode, Opcode::Try)).expect("Try opcode missing");
    let catch_pos = ir.iter().position(|i| matches!(i.opcode, Opcode::Catch)).expect("Catch opcode missing");
    assert!(try_pos < catch_pos, "Catch occurs before Try");

    // verify that a jump over the catch block and an end label exist
    let _jump_pos = ir.iter().position(|i| matches!(i.opcode, Opcode::Jump)).expect("Jump over catch missing");
    let label_count = ir.iter().filter(|i| matches!(i.opcode, Opcode::Label)).count();
    assert!(label_count >= 2, "Labels for catch/end missing");

    // generate assembly and execute to ensure assembly generation works
    let _asm = generate_assembly(&ir);
    // use a fixed name for the binary
    execute_x86(&"output".to_string()).expect("assemble/run");
}
