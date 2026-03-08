use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AssertionKind {
    PreCondition,
    PostCondition,
    Invariant,
    Prove,
    Assert,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Plain,
    Verified,
    Encrypted,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
    True,
    False,
    BufferNonEmpty {
        buffer: String,
    },
    BufferLength {
        buffer: String,
        op: CmpOp,
        value: usize,
    },
    SecurityType {
        buffer: String,
        expected: SecurityLevel,
    },
    NumericBound {
        variable: String,
        op: CmpOp,
        value: i64,
    },
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
    Not(Box<Predicate>),
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Predicate::True => write!(f, "true"),
            Predicate::False => write!(f, "false"),
            Predicate::BufferNonEmpty { buffer } => write!(f, "(> (str.len {}) 0)", buffer),
            Predicate::BufferLength { buffer, op, value } => {
                let op_str = match op {
                    CmpOp::Eq => "=",
                    CmpOp::Ne => "!=",
                    CmpOp::Lt => "<",
                    CmpOp::Le => "<=",
                    CmpOp::Gt => ">",
                    CmpOp::Ge => ">=",
                };
                write!(f, "({} (str.len {}) {})", op_str, buffer, value)
            }
            Predicate::SecurityType { buffer, expected } => {
                let level_str = match expected {
                    SecurityLevel::Plain => "plain",
                    SecurityLevel::Verified => "verified",
                    SecurityLevel::Encrypted => "encrypted",
                };
                write!(f, "(= (security-level {}) {})", buffer, level_str)
            }
            Predicate::NumericBound {
                variable,
                op,
                value,
            } => {
                let op_str = match op {
                    CmpOp::Eq => "=",
                    CmpOp::Ne => "!=",
                    CmpOp::Lt => "<",
                    CmpOp::Le => "<=",
                    CmpOp::Gt => ">",
                    CmpOp::Ge => ">=",
                };
                write!(f, "({} {} {})", op_str, variable, value)
            }
            Predicate::And(a, b) => write!(f, "(and {} {})", a, b),
            Predicate::Or(a, b) => write!(f, "(or {} {})", a, b),
            Predicate::Not(p) => write!(f, "(not {})", p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HoareAnnotation {
    pub kind: AssertionKind,
    pub predicate: Predicate,
    pub source_location: String, // format "line:col"
}
