//! Circuit breaker pattern for fault tolerance
//!
//! Prevents cascading failures by stopping calls to failing services.
//! Implements state machine: Closed → Open (on failures) → HalfOpen (after timeout) → Closed
//!
//! State transitions:
//! - Closed: normal operation
//! - Open: too many failures, reject calls
//! - HalfOpen: allow test call after timeout
//!
//! All operations are pure except state transitions.

use crate::error::CnfNetworkError;
use std::time::{SystemTime, UNIX_EPOCH};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation, calls allowed
    Closed,
    /// Too many failures, calls rejected
    Open,
    /// Testing after timeout, calls allowed (one test call)
    HalfOpen,
}

/// Circuit breaker for fault tolerance
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    /// Current state
    state: CircuitState,
    /// Count of consecutive failures
    failure_count: u32,
    /// Failure threshold (default: 5)
    threshold: u32,
    /// Timeout to transition from Open to HalfOpen (milliseconds, default: 30000)
    reset_timeout_ms: u64,
    /// Timestamp of last failure (milliseconds since UNIX_EPOCH)
    last_failure_time: u64,
}

impl CircuitBreaker {
    /// Create new circuit breaker with custom threshold and timeout
    pub fn new(threshold: u32, reset_timeout_ms: u64) -> Self {
        CircuitBreaker {
            state: CircuitState::Closed,
            failure_count: 0,
            threshold,
            reset_timeout_ms,
            last_failure_time: 0,
        }
    }

    /// Get current state
    pub fn state(&self) -> CircuitState {
        self.state
    }

    /// Get failure count
    pub fn failure_count(&self) -> u32 {
        self.failure_count
    }

    /// Try to execute call (pure - returns whether call should attempt)
    pub fn should_allow_call(&self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if enough time has passed for reset attempt
                self.should_attempt_reset()
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// Record successful call
    pub fn on_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                // Closed stays closed
            }
            CircuitState::HalfOpen => {
                // Successful call in HalfOpen → transition to Closed
                self.reset();
            }
            CircuitState::Open => {
                // Shouldn't happen (should_allow_call checks timeout first)
            }
        }
    }

    /// Record failed call
    pub fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = current_time_ms();

        if self.failure_count >= self.threshold {
            self.trip();
        }
    }

    /// Trip the breaker (Open state due to threshold exceeded)
    fn trip(&mut self) {
        self.state = CircuitState::Open;
    }

    /// Reset the breaker (Closed state)
    fn reset(&mut self) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.last_failure_time = 0;
    }

    /// Check if enough time passed to attempt recovery (HalfOpen)
    fn should_attempt_reset(&self) -> bool {
        if self.state != CircuitState::Open {
            return false;
        }

        let now = current_time_ms();
        let elapsed = now.saturating_sub(self.last_failure_time);
        elapsed >= self.reset_timeout_ms
    }

    /// Attempt call with circuit breaker protection
    pub fn call<F, T>(&mut self, f: F) -> Result<T, CnfNetworkError>
    where
        F: FnOnce() -> Result<T, CnfNetworkError>,
    {
        if !self.should_allow_call() {
            return Err(CnfNetworkError::CircuitOpen(
                "breaker open, retry later".to_string(),
            ));
        }

        // Transition to HalfOpen if timeout elapsed
        if self.state == CircuitState::Open && self.should_attempt_reset() {
            self.state = CircuitState::HalfOpen;
        }

        match f() {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(e)
            }
        }
    }
}

/// Get current time in milliseconds since UNIX_EPOCH
fn current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_new() {
        let breaker = CircuitBreaker::new(5, 30000);
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_closed_allows_calls() {
        let breaker = CircuitBreaker::new(5, 30000);
        assert!(breaker.should_allow_call());
    }

    #[test]
    fn test_circuit_breaker_success_in_closed() {
        let mut breaker = CircuitBreaker::new(5, 30000);
        breaker.on_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_increments_failures() {
        let mut breaker = CircuitBreaker::new(5, 30000);
        breaker.on_failure();
        assert_eq!(breaker.failure_count(), 1);
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_trips_at_threshold() {
        let mut breaker = CircuitBreaker::new(3, 30000);
        breaker.on_failure();
        breaker.on_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);
        breaker.on_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
    }

    #[test]
    fn test_circuit_breaker_rejects_calls_when_open() {
        let mut breaker = CircuitBreaker::new(1, 30000);
        breaker.on_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.should_allow_call());
    }

    #[test]
    fn test_circuit_breaker_transitions_to_halfopen_after_timeout() {
        let mut breaker = CircuitBreaker::new(1, 1); // 1ms timeout for test
        breaker.on_failure();
        assert_eq!(breaker.state(), CircuitState::Open);

        // Wait for timeout (well, just check logic)
        std::thread::sleep(std::time::Duration::from_millis(2));

        // Should allow call in simulation (can't reliably test timing)
        assert!(breaker.should_attempt_reset());
    }

    #[test]
    fn test_circuit_breaker_success_in_halfopen_closes() {
        let mut breaker = CircuitBreaker::new(1, 1);
        breaker.on_failure();
        assert_eq!(breaker.state(), CircuitState::Open);

        std::thread::sleep(std::time::Duration::from_millis(2));

        // Simulate transition to HalfOpen (normally happens in call())
        if breaker.should_attempt_reset() {
            breaker.state = CircuitState::HalfOpen;
        }
        assert_eq!(breaker.state(), CircuitState::HalfOpen);

        // Success in HalfOpen resets
        breaker.on_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn test_circuit_breaker_call_succeeds_in_closed() {
        let mut breaker = CircuitBreaker::new(5, 30000);
        let result = breaker.call(|| Ok::<i32, CnfNetworkError>(42));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_call_fails_when_open() {
        let mut breaker = CircuitBreaker::new(1, 30000);
        breaker.on_failure(); // Trip the breaker

        let result = breaker.call(|| Ok::<i32, CnfNetworkError>(42));
        assert!(result.is_err());
        match result {
            Err(CnfNetworkError::CircuitOpen(_)) => {}
            _ => panic!("Expected CircuitOpen error"),
        }
    }

    #[test]
    fn test_circuit_breaker_call_increments_failures() {
        let mut breaker = CircuitBreaker::new(3, 30000);

        for i in 1..=3 {
            let result = breaker.call(|| {
                Err::<i32, CnfNetworkError>(CnfNetworkError::SendFailed("test failure".to_string()))
            });
            assert!(result.is_err());
            if i < 3 {
                assert_eq!(breaker.state(), CircuitState::Closed);
            }
        }

        assert_eq!(breaker.failure_count(), 3);
        assert_eq!(breaker.state(), CircuitState::Open);
    }
}
