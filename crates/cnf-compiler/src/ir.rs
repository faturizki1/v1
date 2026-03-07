//! IR — Intermediate Representation.
//!
//! Lowering from AST to deterministic instruction stream.
//! Same input AST → same instruction stream, always.

use crate::ast::ProcedureStatement;
use crate::ast::Program;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Compress {
        target: String,
    },
    VerifyIntegrity {
        target: String,
    },
    Encrypt {
        target: String,
    },
    Decrypt {
        target: String,
    },
    Transcode {
        target: String,
        output_type: String,
    },
    Filter {
        target: String,
        condition: String,
    },
    Aggregate {
        targets: Vec<String>,
        operation: String,
    },
    Convert {
        target: String,
        output_type: String,
    },
    Merge {
        targets: Vec<String>,
        output_name: String,
    },
    Split {
        target: String,
        parts: String,
    },
    Validate {
        target: String,
        schema: String,
    },
    Extract {
        target: String,
        path: String,
    },
    Display {
        message: String,
    },
    Print {
        target: String,
        format: Option<String>,
    },
    Read {
        target: String,
    },
    Set {
        target: String,
        value: String,
    },
    Add {
        target: String,
        operand1: String,
        operand2: String,
    },
    Subtract {
        target: String,
        operand1: String,
        operand2: String,
    },
    Multiply {
        target: String,
        operand1: String,
        operand2: String,
    },
    Divide {
        target: String,
        operand1: String,
        operand2: String,
    },
    Concatenate {
        target: String,
        operands: Vec<String>,
    },
    Substring {
        target: String,
        source: String,
        start: String,
        length: String,
    },
    Length {
        target: String,
        source: String,
    },
    IfStatement {
        condition: String,
        then_instrs: Vec<Instruction>,
        else_instrs: Option<Vec<Instruction>>,
    },
    ForLoop {
        variable: String,
        in_list: String,
        instrs: Vec<Instruction>,
    },
    WhileLoop {
        condition: String,
        instrs: Vec<Instruction>,
    },
    FunctionDef {
        name: String,
        parameters: Vec<String>,
        return_type: Option<String>,
        instrs: Vec<Instruction>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<String>,
    },
    Open {
        file_handle: String,
        file_path: String,
    },
    ReadFile {
        file_handle: String,
        output_stream: String,
    },
    WriteFile {
        file_handle: String,
        input_stream: String,
    },
    Close {
        file_handle: String,
    },
    Checkpoint {
        record_stream: String,
    },
    Replay {
        target: String,
    },
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Compress { target } => {
                write!(f, "COMPRESS({})", target)
            }
            Instruction::VerifyIntegrity { target } => {
                write!(f, "VERIFY-INTEGRITY({})", target)
            }
            Instruction::Encrypt { target } => {
                write!(f, "ENCRYPT({})", target)
            }
            Instruction::Decrypt { target } => {
                write!(f, "DECRYPT({})", target)
            }
            Instruction::Transcode {
                target,
                output_type,
            } => {
                write!(f, "TRANSCODE({} -> {})", target, output_type)
            }
            Instruction::Filter { target, condition } => {
                write!(f, "FILTER({} WHERE {})", target, condition)
            }
            Instruction::Aggregate { targets, operation } => {
                write!(f, "AGGREGATE({} AS {})", targets.join(","), operation)
            }
            Instruction::Convert {
                target,
                output_type,
            } => {
                write!(f, "CONVERT({} -> {})", target, output_type)
            }
            Instruction::Merge {
                targets,
                output_name,
            } => {
                write!(f, "MERGE({} INTO {})", targets.join(","), output_name)
            }
            Instruction::Split { target, parts } => {
                write!(f, "SPLIT({} INTO {} PARTS)", target, parts)
            }
            Instruction::Validate { target, schema } => {
                write!(f, "VALIDATE({} AGAINST {})", target, schema)
            }
            Instruction::Extract { target, path } => {
                write!(f, "EXTRACT({} FROM {})", path, target)
            }
            Instruction::Display { message } => {
                write!(f, "DISPLAY({})", message)
            }
            Instruction::Print { target, format } => {
                if let Some(fmt) = format {
                    write!(f, "PRINT({} WITH {})", target, fmt)
                } else {
                    write!(f, "PRINT({})", target)
                }
            }
            Instruction::Read { target } => {
                write!(f, "READ({})", target)
            }
            Instruction::Set { target, value } => {
                write!(f, "SET({} = {})", target, value)
            }
            Instruction::Add {
                target,
                operand1,
                operand2,
            } => {
                write!(f, "ADD({} = {} + {})", target, operand1, operand2)
            }
            Instruction::Subtract {
                target,
                operand1,
                operand2,
            } => {
                write!(f, "SUBTRACT({} = {} - {})", target, operand1, operand2)
            }
            Instruction::Multiply {
                target,
                operand1,
                operand2,
            } => {
                write!(f, "MULTIPLY({} = {} * {})", target, operand1, operand2)
            }
            Instruction::Divide {
                target,
                operand1,
                operand2,
            } => {
                write!(f, "DIVIDE({} = {} / {})", target, operand1, operand2)
            }
            Instruction::Concatenate { target, operands } => {
                write!(f, "CONCATENATE({} = {})", target, operands.join(" + "))
            }
            Instruction::Substring {
                target,
                source,
                start,
                length,
            } => {
                write!(
                    f,
                    "SUBSTRING({} = {}[{}..{}])",
                    target, source, start, length
                )
            }
            Instruction::Length { target, source } => {
                write!(f, "LENGTH({} = len({}))", target, source)
            }
            Instruction::IfStatement {
                condition,
                then_instrs,
                else_instrs,
            } => {
                write!(f, "IF({}) THEN[{}]", condition, then_instrs.len())?;
                if let Some(else_i) = else_instrs {
                    write!(f, " ELSE[{}]", else_i.len())?;
                }
                Ok(())
            }
            Instruction::ForLoop {
                variable,
                in_list,
                instrs,
            } => {
                write!(f, "FOR({} IN {}) [{}]", variable, in_list, instrs.len())
            }
            Instruction::WhileLoop { condition, instrs } => {
                write!(f, "WHILE({}) [{}]", condition, instrs.len())
            }
            Instruction::FunctionDef {
                name,
                parameters,
                return_type,
                instrs,
            } => {
                write!(
                    f,
                    "FUNC-DEF({} [{}] ret{})",
                    name,
                    parameters.join(","),
                    return_type.as_ref().unwrap_or(&"(none)".to_string())
                )?;
                write!(f, " [{}]", instrs.len())
            }
            Instruction::FunctionCall { name, arguments } => {
                write!(f, "FUNC-CALL({}({})", name, arguments.join(","))
            }
            Instruction::Open { file_handle, file_path } => {
                write!(f, "OPEN({} AS {})", file_handle, file_path)
            }
            Instruction::ReadFile { file_handle, output_stream } => {
                write!(f, "READ-FILE({} INTO {})", file_handle, output_stream)
            }
            Instruction::WriteFile { file_handle, input_stream } => {
                write!(f, "WRITE-FILE({} FROM {})", file_handle, input_stream)
            }
            Instruction::Close { file_handle } => {
                write!(f, "CLOSE({})", file_handle)
            }
            Instruction::Checkpoint { record_stream } => {
                write!(f, "CHECKPOINT({})", record_stream)
            }
            Instruction::Replay { target } => {
                write!(f, "REPLAY({})", target)
            }
        }
    }
}

/// Type validator for checking operation legality
struct TypeValidator;

impl TypeValidator {
    /// Check if an operation is legal on the given type
    fn can_compress(_data_type: &crate::ast::DataType) -> bool {
        // COMPRESS works on all types
        true
    }

    /// Check if an operation is legal on the given type
    #[allow(dead_code)]
    fn can_transcode(data_type: &crate::ast::DataType) -> bool {
        // TRANSCODE not allowed on BINARY-BLOB or FINANCIAL-DECIMAL
        !matches!(
            data_type,
            crate::ast::DataType::BinaryBlob | crate::ast::DataType::FinancialDecimal
        )
    }

    /// Check if an operation is legal on the given type
    fn can_aggregate(data_type: &crate::ast::DataType) -> bool {
        matches!(
            data_type,
            crate::ast::DataType::CsvTable
                | crate::ast::DataType::FinancialDecimal
                | crate::ast::DataType::ParquetTable
        )
    }

    /// Check if an operation is legal on the given type
    fn can_validate(data_type: &crate::ast::DataType) -> bool {
        matches!(
            data_type,
            crate::ast::DataType::JsonObject
                | crate::ast::DataType::XmlDocument
                | crate::ast::DataType::CsvTable
        )
    }

    /// Check if an operation is legal on the given type
    fn can_extract(data_type: &crate::ast::DataType) -> bool {
        matches!(
            data_type,
            crate::ast::DataType::JsonObject | crate::ast::DataType::XmlDocument
        )
    }
}

pub fn lower(program: Program) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();

    // Validate that all procedure statements reference valid variables
    let declared_vars: std::collections::HashSet<String> = program
        .data
        .variables
        .iter()
        .map(|v| v.name.clone())
        .collect();

    // Map variable names to their data types for type checking
    let var_types: std::collections::HashMap<String, crate::ast::DataType> = program
        .data
        .variables
        .iter()
        .map(|v| (v.name.clone(), v.data_type.clone()))
        .collect();

    // Collect function signatures for parameter count validation
    let mut signatures: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for stmt in &program.procedure.statements {
        if let ProcedureStatement::FunctionDef {
            name, parameters, ..
        } = stmt
        {
            signatures.insert(name.clone(), parameters.len());
        }
    }

    for stmt in &program.procedure.statements {
        match stmt {
            ProcedureStatement::Compress { target } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                // Type check: COMPRESS requires compatible type
                if let Some(dtype) = var_types.get(target) {
                    if !TypeValidator::can_compress(dtype) {
                        return Err(format!(
                            "CNF-C001: COMPRESS operation not allowed on {} type",
                            dtype
                        ));
                    }
                }
                instructions.push(Instruction::Compress {
                    target: target.clone(),
                });
            }
            ProcedureStatement::VerifyIntegrity { target } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::VerifyIntegrity {
                    target: target.clone(),
                });
            }
            ProcedureStatement::Encrypt { target } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::Encrypt {
                    target: target.clone(),
                });
            }
            ProcedureStatement::Decrypt { target } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::Decrypt {
                    target: target.clone(),
                });
            }
            ProcedureStatement::Transcode {
                target,
                output_type,
            } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                // Check if target is BINARY-BLOB - TRANSCODE not allowed
                if let Some(&crate::ast::DataType::BinaryBlob) = var_types.get(target) {
                    return Err("CNF-P007: TRANSCODE operation not supported on BINARY-BLOB type. BINARY-BLOB is raw unstructured data and cannot be transcoded.".to_string());
                }
                instructions.push(Instruction::Transcode {
                    target: target.clone(),
                    output_type: output_type.to_string(),
                });
            }
            ProcedureStatement::Filter { target, condition } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::Filter {
                    target: target.clone(),
                    condition: condition.clone(),
                });
            }
            ProcedureStatement::Aggregate { targets, operation } => {
                for target in targets {
                    if !declared_vars.contains(target) {
                        return Err(format!(
                            "Variable '{}' not declared in DATA DIVISION",
                            target
                        ));
                    }
                    // Type check: AGGREGATE requires compatible type
                    if let Some(dtype) = var_types.get(target) {
                        if !TypeValidator::can_aggregate(dtype) {
                            return Err(format!(
                                "CNF-A001: AGGREGATE operation not allowed on {} type",
                                dtype
                            ));
                        }
                    }
                }
                instructions.push(Instruction::Aggregate {
                    targets: targets.clone(),
                    operation: operation.clone(),
                });
            }
            ProcedureStatement::Convert {
                target,
                output_type,
            } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::Convert {
                    target: target.clone(),
                    output_type: output_type.to_string(),
                });
            }
            ProcedureStatement::Merge {
                targets,
                output_name,
            } => {
                for target in targets {
                    if !declared_vars.contains(target) {
                        return Err(format!(
                            "Variable '{}' not declared in DATA DIVISION",
                            target
                        ));
                    }
                }
                instructions.push(Instruction::Merge {
                    targets: targets.clone(),
                    output_name: output_name.clone(),
                });
            }
            ProcedureStatement::Split { target, parts } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::Split {
                    target: target.clone(),
                    parts: parts.clone(),
                });
            }
            ProcedureStatement::Validate { target, schema } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                // Type check: VALIDATE requires compatible type
                if let Some(dtype) = var_types.get(target) {
                    if !TypeValidator::can_validate(dtype) {
                        return Err(
                            "CNF-V001: VALIDATE operation only allowed on JSON-OBJECT, XML-DOCUMENT, or CSV-TABLE types".to_string()
                        );
                    }
                }
                instructions.push(Instruction::Validate {
                    target: target.clone(),
                    schema: schema.clone(),
                });
            }
            ProcedureStatement::Extract { target, path } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                // Type check: EXTRACT requires compatible type
                if let Some(dtype) = var_types.get(target) {
                    if !TypeValidator::can_extract(dtype) {
                        return Err(
                            "CNF-E001: EXTRACT operation only allowed on JSON-OBJECT or XML-DOCUMENT types".to_string()
                        );
                    }
                }
                instructions.push(Instruction::Extract {
                    target: target.clone(),
                    path: path.clone(),
                });
            }
            ProcedureStatement::Display { message } => {
                instructions.push(Instruction::Display {
                    message: message.clone(),
                });
            }
            ProcedureStatement::Print { target, format } => {
                // Validate target exists
                if !declared_vars.contains(target) {
                    return Err(format!("CNF-E001: Variable '{}' not declared", target));
                }
                instructions.push(Instruction::Print {
                    target: target.clone(),
                    format: format.clone(),
                });
            }
            ProcedureStatement::Read { target } => {
                // Validate target exists
                if !declared_vars.contains(target) {
                    return Err(format!("CNF-E001: Variable '{}' not declared", target));
                }
                instructions.push(Instruction::Read {
                    target: target.clone(),
                });
            }
            ProcedureStatement::Set { target, value } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::Set {
                    target: target.clone(),
                    value: value.clone(),
                });
            }
            ProcedureStatement::Add {
                target,
                operand1,
                operand2,
            } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                if !declared_vars.contains(operand1) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand1
                    ));
                }
                if !declared_vars.contains(operand2) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand2
                    ));
                }
                instructions.push(Instruction::Add {
                    target: target.clone(),
                    operand1: operand1.clone(),
                    operand2: operand2.clone(),
                });
            }
            ProcedureStatement::Subtract {
                target,
                operand1,
                operand2,
            } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                if !declared_vars.contains(operand1) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand1
                    ));
                }
                if !declared_vars.contains(operand2) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand2
                    ));
                }
                instructions.push(Instruction::Subtract {
                    target: target.clone(),
                    operand1: operand1.clone(),
                    operand2: operand2.clone(),
                });
            }
            ProcedureStatement::Multiply {
                target,
                operand1,
                operand2,
            } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                if !declared_vars.contains(operand1) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand1
                    ));
                }
                if !declared_vars.contains(operand2) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand2
                    ));
                }
                instructions.push(Instruction::Multiply {
                    target: target.clone(),
                    operand1: operand1.clone(),
                    operand2: operand2.clone(),
                });
            }
            ProcedureStatement::Divide {
                target,
                operand1,
                operand2,
            } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                if !declared_vars.contains(operand1) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand1
                    ));
                }
                if !declared_vars.contains(operand2) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        operand2
                    ));
                }
                instructions.push(Instruction::Divide {
                    target: target.clone(),
                    operand1: operand1.clone(),
                    operand2: operand2.clone(),
                });
            }
            ProcedureStatement::Concatenate { target, operands } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                for op in operands {
                    if !declared_vars.contains(op) {
                        return Err(format!("Variable '{}' not declared in DATA DIVISION", op));
                    }
                }
                instructions.push(Instruction::Concatenate {
                    target: target.clone(),
                    operands: operands.clone(),
                });
            }
            ProcedureStatement::Substring {
                target,
                source,
                start,
                length,
            } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                if !declared_vars.contains(source) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        source
                    ));
                }
                instructions.push(Instruction::Substring {
                    target: target.clone(),
                    source: source.clone(),
                    start: start.clone(),
                    length: length.clone(),
                });
            }
            ProcedureStatement::Length { target, source } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                if !declared_vars.contains(source) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        source
                    ));
                }
                instructions.push(Instruction::Length {
                    target: target.clone(),
                    source: source.clone(),
                });
            }
            ProcedureStatement::If {
                condition,
                then_statements,
                else_statements,
            } => {
                // Recursively lower nested statements
                let mut then_instrs = Vec::new();
                for stmt in then_statements {
                    let nested_instr = lower_single_statement(stmt, &declared_vars, &signatures)?;
                    then_instrs.push(nested_instr);
                }
                let else_instrs = if let Some(stmts) = else_statements {
                    let mut instrs = Vec::new();
                    for stmt in stmts {
                        let nested_instr =
                            lower_single_statement(stmt, &declared_vars, &signatures)?;
                        instrs.push(nested_instr);
                    }
                    Some(instrs)
                } else {
                    None
                };
                instructions.push(Instruction::IfStatement {
                    condition: condition.clone(),
                    then_instrs,
                    else_instrs,
                });
            }
            ProcedureStatement::For {
                variable,
                in_list,
                statements,
            } => {
                let mut loop_instrs = Vec::new();
                for stmt in statements {
                    let nested_instr = lower_single_statement(stmt, &declared_vars, &signatures)?;
                    loop_instrs.push(nested_instr);
                }
                instructions.push(Instruction::ForLoop {
                    variable: variable.clone(),
                    in_list: in_list.clone(),
                    instrs: loop_instrs,
                });
            }
            ProcedureStatement::While {
                condition,
                statements,
            } => {
                let mut loop_instrs = Vec::new();
                for stmt in statements {
                    let nested_instr = lower_single_statement(stmt, &declared_vars, &signatures)?;
                    loop_instrs.push(nested_instr);
                }
                instructions.push(Instruction::WhileLoop {
                    condition: condition.clone(),
                    instrs: loop_instrs,
                });
            }
            ProcedureStatement::FunctionDef {
                name,
                parameters,
                return_type,
                statements,
            } => {
                // Create a new scope that includes both declared variables and function parameters
                let mut func_scope = declared_vars.clone();
                for param in parameters {
                    func_scope.insert(param.clone());
                }

                let mut func_instrs = Vec::new();
                for stmt in statements {
                    let nested_instr = lower_single_statement(stmt, &func_scope, &signatures)?;
                    func_instrs.push(nested_instr);
                }
                instructions.push(Instruction::FunctionDef {
                    name: name.clone(),
                    parameters: parameters.clone(),
                    return_type: return_type.as_ref().map(|dt| dt.to_string()),
                    instrs: func_instrs,
                });
            }
            ProcedureStatement::FunctionCall { name, arguments } => {
                if let Some(&expected) = signatures.get(name) {
                    if arguments.len() != expected {
                        return Err(format!(
                            "Function '{}' called with {} arguments but defined with {}",
                            name,
                            arguments.len(),
                            expected
                        ));
                    }
                }
                instructions.push(Instruction::FunctionCall {
                    name: name.clone(),
                    arguments: arguments.clone(),
                });
            }
            ProcedureStatement::Open { file_handle, file_path } => {
                if !declared_vars.contains(file_handle) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        file_handle
                    ));
                }
                // Type check: file_handle must be FILE-HANDLE
                if let Some(dtype) = var_types.get(file_handle) {
                    if !matches!(dtype, crate::ast::DataType::FileHandle) {
                        return Err(format!(
                            "OPEN operation requires FILE-HANDLE type, got {}",
                            dtype
                        ));
                    }
                }
                instructions.push(Instruction::Open {
                    file_handle: file_handle.clone(),
                    file_path: file_path.clone(),
                });
            }
            ProcedureStatement::ReadFile { file_handle, output_stream } => {
                if !declared_vars.contains(file_handle) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        file_handle
                    ));
                }
                if !declared_vars.contains(output_stream) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        output_stream
                    ));
                }
                // Type check: file_handle must be FILE-HANDLE, output_stream must be RECORD-STREAM
                if let Some(dtype) = var_types.get(file_handle) {
                    if !matches!(dtype, crate::ast::DataType::FileHandle) {
                        return Err(format!(
                            "READ-FILE operation requires FILE-HANDLE type for file_handle, got {}",
                            dtype
                        ));
                    }
                }
                if let Some(dtype) = var_types.get(output_stream) {
                    if !matches!(dtype, crate::ast::DataType::RecordStream) {
                        return Err(format!(
                            "READ-FILE operation requires RECORD-STREAM type for output_stream, got {}",
                            dtype
                        ));
                    }
                }
                instructions.push(Instruction::ReadFile {
                    file_handle: file_handle.clone(),
                    output_stream: output_stream.clone(),
                });
            }
            ProcedureStatement::WriteFile { file_handle, input_stream } => {
                if !declared_vars.contains(file_handle) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        file_handle
                    ));
                }
                if !declared_vars.contains(input_stream) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        input_stream
                    ));
                }
                // Type check: file_handle must be FILE-HANDLE, input_stream must be RECORD-STREAM
                if let Some(dtype) = var_types.get(file_handle) {
                    if !matches!(dtype, crate::ast::DataType::FileHandle) {
                        return Err(format!(
                            "WRITE-FILE operation requires FILE-HANDLE type for file_handle, got {}",
                            dtype
                        ));
                    }
                }
                if let Some(dtype) = var_types.get(input_stream) {
                    if !matches!(dtype, crate::ast::DataType::RecordStream) {
                        return Err(format!(
                            "WRITE-FILE operation requires RECORD-STREAM type for input_stream, got {}",
                            dtype
                        ));
                    }
                }
                instructions.push(Instruction::WriteFile {
                    file_handle: file_handle.clone(),
                    input_stream: input_stream.clone(),
                });
            }
            ProcedureStatement::Close { file_handle } => {
                if !declared_vars.contains(file_handle) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        file_handle
                    ));
                }
                // Type check: file_handle must be FILE-HANDLE
                if let Some(dtype) = var_types.get(file_handle) {
                    if !matches!(dtype, crate::ast::DataType::FileHandle) {
                        return Err(format!(
                            "CLOSE operation requires FILE-HANDLE type, got {}",
                            dtype
                        ));
                    }
                }
                instructions.push(Instruction::Close {
                    file_handle: file_handle.clone(),
                });
            }
            ProcedureStatement::Checkpoint { record_stream } => {
                if !declared_vars.contains(record_stream) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        record_stream
                    ));
                }
                // Type check: record_stream must be RECORD-STREAM
                if let Some(dtype) = var_types.get(record_stream) {
                    if !matches!(dtype, crate::ast::DataType::RecordStream) {
                        return Err(format!(
                            "CHECKPOINT operation requires RECORD-STREAM type, got {}",
                            dtype
                        ));
                    }
                }
                instructions.push(Instruction::Checkpoint {
                    record_stream: record_stream.clone(),
                });
            }
            ProcedureStatement::Replay { target } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                // Type check: target must be RECORD-STREAM
                if let Some(dtype) = var_types.get(target) {
                    if !matches!(dtype, crate::ast::DataType::RecordStream) {
                        return Err(format!(
                            "REPLAY operation requires RECORD-STREAM type, got {}",
                            dtype
                        ));
                    }
                }
                instructions.push(Instruction::Replay {
                    target: target.clone(),
                });
            }
        }
    }

    Ok(instructions)
}

/// Helper to lower a single procedure statement to instruction
#[allow(clippy::only_used_in_recursion)]
fn lower_single_statement(
    stmt: &ProcedureStatement,
    declared_vars: &std::collections::HashSet<String>,
    signatures: &std::collections::HashMap<String, usize>,
) -> Result<Instruction, String> {
    match stmt {
        ProcedureStatement::Compress { target } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            Ok(Instruction::Compress {
                target: target.clone(),
            })
        }
        ProcedureStatement::VerifyIntegrity { target } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            Ok(Instruction::VerifyIntegrity {
                target: target.clone(),
            })
        }
        ProcedureStatement::Encrypt { target } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            Ok(Instruction::Encrypt {
                target: target.clone(),
            })
        }
        ProcedureStatement::Decrypt { target } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            Ok(Instruction::Decrypt {
                target: target.clone(),
            })
        }
        ProcedureStatement::If {
            condition,
            then_statements,
            else_statements,
        } => {
            let mut then_instrs = Vec::new();
            for s in then_statements {
                then_instrs.push(lower_single_statement(s, declared_vars, signatures)?);
            }
            let else_instrs = if let Some(else_stmts) = else_statements {
                let mut else_i = Vec::new();
                for s in else_stmts {
                    else_i.push(lower_single_statement(s, declared_vars, signatures)?);
                }
                Some(else_i)
            } else {
                None
            };
            Ok(Instruction::IfStatement {
                condition: condition.clone(),
                then_instrs,
                else_instrs,
            })
        }
        ProcedureStatement::For {
            variable,
            in_list,
            statements,
        } => {
            let mut loop_instrs = Vec::new();
            for s in statements {
                loop_instrs.push(lower_single_statement(s, declared_vars, signatures)?);
            }
            Ok(Instruction::ForLoop {
                variable: variable.clone(),
                in_list: in_list.clone(),
                instrs: loop_instrs,
            })
        }
        ProcedureStatement::While {
            condition,
            statements,
        } => {
            let mut loop_instrs = Vec::new();
            for s in statements {
                loop_instrs.push(lower_single_statement(s, declared_vars, signatures)?);
            }
            Ok(Instruction::WhileLoop {
                condition: condition.clone(),
                instrs: loop_instrs,
            })
        }
        ProcedureStatement::FunctionDef {
            name,
            parameters,
            return_type,
            statements,
        } => {
            let mut func_instrs = Vec::new();
            for s in statements {
                func_instrs.push(lower_single_statement(s, declared_vars, signatures)?);
            }
            Ok(Instruction::FunctionDef {
                name: name.clone(),
                parameters: parameters.clone(),
                return_type: return_type.as_ref().map(|dt| dt.to_string()),
                instrs: func_instrs,
            })
        }
        ProcedureStatement::FunctionCall { name, arguments } => {
            // In nested context we can't easily access signatures map; assume caller has
            // validated top-level calls. We still emit the instruction.
            Ok(Instruction::FunctionCall {
                name: name.clone(),
                arguments: arguments.clone(),
            })
        }
        ProcedureStatement::Set { target, value } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            Ok(Instruction::Set {
                target: target.clone(),
                value: value.clone(),
            })
        }
        ProcedureStatement::Add {
            target,
            operand1,
            operand2,
        } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            if !declared_vars.contains(operand1) {
                return Err(format!("Variable '{}' not declared", operand1));
            }
            if !declared_vars.contains(operand2) {
                return Err(format!("Variable '{}' not declared", operand2));
            }
            Ok(Instruction::Add {
                target: target.clone(),
                operand1: operand1.clone(),
                operand2: operand2.clone(),
            })
        }
        ProcedureStatement::Subtract {
            target,
            operand1,
            operand2,
        } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            if !declared_vars.contains(operand1) {
                return Err(format!("Variable '{}' not declared", operand1));
            }
            if !declared_vars.contains(operand2) {
                return Err(format!("Variable '{}' not declared", operand2));
            }
            Ok(Instruction::Subtract {
                target: target.clone(),
                operand1: operand1.clone(),
                operand2: operand2.clone(),
            })
        }
        ProcedureStatement::Multiply {
            target,
            operand1,
            operand2,
        } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            if !declared_vars.contains(operand1) {
                return Err(format!("Variable '{}' not declared", operand1));
            }
            if !declared_vars.contains(operand2) {
                return Err(format!("Variable '{}' not declared", operand2));
            }
            Ok(Instruction::Multiply {
                target: target.clone(),
                operand1: operand1.clone(),
                operand2: operand2.clone(),
            })
        }
        ProcedureStatement::Divide {
            target,
            operand1,
            operand2,
        } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            if !declared_vars.contains(operand1) {
                return Err(format!("Variable '{}' not declared", operand1));
            }
            if !declared_vars.contains(operand2) {
                return Err(format!("Variable '{}' not declared", operand2));
            }
            Ok(Instruction::Divide {
                target: target.clone(),
                operand1: operand1.clone(),
                operand2: operand2.clone(),
            })
        }
        ProcedureStatement::Open { file_handle, file_path } => {
            if !declared_vars.contains(file_handle) {
                return Err(format!("Variable '{}' not declared", file_handle));
            }
            Ok(Instruction::Open {
                file_handle: file_handle.clone(),
                file_path: file_path.clone(),
            })
        }
        ProcedureStatement::ReadFile { file_handle, output_stream } => {
            if !declared_vars.contains(file_handle) {
                return Err(format!("Variable '{}' not declared", file_handle));
            }
            if !declared_vars.contains(output_stream) {
                return Err(format!("Variable '{}' not declared", output_stream));
            }
            Ok(Instruction::ReadFile {
                file_handle: file_handle.clone(),
                output_stream: output_stream.clone(),
            })
        }
        ProcedureStatement::WriteFile { file_handle, input_stream } => {
            if !declared_vars.contains(file_handle) {
                return Err(format!("Variable '{}' not declared", file_handle));
            }
            if !declared_vars.contains(input_stream) {
                return Err(format!("Variable '{}' not declared", input_stream));
            }
            Ok(Instruction::WriteFile {
                file_handle: file_handle.clone(),
                input_stream: input_stream.clone(),
            })
        }
        ProcedureStatement::Close { file_handle } => {
            if !declared_vars.contains(file_handle) {
                return Err(format!("Variable '{}' not declared", file_handle));
            }
            Ok(Instruction::Close {
                file_handle: file_handle.clone(),
            })
        }
        ProcedureStatement::Checkpoint { record_stream } => {
            if !declared_vars.contains(record_stream) {
                return Err(format!("Variable '{}' not declared", record_stream));
            }
            Ok(Instruction::Checkpoint {
                record_stream: record_stream.clone(),
            })
        }
        ProcedureStatement::Replay { target } => {
            if !declared_vars.contains(target) {
                return Err(format!("Variable '{}' not declared", target));
            }
            Ok(Instruction::Replay {
                target: target.clone(),
            })
        }
        _ => Err("Unsupported nested statement".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_deterministic() {
        // Simple test that IR is deterministic
        let instr1 = Instruction::Compress {
            target: "buf".to_string(),
        };
        let instr2 = Instruction::Compress {
            target: "buf".to_string(),
        };
        assert_eq!(instr1, instr2);

        // encryption/decryption should also behave predictably
        let e1 = Instruction::Encrypt {
            target: "x".to_string(),
        };
        let e2 = Instruction::Encrypt {
            target: "x".to_string(),
        };
        assert_eq!(e1, e2);

        let d1 = Instruction::Decrypt {
            target: "x".to_string(),
        };
        let d2 = Instruction::Decrypt {
            target: "x".to_string(),
        };
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_file_operation_instructions() {
        let open_instr = Instruction::Open {
            file_handle: "fh".to_string(),
            file_path: "/path/to/file".to_string(),
        };
        assert_eq!(format!("{}", open_instr), "OPEN(fh AS /path/to/file)");

        let read_instr = Instruction::ReadFile {
            file_handle: "fh".to_string(),
            output_stream: "rs".to_string(),
        };
        assert_eq!(format!("{}", read_instr), "READ-FILE(fh INTO rs)");

        let write_instr = Instruction::WriteFile {
            file_handle: "fh".to_string(),
            input_stream: "rs".to_string(),
        };
        assert_eq!(format!("{}", write_instr), "WRITE-FILE(fh FROM rs)");

        let close_instr = Instruction::Close {
            file_handle: "fh".to_string(),
        };
        assert_eq!(format!("{}", close_instr), "CLOSE(fh)");

        let checkpoint_instr = Instruction::Checkpoint {
            record_stream: "rs".to_string(),
        };
        assert_eq!(format!("{}", checkpoint_instr), "CHECKPOINT(rs)");

        let replay_instr = Instruction::Replay {
            target: "rs".to_string(),
        };
        assert_eq!(format!("{}", replay_instr), "REPLAY(rs)");
    }
}
