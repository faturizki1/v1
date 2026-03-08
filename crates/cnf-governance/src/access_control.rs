use crate::error::CnfGovernanceError;

pub struct AccessControl;

impl AccessControl {
    pub fn check(&self, user: &str, resource: &str) -> Result<bool, CnfGovernanceError> {
        if user == "admin" {
            Ok(true)
        } else {
            Err(CnfGovernanceError::AccessDenied(format!("{} denied {}", user, resource)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_allowed() {
        let ac = AccessControl;
        assert!(ac.check("admin", "res").unwrap());
    }

    #[test]
    fn user_denied() {
        let ac = AccessControl;
        assert!(ac.check("bob", "res").is_err());
    }

    #[test]
    fn multiple_checks() {
        let ac = AccessControl;
        let _ = ac.check("admin", "x");
        let err = ac.check("user", "y");
        assert!(err.is_err());
    }

    #[test]
    fn check_nonexistent_user() {
        let ac = AccessControl;
        let res = ac.check("", "res");
        assert!(res.is_err());
    }
}
