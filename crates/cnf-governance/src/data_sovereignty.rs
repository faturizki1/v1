use crate::error::CnfGovernanceError;

#[derive(Debug, PartialEq)]
pub enum Region {
    EU,
    US,
    APAC,
    OTHER(String),
}

pub struct SovereigntyChecker;

impl SovereigntyChecker {
    pub fn new() -> Self { SovereigntyChecker }

    pub fn validate_transfer(&self, from: &Region, to: &Region) -> Result<bool, CnfGovernanceError> {
        // dummy: disallow EU -> US for example
        if from == &Region::EU && to == &Region::US {
            Err(CnfGovernanceError::SovereigntyBreach("EU->US forbidden".into()))
        } else {
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approve_same_region() {
        let c = SovereigntyChecker::new();
        assert_eq!(c.validate_transfer(&Region::EU, &Region::EU).unwrap(), true);
    }

    #[test]
    fn disallow_eu_us() {
        let c = SovereigntyChecker::new();
        let res = c.validate_transfer(&Region::EU, &Region::US);
        assert!(res.is_err());
    }

    #[test]
    fn other_regions_allowed() {
        let c = SovereigntyChecker::new();
        assert!(c.validate_transfer(&Region::US, &Region::APAC).unwrap());
    }

    #[test]
    fn region_equality() {
        assert_eq!(Region::US, Region::US);
    }

    #[test]
    fn other_region_custom() {
        let r = Region::OTHER("Antarctica".into());
        match r {
            Region::OTHER(name) => assert_eq!(name, "Antarctica"),
            _ => panic!("wrong variant"),
        }
    }
}
