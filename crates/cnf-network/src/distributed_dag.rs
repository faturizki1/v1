//! Distributed DAG for multi-node execution
//!
//! Extends the local DAG (from cnf-runtime) across multiple nodes in a cluster.
//! Implements layer partitioning for distributed execution:
//! - Each layer can be assigned to a node
//! - Round-robin auto-partitioning for balanced load
//! - Health tracking for node management
//!
//! Deterministic round-robin ensures same execution across restarts.

use crate::circuit_breaker::CircuitBreaker;
use crate::error::CnfNetworkError;
use crate::vector_clock::{NodeId, VectorClock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Health status of a node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    /// Node is operational
    Healthy,
    /// Node has elevated latency or packet loss
    Degraded,
    /// Node is down/unreachable
    Down,
}

/// Information about a cluster node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique node identifier
    pub node_id: NodeId,
    /// Network address (e.g., "127.0.0.1:9001")
    pub address: String,
    /// Current health status
    pub status: NodeStatus,
    /// Layer IDs assigned to this node
    pub assigned_layers: Vec<u32>,
    /// Timestamp of last heartbeat (ms since UNIX_EPOCH)
    pub last_heartbeat_ms: u64,
}

impl NodeInfo {
    /// Create new node info
    pub fn new(node_id: NodeId, address: String) -> Self {
        NodeInfo {
            node_id,
            address,
            status: NodeStatus::Healthy,
            assigned_layers: Vec::new(),
            last_heartbeat_ms: 0,
        }
    }
}

/// Distributed DAG for multi-node CENTRA-NF execution
#[derive(Debug, Clone)]
pub struct DistributedDag {
    /// Vector clock for causal ordering across nodes
    vector_clock: VectorClock,
    /// Known nodes in cluster
    nodes: HashMap<NodeId, NodeInfo>,
    /// Mapping of layer_id → responsible node_id
    partition_map: HashMap<u32, NodeId>,
    /// Circuit breaker for fault tolerance
    breaker: CircuitBreaker,
}

impl DistributedDag {
    /// Create new distributed DAG
    pub fn new() -> Self {
        DistributedDag {
            vector_clock: VectorClock::new(),
            nodes: HashMap::new(),
            partition_map: HashMap::new(),
            breaker: CircuitBreaker::new(5, 30000),
        }
    }

    /// Create with custom circuit breaker settings
    pub fn with_breaker_config(breaker_threshold: u32, breaker_timeout_ms: u64) -> Self {
        DistributedDag {
            vector_clock: VectorClock::new(),
            nodes: HashMap::new(),
            partition_map: HashMap::new(),
            breaker: CircuitBreaker::new(breaker_threshold, breaker_timeout_ms),
        }
    }

    /// Register a new node in the cluster
    pub fn register_node(&mut self, node_info: NodeInfo) -> Result<(), CnfNetworkError> {
        if self.nodes.contains_key(&node_info.node_id) {
            return Err(CnfNetworkError::NodeNotFound(format!(
                "node {} already registered",
                node_info.node_id
            )));
        }
        self.nodes.insert(node_info.node_id.clone(), node_info);
        Ok(())
    }

    /// Assign a layer to a specific node
    pub fn assign_layer_to_node(
        &mut self,
        layer_id: u32,
        node_id: &NodeId,
    ) -> Result<(), CnfNetworkError> {
        if !self.nodes.contains_key(node_id) {
            return Err(CnfNetworkError::NodeNotFound(format!(
                "node {} not registered",
                node_id
            )));
        }

        self.partition_map.insert(layer_id, node_id.clone());

        // Add to node's assigned layers
        if let Some(node) = self.nodes.get_mut(node_id) {
            if !node.assigned_layers.contains(&layer_id) {
                node.assigned_layers.push(layer_id);
            }
        }

        Ok(())
    }

    /// Auto-partition layers across nodes using round-robin
    pub fn auto_partition(&mut self, layer_count: u32) -> Result<(), CnfNetworkError> {
        if self.nodes.is_empty() {
            return Err(CnfNetworkError::NodeNotFound(
                "no nodes available for partitioning".to_string(),
            ));
        }

        // Get deterministic node order (sort by node_id for consistency)
        let mut node_ids: Vec<_> = self.nodes.keys().cloned().collect();
        node_ids.sort();

        // Round-robin assignment
        for layer_id in 0..layer_count {
            let node_idx = (layer_id as usize) % node_ids.len();
            let node_id = node_ids[node_idx].clone();
            self.assign_layer_to_node(layer_id, &node_id)?;
        }

        Ok(())
    }

    /// Get the node responsible for a layer
    pub fn get_node_for_layer(&self, layer_id: u32) -> Result<NodeInfo, CnfNetworkError> {
        let node_id = self.partition_map.get(&layer_id).ok_or_else(|| {
            CnfNetworkError::NodeNotFound(format!("layer {} not assigned", layer_id))
        })?;

        self.nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| CnfNetworkError::NodeNotFound(format!("node {} not found", node_id)))
    }

    /// Mark a node as down (or another status)
    pub fn mark_node_status(
        &mut self,
        node_id: &NodeId,
        status: NodeStatus,
    ) -> Result<(), CnfNetworkError> {
        self.nodes
            .get_mut(node_id)
            .map(|n| n.status = status)
            .ok_or_else(|| CnfNetworkError::NodeNotFound(format!("node {} not found", node_id)))
    }

    /// Get all nodes in cluster
    pub fn nodes(&self) -> &HashMap<NodeId, NodeInfo> {
        &self.nodes
    }

    /// Get partition map (layer → node)
    pub fn partition_map(&self) -> &HashMap<u32, NodeId> {
        &self.partition_map
    }

    /// Get vector clock
    pub fn vector_clock(&self) -> &VectorClock {
        &self.vector_clock
    }

    /// Increment vector clock for this node
    pub fn increment_clock(&mut self, node_id: &NodeId) {
        self.vector_clock.increment(node_id);
    }

    /// Get circuit breaker state
    pub fn circuit_breaker(&self) -> &CircuitBreaker {
        &self.breaker
    }

    /// Execute distributed operation (stub for now)
    pub fn execute_distributed(&mut self, _layer_id: u32) -> Result<(), CnfNetworkError> {
        // TODO: Implement actual distributed execution
        // For now, just a stub that increments vector clock
        Ok(())
    }
}

impl Default for DistributedDag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distributed_dag_new() {
        let dag = DistributedDag::new();
        assert_eq!(dag.nodes().len(), 0);
        assert_eq!(dag.partition_map().len(), 0);
    }

    #[test]
    fn test_distributed_dag_register_node() {
        let mut dag = DistributedDag::new();
        let node = NodeInfo::new("node1".to_string(), "127.0.0.1:9001".to_string());
        let result = dag.register_node(node);
        assert!(result.is_ok());
        assert_eq!(dag.nodes().len(), 1);
    }

    #[test]
    fn test_distributed_dag_register_duplicate_node_fails() {
        let mut dag = DistributedDag::new();
        let node = NodeInfo::new("node1".to_string(), "127.0.0.1:9001".to_string());
        dag.register_node(node.clone()).unwrap();
        let result = dag.register_node(node);
        assert!(result.is_err());
    }

    #[test]
    fn test_distributed_dag_assign_layer_to_node() {
        let mut dag = DistributedDag::new();
        let node = NodeInfo::new("node1".to_string(), "127.0.0.1:9001".to_string());
        dag.register_node(node).unwrap();

        let result = dag.assign_layer_to_node(0, &"node1".to_string());
        assert!(result.is_ok());
        assert_eq!(dag.partition_map().len(), 1);
    }

    #[test]
    fn test_distributed_dag_assign_layer_unknown_node_fails() {
        let mut dag = DistributedDag::new();
        let result = dag.assign_layer_to_node(0, &"unknown".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_distributed_dag_get_node_for_layer() {
        let mut dag = DistributedDag::new();
        let node = NodeInfo::new("node1".to_string(), "127.0.0.1:9001".to_string());
        dag.register_node(node).unwrap();
        dag.assign_layer_to_node(0, &"node1".to_string()).unwrap();

        let result = dag.get_node_for_layer(0);
        assert!(result.is_ok());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.node_id, "node1");
    }

    #[test]
    fn test_distributed_dag_auto_partition_roundrobin() {
        let mut dag = DistributedDag::new();
        let node1 = NodeInfo::new("node1".to_string(), "127.0.0.1:9001".to_string());
        let node2 = NodeInfo::new("node2".to_string(), "127.0.0.1:9002".to_string());
        dag.register_node(node1).unwrap();
        dag.register_node(node2).unwrap();

        let result = dag.auto_partition(4);
        assert!(result.is_ok());
        assert_eq!(dag.partition_map().len(), 4);

        // Verify round-robin: layers 0,2 → node1; layers 1,3 → node2
        assert_eq!(dag.get_node_for_layer(0).unwrap().node_id, "node1");
        assert_eq!(dag.get_node_for_layer(1).unwrap().node_id, "node2");
        assert_eq!(dag.get_node_for_layer(2).unwrap().node_id, "node1");
        assert_eq!(dag.get_node_for_layer(3).unwrap().node_id, "node2");
    }

    #[test]
    fn test_distributed_dag_mark_node_status() {
        let mut dag = DistributedDag::new();
        let node = NodeInfo::new("node1".to_string(), "127.0.0.1:9001".to_string());
        dag.register_node(node).unwrap();

        let result = dag.mark_node_status(&"node1".to_string(), NodeStatus::Down);
        assert!(result.is_ok());
        assert_eq!(dag.nodes().get("node1").unwrap().status, NodeStatus::Down);
    }
}
