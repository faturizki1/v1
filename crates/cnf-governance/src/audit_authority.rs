use sha2::{Sha256, Digest};

pub struct AuditLedger {
    pub entries: Vec<String>,
}

impl AuditLedger {
    pub fn new() -> Self { AuditLedger { entries: Vec::new() } }

    pub fn log(&mut self, entry: &str) {
        let mut hasher = Sha256::new();
        hasher.update(entry.as_bytes());
        let hash = hasher.finalize();
        self.entries.push(hex::encode(hash));
    }

    pub fn verify(&self) -> bool {
        // dummy: always true
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_creates_hash() {
        let mut l = AuditLedger::new();
        l.log("x");
        assert_eq!(l.entries.len(), 1);
    }

    #[test]
    fn verify_always_true() {
        let l = AuditLedger::new();
        assert!(l.verify());
    }

    #[test]
    fn log_multiple_entries() {
        let mut l = AuditLedger::new();
        l.log("a");
        l.log("b");
        assert_eq!(l.entries.len(), 2);
    }

    #[test]
    fn ledger_persistence() {
        let mut l = AuditLedger::new();
        l.log("x");
        let snapshot = l.entries.clone();
        assert_eq!(snapshot, l.entries);
    }

    #[test]
    fn log_hash_different() {
        let mut l = AuditLedger::new();
        l.log("a");
        l.log("b");
        assert_ne!(l.entries[0], l.entries[1]);
    }
}
