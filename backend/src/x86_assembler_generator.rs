use middle::ir::{IRInstruction, IRValue, Opcode};
use std::collections::HashMap;
use std::fs::{OpenOptions};
use std::io;
use std::io::Write;
use std::process::{Command};
use log::info;

pub fn generate_assembly(ir: &Vec<IRInstruction>) -> String {
    let mut assembly_code = String::new();
    let mut has_print = false;
    let mut has_printstr = false;

    let mut variable_offsets = HashMap::new();
    let mut current_offset = 0;
    let mut string_literals = HashMap::new();
    let mut string_counter = 0;

    assembly_code.push_str("section .data\n");

    for instruction in ir {
        match instruction.opcode {
            Opcode::DeclareVar => {
                if let IRValue::Variable(ref name) = instruction.operands[0] {
                    if !variable_offsets.contains_key(name) {
                        current_offset += 8;
                        variable_offsets.insert(name.clone(), current_offset);
                    }
                }
                info!("Declaring variable offsets: {:?}", instruction.operands[0]);
            },
            Opcode::PrintVar => {
                has_print = true;
                info!("PrintVar instruction encountered");
            },
            Opcode::PrintStr => {
                has_printstr = true;
                info!("PrintStr instruction encountered with string: {:?}", instruction.operands[0]);
                if let IRValue::Str(ref s) = instruction.operands[0] {
                    if !string_literals.contains_key(s) {
                        let label = format!("str_{}", string_counter);
                        string_literals.insert(s.clone(), label.clone());
                        string_counter += 1;
                        assembly_code.push_str(&format!("{} db {:?}, 0\n", label, s));
                    }
                }
            },
            _ => {}
        }
    }

    if current_offset % 16 != 0 {
        current_offset = (current_offset + 15) / 16 * 16;
    }

    if has_print || has_printstr {
        assembly_code.push_str("format db \"%d\", 10, 0\n");
        assembly_code.push_str("str_format db \"%s\", 10, 0\n");
    }

    assembly_code.push_str("\nsection .text\n");
    assembly_code.push_str("global _start\n");

    if has_print || has_printstr {
        assembly_code.push_str("extern printf\n");
    }

    for instruction in ir {
        match instruction.opcode {
            Opcode::FuncDef => {
                if let IRValue::FuncName(ref name) = instruction.operands[0] {
                    assembly_code.push_str(&format!("{}:\n", name));
                    assembly_code.push_str("    push rbp\n");
                    assembly_code.push_str("    mov rbp, rsp\n");
                    assembly_code.push_str(&format!("    sub rsp, {}\n", current_offset));
                }
                info!("Function definition: {:?}", instruction.operands[0]);
            },
            Opcode::DeclareVar => {
            },
            Opcode::Param => {
                info!("Parameter instruction: {:?}", instruction.operands);
                if let (IRValue::Number(index), IRValue::Variable(ref name)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let param_reg = match *index {
                        0 => "rdi",
                        1 => "rsi",
                        2 => "rdx",
                        3 => "rcx",
                        4 => "r8",
                        5 => "r9",
                        _ => panic!("Too many parameters"),
                    };
                    let offset = variable_offsets[name];
                    assembly_code.push_str(&format!("    mov [rbp-{}], {}\n", offset, param_reg));
                }

                if let (IRValue::Number(index), IRValue::TempReg(temp)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let param_reg = match *index {
                        0 => "rdi",
                        1 => "rsi",
                        2 => "rdx",
                        3 => "rcx",
                        4 => "r8",
                        5 => "r9",
                        _ => panic!("Too many parameters"),
                    };
                    let reg = get_register(*temp);
                    assembly_code.push_str(&format!("    mov {}, {}\n", param_reg, reg));
                }
            },
            Opcode::LoadConst => {
                info!("Load constant instruction: {:?}", instruction.operands);
                if let (IRValue::Number(num), IRValue::TempReg(temp)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let reg = get_register(*temp);
                    assembly_code.push_str(&format!("    mov {}, {}\n", reg, num));
                }
            },
            Opcode::LoadVar => {
                info!("Load variable instruction: {:?}", instruction.operands);
                if let (IRValue::Variable(ref name), IRValue::TempReg(temp)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let reg = get_register(*temp);
                    let offset = variable_offsets[name];
                    assembly_code.push_str(&format!("    mov {}, [rbp-{}]\n", reg, offset));
                }
            },
            Opcode::StoreVar => {
                info!("Store variable instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(temp), IRValue::Variable(ref name)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let reg = get_register(*temp);
                    let offset = variable_offsets[name];
                    assembly_code.push_str(&format!("    mov [rbp-{}], {}\n", offset, reg));
                }
            },
            Opcode::Add => {
                info!("Add instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(temp1), IRValue::TempReg(temp2), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg2 = get_register(*temp2);
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    mov {}, {}\n", reg_result, reg1));
                    assembly_code.push_str(&format!("    add {}, {}\n", reg_result, reg2));
                }

                if let (IRValue::TempReg(temp1), IRValue::Number(num), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg_result = get_register(*result);
                    let reg1 = get_register(*temp1);
                    assembly_code.push_str(&format!("    mov {}, {}\n", reg_result, reg1));
                    assembly_code.push_str(&format!("    add {}, {}\n", reg_result, num));
                }
            },
            Opcode::Sub => {
                info!("Subtract instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(temp1), IRValue::TempReg(temp2), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg2 = get_register(*temp2);
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    mov {}, {}\n", reg_result, reg1));
                    assembly_code.push_str(&format!("    sub {}, {}\n", reg_result, reg2));
                }

                if let (IRValue::TempReg(temp1), IRValue::Number(num), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    mov {}, {}\n", reg_result, reg1));
                    assembly_code.push_str(&format!("    sub {}, {}\n", reg_result, num));
                }
            },
            Opcode::Mul => {
                info!("Multiply instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(temp1), IRValue::TempReg(temp2), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg2 = get_register(*temp2);
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    mov {}, {}\n", reg_result, reg1));
                    assembly_code.push_str(&format!("    imul {}, {}\n", reg_result, reg2));
                }
            },
            Opcode::Div => {
                /*if let (IRValue::TempReg(temp1), IRValue::TempReg(temp2), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg2 = get_register(*temp2);
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    mov rax, {}\n", reg1));
                    assembly_code.push_str("    cqo\n");
                    assembly_code.push_str(&format!("    idiv {}\n", reg2));
                    assembly_code.push_str(&format!("    mov {}, rax\n", reg_result));
                }*/
            },
            Opcode::FuncCall => {
                if let (IRValue::FuncName(ref name), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1]) {
                    assembly_code.push_str(&format!("    call {}\n", name));
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    mov {}, rax\n", reg_result));
                }
            },
            Opcode::Return => {
                info!("Return instruction: {:?}", instruction.operands);
                if let IRValue::TempReg(temp) = instruction.operands[0] {
                    let reg = get_register(temp);
                    assembly_code.push_str(&format!("    mov rax, {}\n", reg));
                    assembly_code.push_str("    leave\n");
                    assembly_code.push_str("    ret\n");
                } else if let IRValue::Number(num) = instruction.operands[0] {
                    assembly_code.push_str(&format!("    mov rax, {}\n", num));
                    assembly_code.push_str("    leave\n");
                    assembly_code.push_str("    ret\n");
                }
            },
            Opcode::PrintVar => {
                info!("Print variable instruction: {:?}", instruction.operands);
                if let IRValue::TempReg(temp) = instruction.operands[0] {
                    let reg = get_register(temp);
                    assembly_code.push_str("    mov rdi, format\n");
                    assembly_code.push_str(&format!("    mov rsi, {}\n", reg));
                    assembly_code.push_str("    xor rax, rax\n");
                    assembly_code.push_str("    call printf\n");
                }
            },
            Opcode::PrintStr => {
                info!("Print string instruction: {:?}", instruction.operands);
                if let IRValue::Str(ref s) = instruction.operands[0] {
                    let label = string_literals[s].clone();
                    assembly_code.push_str(&format!("    mov rdi, str_format\n"));
                    assembly_code.push_str(&format!("    lea rsi, [{}]\n", label));
                    assembly_code.push_str(&format!("    xor rax, rax\n"));
                    assembly_code.push_str(&format!("    call printf\n"));
                }
            },

            Opcode::Label => {
                info!("Label instruction: {:?}", instruction.operands);
                if let IRValue::Label(label) = instruction.operands[0] {
                    assembly_code.push_str(&format!("label_{}:\n", label));
                }
            },

            Opcode::Jump => {
                info!("Jump instruction: {:?}", instruction.operands);
                if let IRValue::Label(label) = instruction.operands[0] {
                    assembly_code.push_str(&format!("    jmp label_{}\n", label));
                }
            },

            Opcode::NotEqual | Opcode::Equal | Opcode::GreaterThan | Opcode::LessThan | Opcode::LessThanEqual | Opcode::GreaterThanEqual => {
                info!("{:?} instruction: {:?}", instruction.opcode, instruction.operands);
                if let (IRValue::TempReg(temp1), IRValue::TempReg(temp2), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg2 = get_register(*temp2);
                    let _reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    cmp {}, {}\n", reg1, reg2));
                }
                if let (IRValue::TempReg(temp2), IRValue::Number(num), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg2 = get_register(*temp2);
                    let _reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    cmp {}, {}\n", reg2, num));
                }
                if let (IRValue::Number(num), IRValue::TempReg(temp2), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg2 = get_register(*temp2);
                    let _reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    cmp {}, {}\n", reg2, num));
                }
            },

            Opcode::Bool => {
                info!("Bool instruction: {:?}", instruction.operands);
                if let (IRValue::Bool(bool), IRValue::TempReg(temp1)) = (&instruction.operands[0], &instruction.operands[1]) {
                    println!("{:?}", bool);
                    let binary = match bool.as_str() {

                        "true" => 1,
                        "false" => 0,
                        _ => panic!()
                    };

                    let reg2 = get_register(*temp1 -1);
                    assembly_code.push_str(&format!("    cmp byte {}, {}\n", reg2, binary));
                }
            }

            /*Opcode::LessThan => {
                if let (IRValue::TempReg(temp1), IRValue::TempReg(temp2), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg2 = get_register(*temp2);
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    cmp {}, {}\n", reg1, reg2));
                    assembly_code.push_str("    setl al\n");
                    assembly_code.push_str(&format!("    movzx {}, al\n", reg_result));
                }
                if let (IRValue::TempReg(temp1), IRValue::Number(num), IRValue::TempReg(result)) = (&instruction.operands[0], &instruction.operands[1], &instruction.operands[2]) {
                    let reg1 = get_register(*temp1);
                    let reg_result = get_register(*result);
                    assembly_code.push_str(&format!("    cmp {}, {}\n", reg1, num));
                    assembly_code.push_str("    setl al\n");
                    assembly_code.push_str(&format!("    movzx {}, al\n", reg_result));
                }
            },*/

            Opcode::BranchIfTrue => {
                info!("BranchIfTrue instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    jle label_{}\n", label));
                }
            },

            Opcode::JumpGreaterThan => {
                info!("JumpGreaterThan instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    jle label_{}\n", label));
                }
            },

            Opcode::JumpGreaterThanEqual => {
                info!("JumpGreaterThanEqual instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    jl label_{}\n", label));
                }
            },

            Opcode::JumpLessThan => {
                info!("JumpLessThan instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    jge label_{}\n", label));
                }
            },

            Opcode::JumpLessThanEqual => {
                info!("JumpLessThanEqual instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    jg label_{}\n", label));
                }
            },

            Opcode::JumpEqual => {
                info!("JumpEqual instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    jne label_{}\n", label));
                }
            },

            Opcode::JumpNotEqual => {
                info!("JumpNotEqual instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    je label_{}\n", label));
                }
            },
            Opcode::BranchIfFalse => {
                info!("BranchIfFalse instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    jns label_{}\n", label));
                }
            },

            Opcode::JumpBool => {
                info!("JumpBool instruction: {:?}", instruction.operands);
                if let (IRValue::TempReg(cond), IRValue::Label(label)) = (&instruction.operands[0], &instruction.operands[1]) {
                    let _reg_cond = get_register(*cond);
                    //assembly_code.push_str(&format!("    cmp {}, 0\n", reg_cond));
                    assembly_code.push_str(&format!("    je label_{}\n", label));
                }
            }


            _ => unimplemented!("Opcode {:?} not supported yet", instruction.opcode),
        }
    }

    assembly_code.push_str("_start:\n");
    assembly_code.push_str("    call main\n");
    assembly_code.push_str("    mov rdi, rax\n");
    assembly_code.push_str("    mov rax, 60\n");
    assembly_code.push_str("    syscall\n");
    writo_to_file(&assembly_code);
    assembly_code
}

fn get_register(mut temp: u32) -> &'static str {
    const NUM_REGISTERS: u32 = 14;

    temp %= NUM_REGISTERS;
    match temp {
        0 => "rax",
        1 => "rbx",
        2 => "rcx",
        3 => "rdx",
        4 => "rsi",
        5 => "rdi",
        6 => "r8",
        7 => "r9",
        8 => "r10",
        9 => "r11",
        10 => "r12",
        11 => "r13",
        12 => "r14",
        13 => "r15",
        _ => panic!("Temporary register out of range"),
    }
}

fn writo_to_file(code: &String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("asem.asm")
        .unwrap();

    file.write_all(code.as_bytes()).unwrap()
}

pub fn execute_x86(name: &String) -> io::Result<()> {
    Command::new("nasm")
        .args(&["-f", "elf64", "-o"])
        .arg(format!("{}.o", name))
        .arg("asem.asm")
        .output()?;

    Command::new("gcc")
        .args(&["-nostartfiles", "-o"])
        .arg(&name)
        .arg(format!("{}.o", name))
        .args(&["-no-pie", "-lc"])
        .output()?;

    let _exec_output = Command::new(format!("./{}", name))
        .output()?;

    //println!("Output: {}", String::from_utf8_lossy(&exec_output.stdout));

    Ok(())
}