use thiserror::Error;

#[derive(Error, Debug)]
pub enum CnfVerifierError {
    #[error(
        "L7.001.F PreconditionFailed: procedure '{procedure}' failed precondition '{predicate}'"
    )]
    PreconditionFailed {
        procedure: String,
        predicate: String,
    },

    #[error(
        "L7.002.F PostconditionFailed: procedure '{procedure}' failed postcondition '{predicate}'"
    )]
    PostconditionFailed {
        procedure: String,
        predicate: String,
    },

    #[error("L7.003.F InvariantViolated: at '{location}' invariant '{predicate}' violated")]
    InvariantViolated { location: String, predicate: String },

    #[error("L7.004.E Z3Timeout: predicate '{predicate}' timed out after {timeout_ms}ms")]
    Z3Timeout { predicate: String, timeout_ms: u64 },

    #[error("L7.005.E Z3ParseError: failed to parse expression '{expression}': {reason}")]
    Z3ParseError { expression: String, reason: String },

    #[error("L7.006.E ProofNotFound: no proof found for theorem '{theorem}'")]
    ProofNotFound { theorem: String },

    #[error("L7.007.E AuditChainBroken: audit chain broken at entry {entry_seq}: {reason}")]
    AuditChainBroken { entry_seq: u64, reason: String },

    #[error("L7.009.E AuditChainError: audit chain operation failed: {message}")]
    AuditChainError { message: String },

    #[error("L7.008.E SmtEncodingFailed: SMT encoding failed: {reason}")]
    SmtEncodingFailed { reason: String },
}
