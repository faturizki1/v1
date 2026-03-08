use crate::error::CnfGovernanceError;

pub struct ConsensusQuorum {
    pub nodes: usize,
    pub agreement: usize,
}

impl ConsensusQuorum {
    pub fn new(nodes: usize) -> Self {
        let agreement = (nodes * 2 / 3) + 1;
        ConsensusQuorum { nodes, agreement }
    }

    pub fn check(&self, votes: usize) -> Result<bool, CnfGovernanceError> {
        if votes >= self.agreement {
            Ok(true)
        } else {
            Err(CnfGovernanceError::ConsensusFailure("not enough votes".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quorum_calculation() {
        let q = ConsensusQuorum::new(5);
        assert_eq!(q.agreement, 4);
    }

    #[test]
    fn quorum_check_success() {
        let q = ConsensusQuorum::new(3);
        assert!(q.check(3).unwrap());
    }

    #[test]
    fn quorum_check_fail() {
        let q = ConsensusQuorum::new(3);
        assert!(q.check(1).is_err());
    }

    #[test]
    fn quorum_edge_case() {
        let q = ConsensusQuorum::new(1);
        assert_eq!(q.agreement, 1);
        assert!(q.check(1).unwrap());
    }

    #[test]
    fn quorum_zero_nodes() {
        let q = ConsensusQuorum::new(0);
        assert_eq!(q.agreement, 1); // (0*2/3)+1 ==1
        assert!(q.check(0).is_err());
    }

    #[test]
    fn quorum_large_votes() {
        let q = ConsensusQuorum::new(10);
        assert!(q.check(10).unwrap());
    }
}
