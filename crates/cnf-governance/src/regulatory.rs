use serde::{Serialize, Deserialize};

/// Supported standards
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Standard {
    SOC2,
    PciDss,
    HIPAA,
    GDPR,
    ISO27001,
}

impl Default for Standard {
    fn default() -> Self {
        Standard::SOC2
    }
}

/// Collection of regulations for a given standard
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RegulationSet {
    pub standard: Standard,
    pub clauses: Vec<String>,
}

impl RegulationSet {
    pub fn new(std: Standard) -> Self {
        RegulationSet { standard: std, clauses: Vec::new() }
    }

    pub fn add_clause(&mut self, clause: String) {
        self.clauses.push(clause);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_clause_increases_len() {
        let mut r = RegulationSet::new(Standard::GDPR);
        assert_eq!(r.clauses.len(), 0);
        r.add_clause("no export".into());
        assert_eq!(r.clauses.len(), 1);
    }

    #[test]
    fn serialize_standard() {
        let s = serde_json::to_string(&Standard::HIPAA).unwrap();
        assert!(s.contains("HIPAA"));
    }

    #[test]
    fn default_standard_is_soc2() {
        let def: Standard = Default::default();
        assert_eq!(def, Standard::SOC2);
    }

    #[test]
    fn add_multiple_clauses() {
        let mut r = RegulationSet::new(Standard::ISO27001);
        r.add_clause("c1".into());
        r.add_clause("c2".into());
        assert_eq!(r.clauses.len(), 2);
    }

    #[test]
    fn json_roundtrip() {
        let r = RegulationSet::new(Standard::GDPR);
        let s = serde_json::to_string(&r).unwrap();
        let _ : RegulationSet = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn default_regulation_set() {
        let r: RegulationSet = Default::default();
        assert_eq!(r.standard, Standard::SOC2);
        assert!(r.clauses.is_empty());
    }
}
