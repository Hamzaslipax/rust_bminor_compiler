use log::error;
use crate::ir::{IRInstruction, IRValue, Opcode};


pub fn print_ir(instructions: &Vec<IRInstruction>) -> String {
    let mut output = String::new();

    for instr in instructions {
        let line = match &instr.opcode {
            Opcode::FuncDef => {
                if let IRValue::FuncName(name) = &instr.operands[0] {
                    format!("{}:", name)
                } else {
                    format!("Expected FuncName for FuncDef")
                }
            },
            Opcode::DeclareVar => {
                if let IRValue::Variable(var_name) = &instr.operands[0] {
                    format!(" DeclareVar {};", var_name)
                } else {
                    format!("Expected Variable for DeclareVar")
                }
            },
            Opcode::LoadConst => {
                if let (IRValue::Number(num), IRValue::TempReg(temp)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" _t{} = {};", temp, num)
                } else {
                    format!("Expected Number and TempReg for LoadConst")
                }
            },
            Opcode::LoadVar => {
                if let (IRValue::Variable(var_name), IRValue::TempReg(temp)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" _t{} = {};", temp, var_name)
                } else {
                    format!("Expected Variable and TempReg for LoadVar")
                }
            },
            Opcode::StoreVar => {
                if let (IRValue::TempReg(temp), IRValue::Variable(var_name)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" {} = _t{};", var_name, temp)
                } else {
                    format!("Expected TempReg and Variable for StoreVar")
                }
            },
            Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => {
                let (op, symbol) = match &instr.opcode {
                    Opcode::Add => ("Add", "+"),
                    Opcode::Sub => ("Sub", "-"),
                    Opcode::Mul => ("Mul", "*"),
                    Opcode::Div => ("Div", "/"),
                    _ => unreachable!(),
                };
                if let (IRValue::TempReg(left), IRValue::TempReg(right), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} {} _t{};", result, left, symbol, right)
                } else if let (IRValue::TempReg(left), IRValue::Number(num), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} {} _t{};", result, left, symbol, num)
                }
                else {
                    format!("Expected three TempRegs for {}", op)
                }
            },
            Opcode::FuncCall => {
                if let (IRValue::FuncName(func_name), IRValue::TempReg(result)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" LCall {};\n _t{} = rax;", func_name, result)
                } else {
                    format!("Expected FuncName and TempReg for FuncCall")
                }
            },
            Opcode::Return => {
                match &instr.operands[0] {
                    IRValue::Number(num) => format!(" Return {};", num),
                    IRValue::TempReg(temp) => format!(" Return _t{};", temp),
                    _ => format!("Expected Number or TempReg for Return"),
                }
            },
            Opcode::BranchIfTrue => {
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" BranchIfTrue _t{} L{};", cond, label)
                } else {
                    format!("Expected TempReg and Label for BranchIfTrue")
                }
            },
            Opcode::BranchIfFalse => {
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" BranchIfFasle _t{} L{};", cond, label)
                } else {
                    format!("Expected TempReg and Label for BranchIfTrue")
                }
            },
            Opcode::JumpIfZero => {
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" JumpIfZero _t{} L{};", cond, label)
                } else {
                    format!("Expected TempReg and Label for JumpIfZero")
                }
            },
            Opcode::Jump => {
                if let IRValue::Label(label) = &instr.operands[0] {
                    format!(" Jump L{};", label)
                } else {
                    format!("Expected Label for Jump")
                }
            },
            Opcode::Label => {
                if let IRValue::Label(label) = &instr.operands[0] {
                    format!("L{}:", label)
                } else {
                    format!("Expected Label for Label")
                }
            },
            Opcode::PrintStr => {
                if let IRValue::Str(s) = &instr.operands[0] {
                    format!(" PrintStr \"{}\";", s)
                } else {
                    format!("Expected Str for PrintStr")
                }
            },
            Opcode::PrintVar => {
                if let IRValue::TempReg(temp) = &instr.operands[0] {
                    format!(" PrintVar _t{};", temp)
                } else {
                    format!("Expected TempReg for PrintVar")
                }
            },
            Opcode::Param => {
                if let (IRValue::Number(index), IRValue::TempReg(temp)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" Param {} = _t{};", index, temp)
                } else if let (IRValue::Number(index), IRValue::Variable(var)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" Param {} = {};", index, var)
                } else {
                    format!("Expected Number and TempReg or Variable for Param")
                }
            },
            Opcode::GreaterThan => {
                if let (IRValue::TempReg(left), IRValue::TempReg(right), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} > _t{};", result, left, right)
                } else if let (IRValue::TempReg(left), IRValue::Number(num), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} > _t{};", result, left, num)
                } else {
                    format!("Expected Number and TempReg or Variable for Param")
                }
            },
            Opcode::LessThan => {
                if let (IRValue::TempReg(left), IRValue::TempReg(right), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} < _t{};", result, left, right)
                } else {
                    format!("Expected three TempRegs for LessThan")
                }
            },
            Opcode::While => {
                if let (IRValue::Label(start_label), IRValue::TempReg(cond), IRValue::Label(end_label)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" While L{} _t{} L{};", start_label, cond, end_label)
                } else {
                    format!("Expected Label, TempReg, and Label for While")
                }
            },
            Opcode::BranchWhile => {
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" BranchWhile _t{} L{};", cond, label)
                } else {
                    format!("Expected TempReg and Label for BranchWhile")
                }
            },
            Opcode::LessThanEqual => {
                if let (IRValue::TempReg(left), IRValue::TempReg(right), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} <= _t{};", result, left, right)
                } else if let (IRValue::TempReg(left), IRValue::Number(num), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} <= {};", result, left, num)
                } else {
                    unreachable!("Expected TempRegs or TempReg, Number and TempReg for LessThanEqual");
                }
            },
            Opcode::JumpLessThanEqual => {
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" JumpLessThanEqual _t{} L{};", cond, label)
                } else {
                    unreachable!("Expected TempReg and Label for JumpLessThanEqual");
                }
            },
            Opcode::JumpGreaterThan => {
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" JumpGreaterThan _t{} L{};", cond, label)
                } else {
                    format!("Expected TempReg and Label for JumpGreaterThan")
                }
            },
            Opcode::JumpNotEqual => {
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instr.operands[0], &instr.operands[1]) {
                    format!(" JumpNotEqual _t{} L{};", cond, label)
                } else {
                    format!("Expected TempReg and Label for JumpNotEqual")
                }
            },
            Opcode::NotEqual => {
                if let (IRValue::TempReg(left), IRValue::TempReg(right), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} != _t{};", result, left, right)
                } else if let (IRValue::TempReg(left), IRValue::Number(num), IRValue::TempReg(result)) =
                    (&instr.operands[0], &instr.operands[1], &instr.operands[2]) {
                    format!(" _t{} = _t{} != {};", result, left, num)
                } else {
                    format!("Expected TempRegs or TempReg, Number and TempReg for NotEqual")
                }
            },

            Opcode::Goto => {
                if let IRValue::Label(label) = &instr.operands[0] {
                    format!(" Goto L{};", label)
                } else {
                    format!("Expected Label for Goto")
                }
            },
            _ => format!(" Unsupported opcode: {:?}", instr.opcode),

        };
        output.push_str(&line);
        output.push('\n');
    }

    output
}