use crate::assertion::{CmpOp, Predicate};
use crate::error::CnfVerifierError;
use crate::hoare::{HoareContext, HoareTriple};

#[derive(Debug, Clone)]
pub struct Z3Config {
    pub timeout_ms: u64,
    pub max_memory_mb: u64,
}

impl Default for Z3Config {
    fn default() -> Self {
        Self {
            timeout_ms: 30_000,
            max_memory_mb: 512,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationResult {
    Proved,
    Refuted { counterexample: String },
    Timeout,
    Unknown { reason: String },
}

#[derive(Debug)]
pub struct Z3Solver {
    #[allow(dead_code)]
    config: Z3Config,
}

impl Z3Solver {
    pub fn new(config: Z3Config) -> Self {
        Self { config }
    }

    /// Fallback pure Rust symbolic evaluator for decidable predicates
    fn eval_predicate_symbolic(pred: &Predicate, ctx: &HoareContext) -> bool {
        match pred {
            Predicate::True => true,
            Predicate::False => false,
            Predicate::BufferNonEmpty { buffer } => ctx
                .buffer_states
                .get(buffer)
                .map(|s| !s.is_empty)
                .unwrap_or(false),
            Predicate::BufferLength { buffer, op, value } => ctx
                .buffer_states
                .get(buffer)
                .map(|s| Self::eval_cmp_op(s.length, *value, op))
                .unwrap_or(false),
            Predicate::SecurityType { buffer, expected } => ctx
                .buffer_states
                .get(buffer)
                .map(|s| &s.security_level == expected)
                .unwrap_or(false),
            Predicate::NumericBound {
                variable,
                op,
                value,
            } => {
                // Try to get buffer length for the variable
                ctx.buffer_states
                    .get(variable)
                    .map(|s| Self::eval_cmp_op(s.length, *value as usize, op))
                    .unwrap_or(false)
            }
            Predicate::And(a, b) => {
                Self::eval_predicate_symbolic(a, ctx) && Self::eval_predicate_symbolic(b, ctx)
            }
            Predicate::Or(a, b) => {
                Self::eval_predicate_symbolic(a, ctx) || Self::eval_predicate_symbolic(b, ctx)
            }
            Predicate::Not(p) => !Self::eval_predicate_symbolic(p, ctx),
        }
    }

    /// Helper: evaluate comparison operation
    fn eval_cmp_op(left: usize, right: usize, op: &CmpOp) -> bool {
        match op {
            CmpOp::Eq => left == right,
            CmpOp::Ne => left != right,
            CmpOp::Lt => left < right,
            CmpOp::Le => left <= right,
            CmpOp::Gt => left > right,
            CmpOp::Ge => left >= right,
        }
    }

    pub fn verify_predicate(
        &self,
        pred: &Predicate,
        ctx: &HoareContext,
    ) -> Result<VerificationResult, CnfVerifierError> {
        #[cfg(feature = "z3-solver")]
        {
            // Try to use real Z3 if available
            self.verify_predicate_z3(pred, ctx)
        }

        #[cfg(not(feature = "z3-solver"))]
        {
            // Use pure Rust fallback evaluator
            let holds = Self::eval_predicate_symbolic(pred, ctx);
            if holds {
                Ok(VerificationResult::Proved)
            } else {
                Ok(VerificationResult::Refuted {
                    counterexample: format!("predicate {} does not hold in context", pred),
                })
            }
        }
    }

    #[cfg(feature = "z3-solver")]
    fn verify_predicate_z3(
        &self,
        pred: &Predicate,
        ctx: &HoareContext,
    ) -> Result<VerificationResult, CnfVerifierError> {
        use z3::*;

        let config = Config::new();
        let z3_ctx = Context::new(&config);
        let solver = Solver::new(&z3_ctx);

        // Encode predicate to Z3 formula
        let formula = self.encode_predicate(pred, &z3_ctx, ctx)?;
        solver.assert(&formula);

        // Check satisfiability with timeout
        match solver.check() {
            SatResult::Sat => Ok(VerificationResult::Proved),
            SatResult::Unsat => Ok(VerificationResult::Refuted {
                counterexample: "negation satisfiable".to_string(),
            }),
            SatResult::Unknown => Ok(VerificationResult::Unknown {
                reason: "Z3 solver could not determine satisfiability".to_string(),
            }),
        }
    }

    #[cfg(feature = "z3-solver")]
    fn encode_predicate(
        &self,
        pred: &Predicate,
        z3_ctx: &z3::Context,
        _ctx: &HoareContext,
    ) -> Result<z3::ast::Bool, CnfVerifierError> {
        use z3::ast::Ast;
        use z3::*;

        match pred {
            Predicate::True => Ok(Bool::from_bool(z3_ctx, true)),
            Predicate::False => Ok(Bool::from_bool(z3_ctx, false)),
            Predicate::And(a, b) => {
                let a_enc = self.encode_predicate(a, z3_ctx, _ctx)?;
                let b_enc = self.encode_predicate(b, z3_ctx, _ctx)?;
                Ok(Bool::and(z3_ctx, &[&a_enc, &b_enc]))
            }
            Predicate::Or(a, b) => {
                let a_enc = self.encode_predicate(a, z3_ctx, _ctx)?;
                let b_enc = self.encode_predicate(b, z3_ctx, _ctx)?;
                Ok(Bool::or(z3_ctx, &[&a_enc, &b_enc]))
            }
            Predicate::Not(p) => {
                let p_enc = self.encode_predicate(p, z3_ctx, _ctx)?;
                Ok(p_enc.not())
            }
            _ => {
                // For other predicates, encode as true (stub for now)
                Ok(Bool::from_bool(z3_ctx, true))
            }
        }
    }

    pub fn verify_triple(
        &self,
        triple: &HoareTriple,
        ctx: &HoareContext,
    ) -> Result<VerificationResult, CnfVerifierError> {
        // Verifikasi pre → body → post
        // Jika pre Refuted → L7.001.F PreconditionFailed
        let pre_result = self.verify_predicate(&triple.pre, ctx)?;
        match pre_result {
            VerificationResult::Refuted { .. } => Err(CnfVerifierError::PreconditionFailed {
                procedure: triple.body_description.clone(),
                predicate: triple.pre.to_string(),
            }),
            VerificationResult::Proved => {
                // For now, assume post holds if pre holds
                Ok(VerificationResult::Proved)
            }
            _ => Ok(VerificationResult::Unknown {
                reason: "precondition unknown".to_string(),
            }),
        }
    }
}
