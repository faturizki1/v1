use thiserror::Error;

#[derive(Debug, Error)]
pub enum CnfGovernanceError {
    #[error("L9.001.F: Invalid policy formula: {0}")]
    InvalidPolicyFormula(String),

    #[error("L9.101.I: Policy violation detected: {0}")]
    PolicyViolation(String),

    #[error("L9.201.N: Unknown regulation: {0}")]
    UnknownRegulation(String),

    #[error("L9.301.E: Data sovereignty breach: {0}")]
    SovereigntyBreach(String),

    #[error("L9.401.R: Access denied: {0}")]
    AccessDenied(String),

    #[error("L9.451.L: Audit ledger tampered: {0}")]
    LedgerTampered(String),

    #[error("L9.501.E: Consensus failure: {0}")]
    ConsensusFailure(String),

    #[error("L9.502.U: Unexpected governance error: {0}")]
    Unexpected(String),

    #[error("L9.503.I: Initialization error: {0}")]
    Initialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_formats() {
        let e = CnfGovernanceError::InvalidPolicyFormula("foo".to_string());
        assert!(format!("{}", e).contains("L9.001.F"));
    }

    #[test]
    fn multiple_error_variants() {
        let e = CnfGovernanceError::AccessDenied("x".into());
        assert!(format!("{}", e).contains("L9.401.R"));
    }

    #[test]
    fn error_debug() {
        let e = CnfGovernanceError::ConsensusFailure("y".into());
        let _ = format!("{:?}", e);
    }

    #[test]
    fn unexpected_error() {
        let e = CnfGovernanceError::Unexpected("z".into());
        assert!(matches!(e, CnfGovernanceError::Unexpected(_)));
    }

    #[test]
    fn initialization_error() {
        let e = CnfGovernanceError::Initialization("init".into());
        assert!(format!("{}", e).contains("L9.503.I"));
    }
}
