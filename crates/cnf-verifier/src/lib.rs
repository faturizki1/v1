pub mod assertion;
pub mod audit_chain;
pub mod error;
pub mod hoare;
pub mod z3_bridge;

pub use assertion::{AssertionKind, CmpOp, HoareAnnotation, Predicate, SecurityLevel};
pub use audit_chain::AuditChain;
pub use error::CnfVerifierError;
pub use hoare::{BufferState, HoareContext, HoareTriple};
pub use z3_bridge::{VerificationResult, Z3Config, Z3Solver};

pub struct Verifier {
    solver: Z3Solver,
    context: HoareContext,
}

impl Verifier {
    pub fn new(config: Z3Config) -> Self {
        Self {
            solver: Z3Solver::new(config),
            context: HoareContext::new(),
        }
    }

    pub fn load_context(&mut self, ctx: HoareContext) -> &mut Self {
        self.context = ctx;
        self
    }

    pub fn verify_all(&mut self) -> Result<Vec<VerificationResult>, CnfVerifierError> {
        let triples = self.context.collect_triples();
        let mut results = Vec::new();
        for triple in &triples {
            let result = self.solver.verify_triple(triple, &self.context)?;
            results.push(result);
        }
        Ok(results)
    }
}
