#![allow(warnings)]

use std::collections::HashMap;
use std::fmt::format;
use std::sync::mpsc::sync_channel;
use frontend::ast::{Expr, Operator};
use log::info;

#[derive(Clone)]
pub enum Func {
    RetTyp(String),
    ParamType(String),
}

#[derive(Clone, Debug)]
pub enum SymbolInfo {
    Variable { typ: String },
    Function { ret_type: String, param_types: Vec<(String, String)> },
    Boolean { value: bool },
}

pub struct SymbolTable {
    tables: Vec<HashMap<String, SymbolInfo>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            tables: vec![HashMap::new()],
        }
    }
    pub fn declare_boolean(&mut self, name: String, value: bool) {
        if let Some(scope) = self.tables.last_mut() {
            scope.insert(name, SymbolInfo::Boolean { value });
        }
    }

    pub fn get_bool_value(&self, name: &str) -> Result<bool, String> {
        for scope in self.tables.iter().rev() {
            if let Some(symbol_info) = scope.get(name) {
                if let SymbolInfo::Boolean { value } = symbol_info {
                    return Ok(*value);
                }
            }
        }
        Err(format!("Boolean '{}' not found in any scope", name))
    }
    pub fn get_type(&self, var_name: &str) -> Result<String, String> {
        for scope in self.tables.iter().rev() {
            if let Some(symbol_info) = scope.get(var_name) {
                return match symbol_info {
                    SymbolInfo::Variable { typ } => Ok(typ.clone()),
                    SymbolInfo::Function { ret_type, .. } => Ok(ret_type.clone()),
                    _ => Ok("{}".to_string())
                };
            }
        }
        Err(format!("Variable '{}' not found in any scope", var_name))
    }

    pub(crate) fn enter_scope(&mut self) {
        self.tables.push(HashMap::new())
    }

    pub fn lookup_current_scope(&self, name: &str) -> Option<&SymbolInfo> {
        self.tables.last().and_then(|scope| scope.get(name))
    }

    pub(crate) fn exit_scope(&mut self) {
        self.tables.pop();
    }

    pub fn declare_variable(&mut self, name: String, typ: String) {
        if let Some(scope) = self.tables.last_mut() {
            scope.insert(name, SymbolInfo::Variable { typ });
        }
    }

    pub fn declare_function(&mut self, name: String, ret_type: String, param_types: Vec<(String, String)>) {
        if let Some(scope) = self.tables.last_mut() {
            scope.insert(name, SymbolInfo::Function { ret_type, param_types });
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.tables.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn add_fncs_g_variables(&mut self) {
        let global_scope = self.tables[0].clone();
        if let Some(current_scope) = self.tables.last_mut() {
            for (name, symbol_info) in global_scope {
                current_scope.insert(name.clone(), symbol_info);
            }
        }
    }
}

pub fn semantic_analysis(ast: &Expr, sym_table: &mut SymbolTable) -> Result<String, String> {
    match ast {
        Expr::MainFuncDef(statements) => {
            info!("Analyzing main function");
            if sym_table.lookup_current_scope("main").is_some() {
                return Err("Main function already declared".to_string());
            }

            sym_table.declare_function("main".to_string(), "void".to_string(), vec![]);
            sym_table.enter_scope();
            sym_table.add_fncs_g_variables();

            let _body_type = semantic_analysis(statements, sym_table)?;

            sym_table.exit_scope();
            Ok("void".to_string())
        }

        Expr::Number(value) => {
            info!("Analyzing number: {}", value);
            Ok("integer".to_string())
        }

        Expr::Variable(name) => {
            info!("Analyzing variable: {}", name);
            if sym_table.lookup(name).is_none() {
                Err(format!("Undefined variable '{}'", name))
            } else {
                Ok(sym_table.get_type(name).unwrap())
            }
        }

        Expr::VarDeclaration(name, typ) => {
            info!("Declaring variable: {} of type {}", name, typ);
            if sym_table.lookup_current_scope(name).is_some() {
                Err(format!("Variable '{}' already declared", name))
            } else {
                sym_table.declare_variable(name.clone(), typ.clone());
                Ok(typ.clone())
            }
        }

        Expr::Binary(left, op, right) => {
            info!("Analyzing binary operation: {:?} {:?} {:?}", left, op, right);
            let left_type = semantic_analysis(left, sym_table)?;
            let right_type = semantic_analysis(right, sym_table)?;

            if left_type == right_type {
                match op {
                    Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => {
                        if left_type == "integer" {
                            Ok("integer".to_string())
                        } else {
                            Err("Not supported operation".to_string())
                        }
                    }

                    Operator::GreaterThan | Operator::LessThan | Operator::GreaterThanEqual | Operator::LessThanEqual | Operator::Equal => Ok("bool".to_string()),
                    _ => Err("Unsupported operation".to_string()),
                }
            } else {
                Err("Type mismatch in binary operation".to_string())
            }
        }

        Expr::Assign(lhs, rhs) => {
            info!("Analyzing assignment: {:?} = {:?}", lhs, rhs);
            if let Expr::Variable(var_name) = &**lhs {
                let var_type = sym_table.get_type(var_name)?;
                let expr_type = semantic_analysis(rhs, sym_table)?;
                if var_type == expr_type {
                    Ok(var_type)
                } else {
                    Err(format!("Type mismatch in assignment: {} to {}", expr_type, var_type))
                }
            } else {
                Err("Left-hand side must be a variable".to_string())
            }
        }

        Expr::Boolean(value) => {
            info!("Analyzing boolean: {}", value);
            Ok("bool".to_string())
        }

        Expr::String(value) => {
            info!("Analyzing string: {}", value);
            Ok("string".to_string())
        }

        Expr::FuncDef(name, ret_typ, params, body) => {
            info!("Analyzing function: {} with return type: {}", name, ret_typ);
            if sym_table.lookup_current_scope(name).is_some() {
                return Err(format!("Function '{}' already declared", name));
            }

            let params: Vec<(String, String)> = params.iter().cloned().collect();
            sym_table.declare_function(name.clone(), ret_typ.clone(), params.clone());

            sym_table.enter_scope();
            for (param_name, param_type) in &params {
                sym_table.declare_variable(param_name.clone(), param_type.clone());
            }

            let body_type = semantic_analysis(body, sym_table)?;

            match &**body {
                Expr::Statements(statements) => {
                    if let Some(Expr::Return(_)) = statements.last() {
                        if body_type != *ret_typ {
                            return Err(format!("Return type mismatch in function '{}': expected '{}', found '{}'", name, ret_typ, body_type));
                        }
                    } else {
                        if *ret_typ != "void".to_string() {
                            return Err(format!("Return in function '{}' not specified : expected '{}', found '{}'", name, ret_typ, body_type));
                        }
                    }
                }
                _ => return Err(format!("Function '{}' body must be a block of statements", name)),
            }

            sym_table.exit_scope();
            Ok(ret_typ.clone())
        }

        Expr::Statements(statements) => {
            let mut last_type = "".to_string();
            for statement in statements {
                last_type = semantic_analysis(statement, sym_table)?;
            }
            Ok(last_type)
        }

        Expr::Return(expr) => {
            info!("Analyzing return: {:?}", expr);
            semantic_analysis(expr, sym_table)
        }

        Expr::PrintStr(expr) => {
            info!("Analyzing print string: {:?}", expr);
            Ok("void".to_string())
        }

        Expr::Print(expr) => {
            info!("Analyzing print: {:?}", expr);
            Ok("void".to_string())
        }

        Expr::FuncCall(name, args) => {
            info!("Analyzing function call: {}({:?})", name, args);
            if sym_table.lookup(name).is_none() {
                return Err(format!("Function '{}' isn't declared", name));
            }

            Ok("integer".to_string())
        }

        Expr::If(cond, then, els) => {
            info!("Analyzing if statement");

            let cond_type = semantic_analysis(cond, sym_table)?;

            if cond_type != "bool" {
                return Err(format!("Expected condition to be bool, found {}", cond_type));
            }

            let then_type = semantic_analysis(then, sym_table)?;

            if let Some(els) = els {
                let else_type = semantic_analysis(els, sym_table)?;
                /*if then_type != else_type {
                    return Err("Type mismatch between then and else".to_string());
                }*/
            }
            Ok(then_type)
        }

        Expr::VarDeclarationWithAssignment(name, typ, expr) => {
            info!("Declaring variable with assignment: {} of type {}", name, typ);
            if sym_table.lookup_current_scope(name).is_some() {
                return Err(format!("Variable '{}' already declared", name));
            } else {
                let expr_type = semantic_analysis(expr, sym_table)?;
                if expr_type == *typ {
                    sym_table.declare_variable(name.clone(), typ.clone());
                    Ok(typ.clone())
                } else {
                    Err(format!("Type mismatch in variable declaration with assignment: expected '{}', found '{}'", typ, expr_type))
                }
            }
        }

        Expr::Semicolon(expr) => {

            Ok("void".to_string())
        }

        Expr::Program(statements) => {
            info!("Analyzing program");
            let mut last_type = "".to_string();
            for statement in statements {
                last_type = semantic_analysis(statement, sym_table)?;
            }
            Ok(last_type)
        }

        Expr::While(cond, body) => {
            info!("Analyzing while loop");

            let cond_type = semantic_analysis(cond, sym_table)?;
            if cond_type != "bool" {
                return Err(format!("Expected condition to be bool, found {}", cond_type));
            }

            semantic_analysis(body, sym_table)?;
            Ok("void".to_string())
        }

        _ => Err("Unsupported expression type".to_string()),
    }
}


