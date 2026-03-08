//! CENTRA-NF Network Layer (L6)
//!
//! Distributed networking for causal message ordering and fault-tolerant
//! node-to-node communication. Implements vector clocks for deterministic
//! event ordering and synchronous TCP transport.
//!
//! Key components:
//! - Vector clock for causal ordering
//! - Network frame serialization with CRC32 checksums
//! - Message types for remote operations
//! - Synchronous TCP transport
//! - Circuit breaker pattern for fault tolerance
//! - Distributed DAG for multi-node execution
//! - Network node for cluster participation

pub mod circuit_breaker;
pub mod distributed_dag;
pub mod error;
pub mod node;
pub mod transport;
pub mod vector_clock;

pub use circuit_breaker::{CircuitBreaker, CircuitState};
pub use distributed_dag::{DistributedDag, NodeInfo, NodeStatus};
pub use error::CnfNetworkError;
pub use node::NetworkNode;
pub use transport::{CnfMessage, NetworkFrame, TcpTransport};
pub use vector_clock::{NodeId, VectorClock};

pub type Result<T> = std::result::Result<T, CnfNetworkError>;
