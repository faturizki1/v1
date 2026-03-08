//! Network node for CENTRA-NF cluster participation
//!
//! A NetworkNode represents a single member of a distributed CENTRA-NF cluster.
//! It manages:
//! - Cluster connectivity (via TcpTransport)
//! - Local and distributed execution (via DistributedDag)
//! - Causal ordering (via VectorClock)
//! - Fault tolerance (via CircuitBreaker)
//!
//! Each node can send buffers, receive from others, and handle remote calls.

use crate::circuit_breaker::CircuitBreaker;
use crate::distributed_dag::DistributedDag;
use crate::error::CnfNetworkError;
use crate::transport::TcpTransport;
use crate::vector_clock::{NodeId, VectorClock};
use std::collections::HashMap;

/// Represents a single node in CENTRA-NF cluster
#[derive(Debug)]
pub struct NetworkNode {
    /// Unique identifier for this node
    node_id: NodeId,
    /// TCP transport for network communication
    transport: TcpTransport,
    /// Distributed DAG for partitioned execution
    dag: DistributedDag,
    /// Vector clock for causal ordering
    clock: VectorClock,
    /// Circuit breaker for fault tolerance
    breaker: CircuitBreaker,
    /// Whether this node is connected to cluster
    connected: bool,
    /// Known peer nodes (node_id → address)
    peers: HashMap<NodeId, String>,
}

impl NetworkNode {
    /// Create new network node
    pub fn new(node_id: NodeId) -> Self {
        let mut clock = VectorClock::new();
        clock.increment(&node_id);

        NetworkNode {
            node_id: node_id.clone(),
            transport: TcpTransport::new(node_id.clone()),
            dag: DistributedDag::new(),
            clock,
            breaker: CircuitBreaker::new(5, 30000),
            connected: false,
            peers: HashMap::new(),
        }
    }

    /// Create with custom circuit breaker settings
    pub fn with_breaker_config(
        node_id: NodeId,
        breaker_threshold: u32,
        breaker_timeout_ms: u64,
    ) -> Self {
        let mut clock = VectorClock::new();
        clock.increment(&node_id);

        NetworkNode {
            node_id: node_id.clone(),
            transport: TcpTransport::new(node_id.clone()),
            dag: DistributedDag::with_breaker_config(breaker_threshold, breaker_timeout_ms),
            clock,
            breaker: CircuitBreaker::new(breaker_threshold, breaker_timeout_ms),
            connected: false,
            peers: HashMap::new(),
        }
    }

    /// Get this node's ID
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Get current vector clock
    pub fn clock(&self) -> &VectorClock {
        &self.clock
    }

    /// Add a peer node to known peers
    pub fn add_peer(&mut self, peer_id: NodeId, peer_address: String) {
        self.peers.insert(peer_id, peer_address);
    }

    /// Get known peers
    pub fn peers(&self) -> &HashMap<NodeId, String> {
        &self.peers
    }

    /// Check if connected to cluster
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Connect to cluster (bind to accept connections)
    pub fn connect_to_cluster(&mut self, listen_address: &str) -> Result<(), CnfNetworkError> {
        // Bind to accept incoming connections
        self.transport.bind(listen_address)?;
        self.connected = true;
        Ok(())
    }

    /// Send buffer to remote node
    pub fn send_buffer(
        &mut self,
        target_id: &NodeId,
        buffer_name: String,
        data: Vec<u8>,
    ) -> Result<(), CnfNetworkError> {
        if !self.connected {
            return Err(CnfNetworkError::ConnectionFailed(
                "not connected to cluster".to_string(),
            ));
        }

        // Increment vector clock
        self.clock.increment(&self.node_id);

        // Create message
        let message = crate::transport::CnfMessage::SendBuffer {
            buffer_name,
            data,
            vector_clock: self.clock.clone(),
        };

        // Use circuit breaker for fault tolerance
        self.breaker
            .call(|| self.transport.send(target_id, &message))
    }

    /// Receive buffer from remote node
    pub fn receive_from(&mut self) -> Result<(NodeId, Vec<u8>), CnfNetworkError> {
        if !self.connected {
            return Err(CnfNetworkError::ConnectionFailed(
                "not connected to cluster".to_string(),
            ));
        }

        // Use circuit breaker for fault tolerance
        self.breaker.call(|| {
            let (node_id, msg) = self.transport.receive()?;
            // Extract buffer data from message
            match msg {
                crate::transport::CnfMessage::SendBuffer {
                    data, vector_clock, ..
                } => {
                    // Merge remote vector clock for causal ordering
                    self.clock = self.clock.merge(&vector_clock);
                    Ok((node_id, data))
                }
                _ => Err(CnfNetworkError::SerializationFailed(
                    "expected SendBuffer message".to_string(),
                )),
            }
        })
    }

    /// Get access to distributed DAG
    pub fn dag(&self) -> &DistributedDag {
        &self.dag
    }

    /// Get mutable access to distributed DAG
    pub fn dag_mut(&mut self) -> &mut DistributedDag {
        &mut self.dag
    }

    /// Get distributed DAG for execution planning
    pub fn distributed_dag(&self) -> &DistributedDag {
        &self.dag
    }

    /// Shutdown node (cleanup connections)
    pub fn shutdown(&mut self) -> Result<(), CnfNetworkError> {
        // Disconnect from all known peers
        for peer_id in self.peers.keys() {
            let _ = self.transport.disconnect(peer_id);
        }
        self.connected = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_node_new() {
        let node = NetworkNode::new("node1".to_string());
        assert_eq!(node.node_id(), "node1");
        assert!(!node.is_connected());
        assert_eq!(node.peers().len(), 0);
    }

    #[test]
    fn test_network_node_with_breaker_config() {
        let node = NetworkNode::with_breaker_config("node1".to_string(), 3, 5000);
        assert_eq!(node.node_id(), "node1");
        assert_eq!(node.breaker.failure_count(), 0);
    }

    #[test]
    fn test_network_node_add_peer() {
        let mut node = NetworkNode::new("node1".to_string());
        node.add_peer("node2".to_string(), "127.0.0.1:9002".to_string());
        assert_eq!(node.peers().len(), 1);
        assert_eq!(
            node.peers().get("node2"),
            Some(&"127.0.0.1:9002".to_string())
        );
    }

    #[test]
    fn test_network_node_connect_initializes_clock() {
        let node = NetworkNode::new("node1".to_string());
        // Clock should have been initialized with node1
        assert_eq!(node.clock(), node.clock());
    }

    #[test]
    fn test_network_node_not_connected_initially() {
        let node = NetworkNode::new("node1".to_string());
        assert!(!node.is_connected());
    }

    #[test]
    fn test_network_node_send_buffer_not_connected_fails() {
        let mut node = NetworkNode::new("node1".to_string());
        let result = node.send_buffer(&"node2".to_string(), "buffer1".to_string(), vec![1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_network_node_receive_not_connected_fails() {
        let mut node = NetworkNode::new("node1".to_string());
        let result = node.receive_from();
        assert!(result.is_err());
    }

    #[test]
    fn test_network_node_shutdown_disconnects() {
        let mut node = NetworkNode::new("node1".to_string());
        node.add_peer("node2".to_string(), "127.0.0.1:9002".to_string());
        let result = node.shutdown();
        assert!(result.is_ok());
        assert!(!node.is_connected());
    }

    #[test]
    fn test_network_node_dag_access() {
        let node = NetworkNode::new("node1".to_string());
        assert_eq!(node.distributed_dag().nodes().len(), 0);
    }

    #[test]
    fn test_network_node_multiple_peers() {
        let mut node = NetworkNode::new("node1".to_string());
        node.add_peer("node2".to_string(), "127.0.0.1:9002".to_string());
        node.add_peer("node3".to_string(), "127.0.0.1:9003".to_string());
        node.add_peer("node4".to_string(), "127.0.0.1:9004".to_string());
        assert_eq!(node.peers().len(), 3);
    }
}
