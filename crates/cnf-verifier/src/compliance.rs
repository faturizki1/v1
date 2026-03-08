use crate::audit_chain::AuditChain;
use crate::CnfVerifierError;
use cnf_security::sha256_hex;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;

/// Standards supported by the compliance report generator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStandard {
    Soc2,
    PciDss,
    Hipaa,
}

impl std::fmt::Display for ComplianceStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplianceStandard::Soc2 => write!(f, "SOC2"),
            ComplianceStandard::PciDss => write!(f, "PCI-DSS"),
            ComplianceStandard::Hipaa => write!(f, "HIPAA"),
        }
    }
}

impl FromStr for ComplianceStandard {
    type Err = CnfVerifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "SOC2" => Ok(ComplianceStandard::Soc2),
            "PCI-DSS" | "PCIDSS" | "PCI" => Ok(ComplianceStandard::PciDss),
            "HIPAA" => Ok(ComplianceStandard::Hipaa),
            other => Err(CnfVerifierError::AuditChainError { // reuse existing variant
                message: format!("Unknown compliance standard '{}'", other),
            }),
        }
    }
}

/// Status for an individual control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlStatus {
    Passed,
    Failed,
    NotApplicable,
    NeedsReview,
}

impl std::fmt::Display for ControlStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlStatus::Passed => write!(f, "Passed"),
            ControlStatus::Failed => write!(f, "Failed"),
            ControlStatus::NotApplicable => write!(f, "Not Applicable"),
            ControlStatus::NeedsReview => write!(f, "Needs Review"),
        }
    }
}

/// Single control within a report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceControl {
    pub control_id: String,
    pub description: String,
    pub status: ControlStatus,
    pub evidence: Vec<String>,
}

/// Compliance report object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub standard: ComplianceStandard,
    pub generated_at_ms: u64,
    pub program_id: String,
    pub controls: Vec<ComplianceControl>,
    pub overall_status: ControlStatus,
    pub audit_chain_hash: String,
}

impl ComplianceReport {
    /// Generate a report for the given standard, using audit chain logs.
    pub fn generate(
        standard: ComplianceStandard,
        audit_chain: &AuditChain,
        program_id: &str,
    ) -> Self {
        let generated_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        // initialize controls based on standard
        let mut controls: Vec<ComplianceControl> = Vec::new();

        match standard {
            ComplianceStandard::Soc2 => {
                controls.push(ComplianceControl {
                    control_id: "CC6.1".to_string(),
                    description: "Integrity verification".to_string(),
                    status: ControlStatus::NeedsReview,
                    evidence: Vec::new(),
                });
                controls.push(ComplianceControl {
                    control_id: "ACCESS".to_string(),
                    description: "Access control".to_string(),
                    status: ControlStatus::NeedsReview,
                    evidence: Vec::new(),
                });
            }
            ComplianceStandard::PciDss => {
                controls.push(ComplianceControl {
                    control_id: "3.4".to_string(),
                    description: "Encryption at rest".to_string(),
                    status: ControlStatus::NeedsReview,
                    evidence: Vec::new(),
                });
                controls.push(ComplianceControl {
                    control_id: "ACCESS".to_string(),
                    description: "Access control".to_string(),
                    status: ControlStatus::NeedsReview,
                    evidence: Vec::new(),
                });
            }
            ComplianceStandard::Hipaa => {
                controls.push(ComplianceControl {
                    control_id: "§164.312".to_string(),
                    description: "Backup requirement".to_string(),
                    status: ControlStatus::NeedsReview,
                    evidence: Vec::new(),
                });
                controls.push(ComplianceControl {
                    control_id: "ACCESS".to_string(),
                    description: "Access control".to_string(),
                    status: ControlStatus::NeedsReview,
                    evidence: Vec::new(),
                });
            }
        }

        // map audit entries to controls
        for entry in audit_chain.entries().iter() {
            let msg = entry.message.to_uppercase();
            if msg.contains("ENCRYPT") {
                // PCI 3.4
                if let Some(ctrl) = controls.iter_mut().find(|c| c.control_id == "3.4") {
                    ctrl.evidence.push(entry.message.clone());
                    ctrl.status = ControlStatus::Passed;
                }
            }
            if msg.contains("VERIFY") {
                if let Some(ctrl) = controls.iter_mut().find(|c| c.control_id == "CC6.1") {
                    ctrl.evidence.push(entry.message.clone());
                    ctrl.status = ControlStatus::Passed;
                }
            }
            if msg.contains("CHECKPOINT") {
                if let Some(ctrl) = controls.iter_mut().find(|c| c.control_id == "§164.312") {
                    ctrl.evidence.push(entry.message.clone());
                    ctrl.status = ControlStatus::Passed;
                }
            }
            if msg.contains("ACCESS") {
                for ctrl in controls.iter_mut().filter(|c| c.control_id == "ACCESS") {
                    ctrl.evidence.push(entry.message.clone());
                    ctrl.status = ControlStatus::Passed;
                }
            }
        }

        // determine overall status
        let overall_status = if controls.iter().any(|c| c.status == ControlStatus::Failed) {
            ControlStatus::Failed
        } else if controls.iter().any(|c| c.status == ControlStatus::NeedsReview) {
            ControlStatus::NeedsReview
        } else {
            ControlStatus::Passed
        };

        // compute audit_chain_hash
        let mut concat_bytes = Vec::new();
        for entry in audit_chain.entries().iter() {
            concat_bytes.extend_from_slice(&entry.hmac);
        }
        let audit_chain_hash = sha256_hex(&concat_bytes);

        ComplianceReport {
            standard,
            generated_at_ms,
            program_id: program_id.to_string(),
            controls,
            overall_status,
            audit_chain_hash,
        }
    }

    /// Serialize report to compact JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Human-readable summary for display
    pub fn to_summary(&self) -> String {
        let mut s = format!(
            "Compliance {} report for {} at {}\n",
            self.standard, self.program_id, self.generated_at_ms
        );
        for ctrl in &self.controls {
            s.push_str(&format!(
                "{} ({}) - {} evidence entries\n",
                ctrl.control_id,
                ctrl.status,
                ctrl.evidence.len()
            ));
        }
        s.push_str(&format!("Overall: {}\nHash: {}", self.overall_status, self.audit_chain_hash));
        s
    }
}
