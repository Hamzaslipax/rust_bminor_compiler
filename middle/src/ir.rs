use log::info;
use frontend::ast::{Expr, Operator};
#[derive(Debug, Clone)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Store,
    LoadConst,
    LoadVar,
    StoreVar,
    FuncCall,
    FuncDef,
    Return,
    DeclareVar,
    BranchIfFalse,
    JumpIfZero,
    Jump,
    Label,
    Param,
    PrintStr,
    PrintVar,
    While,
    BranchWhile,
    Goto,
    BranchIfTrue,
    Default,
    GreaterThan,
    LessThan,
    JumpGreaterThan,
    JumpLessThan,
    JumpGreaterThanEqual,
    JumpLessThanEqual,
    GreaterThanEqual,
    LessThanEqual,
    Equal,
    NotEqual,
    JumpEqual,
    JumpNotEqual,
    Bool,
    JumpBool,
    ReadFile,
}

#[derive(Debug, Clone)]
pub enum IRValue {
    Number(i32),
    TempReg(u32),
    Variable(String),
    FuncName(String),
    Label(u32),
    Str(String),
    Bool(String),
}

#[derive(Debug, Clone)]
pub struct IRInstruction {
    pub opcode: Opcode,
    pub operands: Vec<IRValue>,
}

pub fn generate_ir(ast: &Expr) -> Vec<IRInstruction> {
    let mut instructions = Vec::new();
    let mut reg_counter = 0;
    let mut label_counter = 0;

    generate_ir_recursive(ast, &mut instructions, &mut reg_counter, &mut label_counter);
    instructions
}

fn generate_ir_recursive(ast: &Expr, instructions: &mut Vec<IRInstruction>, reg_counter: &mut u32, label_counter: &mut u32) {
    match ast {
        Expr::Program(statements) => {
            info!("Generating IR for program");
            for statement in statements {
                generate_ir_recursive(statement, instructions, reg_counter, label_counter);
            }
        }

        Expr::MainFuncDef(statements) => {
            info!("Generating IR for main function");

            instructions.push(IRInstruction {
                opcode: Opcode::FuncDef,
                operands: vec![IRValue::FuncName("main".to_string())],
            });

            generate_ir_recursive(statements, instructions, reg_counter, label_counter);

            instructions.push(IRInstruction {
                opcode: Opcode::Return,
                operands: vec![IRValue::Number(0)],
            });
        }

        Expr::Number(value) => {
            info!("Generating IR for number: {}", value);
            instructions.push(IRInstruction {
                opcode: Opcode::LoadConst,
                operands: vec![IRValue::Number(*value), IRValue::TempReg(*reg_counter)],
            });
            *reg_counter += 1;
        }

        Expr::Variable(name) => {
                instructions.push(IRInstruction {
                    opcode: Opcode::LoadVar,
                    operands: vec![IRValue::Variable(name.clone()), IRValue::TempReg(*reg_counter)],
                });
                *reg_counter += 1;
            }


        Expr::Binary(left, op, right) => {
            info!("Generating IR for binary operation: {:?} {:?} {:?}", left, op, right);
            generate_ir_recursive(left, instructions, reg_counter, label_counter);
            let left_reg = *reg_counter - 1;

            let right_operand = if let Expr::Number(value) = **right {
                IRValue::Number(value)
            } else {
                generate_ir_recursive(right, instructions, reg_counter, label_counter);
                let right_reg = *reg_counter - 1;
                IRValue::TempReg(right_reg)
            };

            let result_reg = *reg_counter;

            let opcode = match op {
                Operator::Add => Opcode::Add,
                Operator::Subtract => Opcode::Sub,
                Operator::Multiply => Opcode::Mul,
                Operator::Divide => Opcode::Div,
                Operator::GreaterThan => Opcode::GreaterThan,
                Operator::LessThan => Opcode::LessThan,
                Operator::GreaterThanEqual => Opcode::GreaterThanEqual,
                Operator::LessThanEqual => Opcode::LessThanEqual,
                Operator::Equal => Opcode::Equal,
                Operator::NotEqual => Opcode::NotEqual,

            };

            instructions.push(IRInstruction {
                opcode,
                operands: vec![IRValue::TempReg(left_reg), right_operand, IRValue::TempReg(result_reg)],
            });
            *reg_counter += 1;
        }

        Expr::Boolean(value) => {
            info!("Generating IR for boolean: {}", value);
            instructions.push(IRInstruction {
                opcode: Opcode::LoadConst,
                operands: vec![IRValue::Bool(value.to_string()), IRValue::TempReg(*reg_counter)],
            });
            *reg_counter += 1;
        }

        Expr::Assign(lhs, rhs) => {
            info!("Generating IR for assignment: {:?} = {:?}", lhs, rhs);
            generate_ir_recursive(rhs, instructions, reg_counter, label_counter);
            let rhs_reg = *reg_counter - 1;

            if let Expr::Variable(ref var_name) = **lhs {
                instructions.push(IRInstruction {
                    opcode: Opcode::StoreVar,
                    operands: vec![IRValue::TempReg(rhs_reg), IRValue::Variable(var_name.clone())],
                });
            } else {
                panic!("Left side of an assignment must be a variable");
            }
        }

        Expr::VarDeclaration(name, typ) => {
            info!("Generating IR for variable declaration: {} : {}", name, typ);
            instructions.push(IRInstruction {
                opcode: Opcode::DeclareVar,
                operands: vec![IRValue::Variable(name.clone()), IRValue::Variable(typ.clone())],
            });
        }

        Expr::VarDeclarationWithAssignment(name, typ, expr) => {
            info!("Generating IR for variable declaration with assignment: {} : {} = {:?}", name, typ, expr);
            instructions.push(IRInstruction {
                opcode: Opcode::DeclareVar,
                operands: vec![IRValue::Variable(name.clone()), IRValue::Variable(typ.clone())],
            });
            generate_ir_recursive(expr, instructions, reg_counter, label_counter);
            let expr_reg = *reg_counter - 1;
            instructions.push(IRInstruction {
                opcode: Opcode::StoreVar,
                operands: vec![IRValue::TempReg(expr_reg), IRValue::Variable(name.clone())],
            });
        }

        Expr::FuncDef(name, ret_type, params, body) => {
            info!("Generating IR for function definition: {} : {}", name, ret_type);
            instructions.push(IRInstruction {
                opcode: Opcode::FuncDef,
                operands: vec![IRValue::FuncName(name.clone())],
            });

            for (i, param) in params.iter().enumerate() {
                instructions.push(IRInstruction {
                    opcode: Opcode::DeclareVar,
                    operands: vec![IRValue::Variable(param.0.clone()), IRValue::Variable(param.1.clone())],
                });
                instructions.push(IRInstruction {
                    opcode: Opcode::Param,
                    operands: vec![IRValue::Number(i as i32), IRValue::Variable(param.0.clone())],
                });
            }

            generate_ir_recursive(body, instructions, reg_counter, label_counter);
        }

        Expr::FuncCall(name, args) => {
            info!("Generating IR for function call: {}({:?})", name, args);
            for (i, arg) in args.iter().enumerate() {
                generate_ir_recursive(arg, instructions, reg_counter, label_counter);
                instructions.push(IRInstruction {
                    opcode: Opcode::Param,
                    operands: vec![IRValue::Number(i as i32), IRValue::TempReg(*reg_counter - 1)],
                })
            }

            let call_reg = *reg_counter;
            instructions.push(IRInstruction {
                opcode: Opcode::FuncCall,
                operands: vec![IRValue::FuncName(name.clone()), IRValue::TempReg(call_reg)],
            });

            *reg_counter += 1;
        }

        Expr::Statements(body) => {
            info!("Generating IR for statements");
            for statement in body {
                generate_ir_recursive(statement, instructions, reg_counter, label_counter);
            }
        }

        Expr::Return(ret) => {
            info!("Generating IR for return: {:?}", ret);
            generate_ir_recursive(ret, instructions, reg_counter, label_counter);
            let ret_reg = *reg_counter - 1;
            instructions.push(IRInstruction {
                opcode: Opcode::Return,
                operands: vec![IRValue::TempReg(ret_reg)],
            });
        }

        Expr::If(cond, then, els) => {
            info!("Generating IR for if statement: {:?}", cond);
            generate_ir_recursive(cond, instructions, reg_counter, label_counter);
            let cond_reg = *reg_counter - 1;

            let first_label = *label_counter;
            *label_counter += 1;
            let end_label = *label_counter;
            *label_counter += 1;
            let mut opcode = Opcode::Default;
            match **cond {
                Expr::Binary(_, op, _) =>{
                    match op {
                        Operator::GreaterThan => {
                            opcode = Opcode::JumpGreaterThan
                        }

                        Operator::GreaterThanEqual => {
                            opcode = Opcode::JumpGreaterThanEqual
                        }

                        Operator::LessThanEqual => {
                            opcode = Opcode::JumpLessThanEqual
                        }

                        Operator::LessThan => {
                            opcode = Opcode::JumpLessThan
                        }

                        Operator::Equal => {
                            opcode = Opcode::JumpEqual
                        }

                        Operator::NotEqual => {
                            opcode = Opcode::JumpNotEqual
                        }
                        _ => {}
                    }
                }


                Expr::Variable(ref _value) => {
                    instructions.push(IRInstruction {
                        opcode: Opcode::Bool,
                        operands: vec![IRValue::Bool("true".to_string()), IRValue::TempReg(*reg_counter)],

                    });
                    *reg_counter += 1;
                    opcode = Opcode::JumpBool

                }
                _ => {}
            }

            instructions.push(IRInstruction {
                opcode,
                operands: vec![IRValue::TempReg(cond_reg), IRValue::Label(first_label)],
            });

            generate_ir_recursive(then, instructions, reg_counter, label_counter);

            instructions.push(IRInstruction {
                opcode: Opcode::Jump,
                operands: vec![IRValue::Label(end_label)],
            });

            instructions.push(IRInstruction {
                opcode: Opcode::Label,
                operands: vec![IRValue::Label(first_label)],
            });

            if let Some(els_exp) = els {
                generate_ir_recursive(els_exp, instructions, reg_counter, label_counter);
            }

            instructions.push(IRInstruction {
                opcode: Opcode::Label,
                operands: vec![IRValue::Label(end_label)],
            });

        }

        Expr::While(condition, body) => {
            info!("Generating IR for While: {:?}", condition);

            let start_label = *label_counter;
            *label_counter += 1;
            let end_label = *label_counter;
            *label_counter += 1;

            instructions.push(IRInstruction {
                opcode: Opcode::Label,
                operands: vec![IRValue::Label(start_label)],
            });

            generate_ir_recursive(condition, instructions, reg_counter, label_counter);
            let cond_reg = *reg_counter - 1;

            let opcode = match **condition {
                Expr::Binary(_, op, _) => match op {
                    Operator::GreaterThan => Opcode::JumpGreaterThan,
                    Operator::GreaterThanEqual => Opcode::JumpGreaterThanEqual,
                    Operator::LessThan => Opcode::JumpLessThan,
                    Operator::LessThanEqual => Opcode::JumpLessThanEqual,
                    Operator::Equal => Opcode::JumpEqual,
                    Operator::NotEqual => Opcode::JumpNotEqual,

                    _ => panic!("Unsupported binary op in condition"),
                },
                _ => panic!("Unsupported cond type for while loop"),
            };
            instructions.push(IRInstruction {
                opcode,
                operands: vec![IRValue::TempReg(cond_reg), IRValue::Label(end_label)],
            });

            generate_ir_recursive(body, instructions, reg_counter, label_counter);

            instructions.push(IRInstruction {
                opcode: Opcode::Jump,
                operands: vec![IRValue::Label(start_label)],
            });

            instructions.push(IRInstruction {
                opcode: Opcode::Label,
                operands: vec![IRValue::Label(end_label)],
            })
        }

        Expr::PrintStr(str) => {
            info!("Generating IR for print string: {}", str);
            instructions.push(IRInstruction {
                opcode: Opcode::PrintStr,
                operands: vec![IRValue::Str(str.clone())],
            });
        }

        Expr::Print(var) => {
            info!("Generating IR for print variable: {:?}", var);
            generate_ir_recursive(var, instructions, reg_counter, label_counter);
            let var_reg = *reg_counter - 1;
            instructions.push(IRInstruction {
                opcode: Opcode::PrintVar,
                operands: vec![IRValue::TempReg(var_reg)],
            });
        }

        Expr::PrintStr(str) => {
            info!("Generating IR for print string: {:?}", str);
            instructions.push(IRInstruction{
                opcode: Opcode::PrintStr,
                operands: vec![IRValue::Str(str.clone())],
            })
        }



        _ => panic!("Unsupported expression type for IR generation {:?}", ast),

    }
}

