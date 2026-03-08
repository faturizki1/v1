use cnf_verifier::{
    AssertionKind, BufferState, CmpOp, CnfVerifierError, HoareAnnotation, HoareContext,
    HoareTriple, Predicate, SecurityLevel, VerificationResult, Verifier, Z3Config, Z3Solver,
};

#[test]
fn test_predicate_true_verifies_to_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let ctx = HoareContext::new();
    let result = solver.verify_predicate(&Predicate::True, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_predicate_false_verifies_to_refuted() {
    let solver = Z3Solver::new(Z3Config::default());
    let ctx = HoareContext::new();
    let result = solver.verify_predicate(&Predicate::False, &ctx).unwrap();
    assert_eq!(
        result,
        VerificationResult::Refuted {
            counterexample: "predicate false does not hold in context".to_string(),
        }
    );
}

#[test]
fn test_predicate_and_true_false_refuted() {
    let solver = Z3Solver::new(Z3Config::default());
    let ctx = HoareContext::new();
    let pred = Predicate::And(Box::new(Predicate::True), Box::new(Predicate::False));
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(
        result,
        VerificationResult::Refuted {
            counterexample: "predicate (and true false) does not hold in context".to_string()
        }
    );
}

#[test]
fn test_predicate_not_false_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let ctx = HoareContext::new();
    let pred = Predicate::Not(Box::new(Predicate::False));
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_buffer_non_empty_unknown_stub() {
    let solver = Z3Solver::new(Z3Config::default());
    let ctx = HoareContext::new();
    let pred = Predicate::BufferNonEmpty {
        buffer: "buf".to_string(),
    };
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(
        result,
        VerificationResult::Refuted {
            counterexample: "predicate (> (str.len buf) 0) does not hold in context".to_string()
        }
    );
}

#[test]
fn test_predicate_display_true() {
    let pred = Predicate::True;
    assert_eq!(format!("{}", pred), "true");
}

#[test]
fn test_predicate_display_false() {
    let pred = Predicate::False;
    assert_eq!(format!("{}", pred), "false");
}

#[test]
fn test_predicate_display_buffer_non_empty() {
    let pred = Predicate::BufferNonEmpty {
        buffer: "mybuf".to_string(),
    };
    assert_eq!(format!("{}", pred), "(> (str.len mybuf) 0)");
}

#[test]
fn test_predicate_display_buffer_length() {
    let pred = Predicate::BufferLength {
        buffer: "buf".to_string(),
        op: CmpOp::Gt,
        value: 10,
    };
    assert_eq!(format!("{}", pred), "(> (str.len buf) 10)");
}

#[test]
fn test_predicate_display_security_type() {
    let pred = Predicate::SecurityType {
        buffer: "buf".to_string(),
        expected: SecurityLevel::Encrypted,
    };
    assert_eq!(format!("{}", pred), "(= (security-level buf) encrypted)");
}

#[test]
fn test_predicate_display_numeric_bound() {
    let pred = Predicate::NumericBound {
        variable: "x".to_string(),
        op: CmpOp::Eq,
        value: 42,
    };
    assert_eq!(format!("{}", pred), "(= x 42)");
}

#[test]
fn test_predicate_display_and() {
    let pred = Predicate::And(Box::new(Predicate::True), Box::new(Predicate::False));
    assert_eq!(format!("{}", pred), "(and true false)");
}

#[test]
fn test_predicate_display_or() {
    let pred = Predicate::Or(Box::new(Predicate::True), Box::new(Predicate::False));
    assert_eq!(format!("{}", pred), "(or true false)");
}

#[test]
fn test_predicate_display_not() {
    let pred = Predicate::Not(Box::new(Predicate::True));
    assert_eq!(format!("{}", pred), "(not true)");
}

#[test]
fn test_hoare_context_add_annotation_and_collect_triples() {
    let mut ctx = HoareContext::new();
    ctx.add_annotation(HoareAnnotation {
        kind: AssertionKind::PreCondition,
        predicate: Predicate::True,
        source_location: "1:1".to_string(),
    });
    ctx.add_annotation(HoareAnnotation {
        kind: AssertionKind::PostCondition,
        predicate: Predicate::True,
        source_location: "2:1".to_string(),
    });
    let triples = ctx.collect_triples();
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].pre, Predicate::True);
    assert_eq!(triples[0].post, Predicate::True);
}

#[test]
fn test_hoare_context_set_buffer_state() {
    let mut ctx = HoareContext::new();
    let state = BufferState {
        length: 100,
        security_level: SecurityLevel::Encrypted,
        is_empty: false,
    };
    ctx.set_buffer_state("buf".to_string(), state.clone());
    assert_eq!(ctx.buffer_states.get("buf"), Some(&state));
}

#[test]
fn test_verifier_verify_all_with_true_triples() {
    let mut verifier = Verifier::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.add_annotation(HoareAnnotation {
        kind: AssertionKind::PreCondition,
        predicate: Predicate::True,
        source_location: "1:1".to_string(),
    });
    ctx.add_annotation(HoareAnnotation {
        kind: AssertionKind::PostCondition,
        predicate: Predicate::True,
        source_location: "2:1".to_string(),
    });
    verifier.load_context(ctx);
    let results = verifier.verify_all().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], VerificationResult::Proved);
}

#[test]
fn test_cnf_verifier_error_precondition_failed() {
    let err = CnfVerifierError::PreconditionFailed {
        procedure: "test_proc".to_string(),
        predicate: "false".to_string(),
    };
    assert!(format!("{}", err).contains("L7.001.F"));
}

#[test]
fn test_cnf_verifier_error_postcondition_failed() {
    let err = CnfVerifierError::PostconditionFailed {
        procedure: "test_proc".to_string(),
        predicate: "false".to_string(),
    };
    assert!(format!("{}", err).contains("L7.002.F"));
}

#[test]
fn test_cnf_verifier_error_invariant_violated() {
    let err = CnfVerifierError::InvariantViolated {
        location: "1:1".to_string(),
        predicate: "false".to_string(),
    };
    assert!(format!("{}", err).contains("L7.003.F"));
}

#[test]
fn test_cnf_verifier_error_z3_timeout() {
    let err = CnfVerifierError::Z3Timeout {
        predicate: "complex".to_string(),
        timeout_ms: 30000,
    };
    assert!(format!("{}", err).contains("L7.004.E"));
}

#[test]
fn test_cnf_verifier_error_z3_parse_error() {
    let err = CnfVerifierError::Z3ParseError {
        expression: "invalid".to_string(),
        reason: "syntax error".to_string(),
    };
    assert!(format!("{}", err).contains("L7.005.E"));
}

#[test]
fn test_cnf_verifier_error_proof_not_found() {
    let err = CnfVerifierError::ProofNotFound {
        theorem: "unsolvable".to_string(),
    };
    assert!(format!("{}", err).contains("L7.006.E"));
}

#[test]
fn test_cnf_verifier_error_audit_chain_broken() {
    let err = CnfVerifierError::AuditChainBroken {
        entry_seq: 42,
        reason: "tampered".to_string(),
    };
    assert!(format!("{}", err).contains("L7.007.E"));
}

#[test]
fn test_z3_solver_buffer_non_empty_with_ctx_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 10,
            security_level: SecurityLevel::Plain,
            is_empty: false,
        },
    );
    let pred = Predicate::BufferNonEmpty {
        buffer: "buf".to_string(),
    };
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_z3_solver_buffer_non_empty_without_ctx_refuted() {
    let solver = Z3Solver::new(Z3Config::default());
    let ctx = HoareContext::new();
    let pred = Predicate::BufferNonEmpty {
        buffer: "buf".to_string(),
    };
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(
        result,
        VerificationResult::Refuted {
            counterexample: "predicate (> (str.len buf) 0) does not hold in context".to_string(),
        }
    );
}

#[test]
fn test_z3_solver_buffer_length_gt_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 20,
            security_level: SecurityLevel::Plain,
            is_empty: false,
        },
    );
    let pred = Predicate::BufferLength {
        buffer: "buf".to_string(),
        op: CmpOp::Gt,
        value: 10,
    };
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_z3_solver_buffer_length_lt_refuted() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 5,
            security_level: SecurityLevel::Plain,
            is_empty: false,
        },
    );
    let pred = Predicate::BufferLength {
        buffer: "buf".to_string(),
        op: CmpOp::Gt,
        value: 10,
    };
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(
        result,
        VerificationResult::Refuted {
            counterexample: "predicate (> (str.len buf) 10) does not hold in context".to_string(),
        }
    );
}

#[test]
fn test_z3_solver_security_type_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 10,
            security_level: SecurityLevel::Encrypted,
            is_empty: false,
        },
    );
    let pred = Predicate::SecurityType {
        buffer: "buf".to_string(),
        expected: SecurityLevel::Encrypted,
    };
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_z3_solver_and_true_false_refuted() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf1".to_string(),
        BufferState {
            length: 10,
            security_level: SecurityLevel::Plain,
            is_empty: false,
        },
    );
    ctx.set_buffer_state(
        "buf2".to_string(),
        BufferState {
            length: 0,
            security_level: SecurityLevel::Plain,
            is_empty: true,
        },
    );
    let pred = Predicate::And(
        Box::new(Predicate::BufferNonEmpty {
            buffer: "buf1".to_string(),
        }),
        Box::new(Predicate::BufferNonEmpty {
            buffer: "buf2".to_string(),
        }),
    );
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(
        result,
        VerificationResult::Refuted {
            counterexample:
                "predicate (and (> (str.len buf1) 0) (> (str.len buf2) 0)) does not hold in context"
                    .to_string(),
        }
    );
}

#[test]
fn test_z3_solver_or_true_false_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf1".to_string(),
        BufferState {
            length: 10,
            security_level: SecurityLevel::Plain,
            is_empty: false,
        },
    );
    ctx.set_buffer_state(
        "buf2".to_string(),
        BufferState {
            length: 0,
            security_level: SecurityLevel::Plain,
            is_empty: true,
        },
    );
    let pred = Predicate::Or(
        Box::new(Predicate::BufferNonEmpty {
            buffer: "buf1".to_string(),
        }),
        Box::new(Predicate::BufferNonEmpty {
            buffer: "buf2".to_string(),
        }),
    );
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_z3_solver_not_false_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 0,
            security_level: SecurityLevel::Plain,
            is_empty: true,
        },
    );
    let pred = Predicate::Not(Box::new(Predicate::BufferNonEmpty {
        buffer: "buf".to_string(),
    }));
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_z3_solver_numeric_bound_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 15,
            security_level: SecurityLevel::Plain,
            is_empty: false,
        },
    );
    let pred = Predicate::NumericBound {
        variable: "buf".to_string(),
        op: CmpOp::Ge,
        value: 10,
    };
    let result = solver.verify_predicate(&pred, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}

#[test]
fn test_z3_solver_triple_precondition_failed() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 0,
            security_level: SecurityLevel::Plain,
            is_empty: true,
        },
    );
    let triple = HoareTriple {
        pre: Predicate::BufferNonEmpty {
            buffer: "buf".to_string(),
        },
        body_description: "test procedure".to_string(),
        post: Predicate::True,
    };
    let result = solver.verify_triple(&triple, &ctx);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("L7.001.F"));
    assert!(format!("{}", err).contains("PreconditionFailed"));
}

#[test]
fn test_z3_solver_triple_proved() {
    let solver = Z3Solver::new(Z3Config::default());
    let mut ctx = HoareContext::new();
    ctx.set_buffer_state(
        "buf".to_string(),
        BufferState {
            length: 10,
            security_level: SecurityLevel::Plain,
            is_empty: false,
        },
    );
    let triple = HoareTriple {
        pre: Predicate::BufferNonEmpty {
            buffer: "buf".to_string(),
        },
        body_description: "test procedure".to_string(),
        post: Predicate::True,
    };
    let result = solver.verify_triple(&triple, &ctx).unwrap();
    assert_eq!(result, VerificationResult::Proved);
}
