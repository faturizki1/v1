//! Vector clock implementation for causal ordering
//!
//! VectorClock tracks per-node logical timestamps for deterministic
//! ordering of distributed events. All methods except increment are pure.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

pub type NodeId = String;

/// Vector clock for tracking causal relationships
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorClock {
    /// Map of node_id → logical timestamp
    clock: HashMap<NodeId, u64>,
}

impl VectorClock {
    /// Create empty vector clock
    pub fn new() -> Self {
        VectorClock {
            clock: HashMap::new(),
        }
    }

    /// Increment this node's timestamp (mutating)
    pub fn increment(&mut self, node_id: &NodeId) {
        *self.clock.entry(node_id.clone()).or_insert(0) += 1;
    }

    /// Merge with another clock (pure)
    pub fn merge(&self, other: &VectorClock) -> VectorClock {
        let mut merged = self.clock.clone();
        for (node_id, timestamp) in &other.clock {
            merged
                .entry(node_id.clone())
                .and_modify(|t| *t = (*t).max(*timestamp))
                .or_insert(*timestamp);
        }
        VectorClock { clock: merged }
    }

    /// Check if this clock happened before another (pure)
    pub fn happened_before(&self, other: &VectorClock) -> bool {
        if self == other {
            return false;
        }

        // self < other if:
        // - self[i] <= other[i] for all i
        // - self[j] < other[j] for some j
        let mut has_strict_less = false;

        let all_nodes: std::collections::HashSet<_> =
            self.clock.keys().chain(other.clock.keys()).collect();

        for node_id in all_nodes {
            let self_ts = self.clock.get(node_id).copied().unwrap_or(0);
            let other_ts = other.clock.get(node_id).copied().unwrap_or(0);

            if self_ts > other_ts {
                return false;
            }
            if self_ts < other_ts {
                has_strict_less = true;
            }
        }

        has_strict_less
    }

    /// Check if two clocks are concurrent (pure)
    pub fn is_concurrent(&self, other: &VectorClock) -> bool {
        !self.happened_before(other) && !other.happened_before(self)
    }

    /// Deterministic total order for concurrent events (pure)
    pub fn deterministic_order(
        &self,
        other: &VectorClock,
        self_node: &NodeId,
        other_node: &NodeId,
    ) -> Ordering {
        // If causally ordered, use that
        if self.happened_before(other) {
            return Ordering::Less;
        }
        if other.happened_before(self) {
            return Ordering::Greater;
        }

        // For concurrent events, use node_id lexicographic order as tiebreaker
        self_node.cmp(other_node)
    }

    /// Get timestamp for a node (pure)
    pub fn get(&self, node_id: &NodeId) -> u64 {
        self.clock.get(node_id).copied().unwrap_or(0)
    }

    /// Get all entries (pure)
    pub fn entries(&self) -> &HashMap<NodeId, u64> {
        &self.clock
    }
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_clock_empty() {
        let vc = VectorClock::new();
        assert_eq!(vc.entries().len(), 0);
    }

    #[test]
    fn test_vector_clock_increment() {
        let mut vc = VectorClock::new();
        let node_a = "node_a".to_string();

        vc.increment(&node_a);
        assert_eq!(vc.get(&node_a), 1);

        vc.increment(&node_a);
        assert_eq!(vc.get(&node_a), 2);
    }

    #[test]
    fn test_vector_clock_increment_multiple_nodes() {
        let mut vc = VectorClock::new();
        let node_a = "node_a".to_string();
        let node_b = "node_b".to_string();

        vc.increment(&node_a);
        vc.increment(&node_b);
        vc.increment(&node_a);

        assert_eq!(vc.get(&node_a), 2);
        assert_eq!(vc.get(&node_b), 1);
    }

    #[test]
    fn test_vector_clock_merge_empty() {
        let vc1 = VectorClock::new();
        let vc2 = VectorClock::new();
        let merged = vc1.merge(&vc2);
        assert_eq!(merged.entries().len(), 0);
    }

    #[test]
    fn test_vector_clock_merge_simple() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();
        let node_b = "node_b".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_b);

        let merged = vc1.merge(&vc2);
        assert_eq!(merged.get(&node_a), 1);
        assert_eq!(merged.get(&node_b), 1);
    }

    #[test]
    fn test_vector_clock_merge_overlapping() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();
        let node_b = "node_b".to_string();

        // vc1: {a:2, b:1}
        vc1.increment(&node_a);
        vc1.increment(&node_a);
        vc1.increment(&node_b);

        // vc2: {a:1, b:3}
        vc2.increment(&node_a);
        vc2.increment(&node_b);
        vc2.increment(&node_b);
        vc2.increment(&node_b);

        let merged = vc1.merge(&vc2);
        assert_eq!(merged.get(&node_a), 2);
        assert_eq!(merged.get(&node_b), 3);
    }

    #[test]
    fn test_vector_clock_happened_before_true() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();
        let node_b = "node_b".to_string();

        vc1.increment(&node_a);
        vc1.increment(&node_b);

        vc2.increment(&node_a);
        vc2.increment(&node_b);
        vc2.increment(&node_b);

        assert!(vc1.happened_before(&vc2));
    }

    #[test]
    fn test_vector_clock_happened_before_false() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();
        let node_b = "node_b".to_string();

        vc1.increment(&node_a);
        vc1.increment(&node_a);

        vc2.increment(&node_b);

        assert!(!vc1.happened_before(&vc2));
    }

    #[test]
    fn test_vector_clock_happened_before_equal() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_a);

        assert!(!vc1.happened_before(&vc2));
        assert!(!vc2.happened_before(&vc1));
    }

    #[test]
    fn test_vector_clock_is_concurrent_true() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();
        let node_b = "node_b".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_b);

        assert!(vc1.is_concurrent(&vc2));
    }

    #[test]
    fn test_vector_clock_is_concurrent_false() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_a);
        vc2.increment(&node_a);

        assert!(!vc1.is_concurrent(&vc2));
    }

    #[test]
    fn test_vector_clock_deterministic_order_happened_before() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_a);
        vc2.increment(&node_a);

        let order = vc1.deterministic_order(&vc2, &node_a, &node_a);
        assert_eq!(order, Ordering::Less);
    }

    #[test]
    fn test_vector_clock_deterministic_order_concurrent_tiebreak() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();
        let node_b = "node_b".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_b);

        let order = vc1.deterministic_order(&vc2, &node_a, &node_b);
        assert_eq!(order, Ordering::Less); // node_a < node_b lexicographically

        let order_rev = vc2.deterministic_order(&vc1, &node_b, &node_a);
        assert_eq!(order_rev, Ordering::Greater); // node_b > node_a lexicographically
    }

    #[test]
    fn test_vector_clock_deterministic_order_same_nodes() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_a);

        let order = vc1.deterministic_order(&vc2, &node_a, &node_a);
        assert_eq!(order, Ordering::Equal);
    }

    #[test]
    fn test_vector_clock_merge_is_pure() {
        let mut vc1 = VectorClock::new();
        let mut vc2 = VectorClock::new();

        let node_a = "node_a".to_string();

        vc1.increment(&node_a);
        vc2.increment(&node_a);
        vc2.increment(&node_a);

        let vc1_before = vc1.clone();
        let vc2_before = vc2.clone();

        let _merged = vc1.merge(&vc2);

        assert_eq!(vc1, vc1_before);
        assert_eq!(vc2, vc2_before);
    }
}
