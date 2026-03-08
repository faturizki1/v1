//! Network transport layer for distributed CENTRA-NF
//!
//! Implements NetworkFrame format with CRC32 checksums and message routing.
//! Uses synchronous TCP (std::net, not async).

use crate::error::CnfNetworkError;
use crate::vector_clock::{NodeId, VectorClock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;

/// Network frame with length prefix, payload, and CRC32 checksum
///
/// Format: [u32 length][payload][u32 crc32]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkFrame {
    payload: Vec<u8>,
}

/// Message types for distributed operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CnfMessage {
    /// Send buffer to remote node
    SendBuffer {
        buffer_name: String,
        data: Vec<u8>,
        vector_clock: VectorClock,
    },
    /// Stream pipe data between nodes
    PipeStream {
        stream_id: String,
        data: Vec<u8>,
        vector_clock: VectorClock,
    },
    /// Remote procedure call
    RemoteCall {
        call_id: String,
        method: String,
        args: Vec<String>,
        vector_clock: VectorClock,
    },
    /// Acknowledgment of message receipt
    Ack {
        message_id: String,
        vector_clock: VectorClock,
    },
    /// Heartbeat/keepalive
    Heartbeat {
        node_id: NodeId,
        vector_clock: VectorClock,
    },
}

impl NetworkFrame {
    /// Create frame from message
    pub fn from_message(msg: &CnfMessage) -> Result<Self, CnfNetworkError> {
        let json = serde_json::to_vec(msg)
            .map_err(|e| CnfNetworkError::SerializationFailed(e.to_string()))?;
        Ok(NetworkFrame { payload: json })
    }

    /// Parse message from frame
    pub fn to_message(&self) -> Result<CnfMessage, CnfNetworkError> {
        serde_json::from_slice(&self.payload)
            .map_err(|e| CnfNetworkError::SerializationFailed(e.to_string()))
    }

    /// Serialize frame with CRC32 checksum
    pub fn serialize(&self) -> Result<Vec<u8>, CnfNetworkError> {
        let mut buf = Vec::new();

        // Write length (u32, big-endian)
        let len = self.payload.len() as u32;
        buf.extend_from_slice(&len.to_be_bytes());

        // Write payload
        buf.extend_from_slice(&self.payload);

        // Calculate and write CRC32
        let crc = crc32fast::hash(&buf);
        buf.extend_from_slice(&crc.to_be_bytes());

        Ok(buf)
    }

    /// Deserialize frame with CRC32 validation
    pub fn deserialize(data: &[u8]) -> Result<Self, CnfNetworkError> {
        if data.len() < 8 {
            return Err(CnfNetworkError::SendFailed("Frame too short".to_string()));
        }

        // Read length
        let len_bytes = &data[0..4];
        let len =
            u32::from_be_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;

        if data.len() < 4 + len + 4 {
            return Err(CnfNetworkError::SendFailed("Frame incomplete".to_string()));
        }

        // Extract payload
        let payload = data[4..4 + len].to_vec();

        // Read and verify CRC32
        let received_crc_bytes = &data[4 + len..4 + len + 4];
        let received_crc = u32::from_be_bytes([
            received_crc_bytes[0],
            received_crc_bytes[1],
            received_crc_bytes[2],
            received_crc_bytes[3],
        ]);

        // Calculate CRC over length + payload
        let mut crc_data = Vec::new();
        crc_data.extend_from_slice(len_bytes);
        crc_data.extend_from_slice(&payload);
        let expected_crc = crc32fast::hash(&crc_data);

        if received_crc != expected_crc {
            return Err(CnfNetworkError::ChecksumMismatch {
                expected: expected_crc,
                received: received_crc,
            });
        }

        Ok(NetworkFrame { payload })
    }

    /// Get payload length
    pub fn payload_len(&self) -> usize {
        self.payload.len()
    }
}

/// Synchronous TCP transport for node-to-node communication
pub struct TcpTransport {
    #[allow(dead_code)]
    node_id: NodeId,
    listener: Option<TcpListener>,
    connections: Arc<Mutex<HashMap<NodeId, TcpStream>>>,
}

impl std::fmt::Debug for TcpTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpTransport")
            .field("node_id", &self.node_id)
            .field("listener", &self.listener.is_some())
            .field("connections_count", &self.connections.lock().unwrap().len())
            .finish()
    }
}

impl TcpTransport {
    /// Create new transport
    pub fn new(node_id: NodeId) -> Self {
        TcpTransport {
            node_id,
            listener: None,
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Bind to listening address
    pub fn bind(&mut self, addr: &str) -> Result<(), CnfNetworkError> {
        let listener = TcpListener::bind(addr).map_err(|e| {
            CnfNetworkError::ConnectionFailed(format!("Failed to bind {}: {}", addr, e))
        })?;
        self.listener = Some(listener);
        Ok(())
    }

    /// Connect to remote node
    pub fn connect(&self, node_id: NodeId, addr: &str) -> Result<(), CnfNetworkError> {
        let stream = TcpStream::connect(addr).map_err(|e| {
            CnfNetworkError::ConnectionFailed(format!("Failed to connect to {}: {}", node_id, e))
        })?;

        let mut conns = self.connections.lock().unwrap();
        conns.insert(node_id, stream);
        Ok(())
    }

    /// Send message to target node
    pub fn send(&self, target: &NodeId, msg: &CnfMessage) -> Result<(), CnfNetworkError> {
        let frame = NetworkFrame::from_message(msg)?;
        let serialized = frame.serialize()?;

        let mut conns = self.connections.lock().unwrap();
        let stream = conns
            .get_mut(target)
            .ok_or_else(|| CnfNetworkError::NodeNotFound(target.to_string()))?;

        stream
            .write_all(&serialized)
            .map_err(|e| CnfNetworkError::SendFailed(e.to_string()))?;

        Ok(())
    }

    /// Receive message from any connected node (non-blocking returns error if none)
    pub fn receive(&mut self) -> Result<(NodeId, CnfMessage), CnfNetworkError> {
        let listener = self
            .listener
            .as_ref()
            .ok_or_else(|| CnfNetworkError::SendFailed("Not bound".to_string()))?;

        // Non-blocking accept (would need external handling for production)
        listener
            .set_nonblocking(true)
            .map_err(|e| CnfNetworkError::SendFailed(e.to_string()))?;

        match listener.accept() {
            Ok((mut stream, _addr)) => {
                // Read frame
                let mut size_buf = [0u8; 4];
                stream
                    .read_exact(&mut size_buf)
                    .map_err(|e| CnfNetworkError::ReceiveTimeout(e.to_string()))?;

                let len = u32::from_be_bytes(size_buf) as usize;
                let mut frame_buf = vec![0u8; 4 + len + 4];
                frame_buf[0..4].copy_from_slice(&size_buf);

                stream
                    .read_exact(&mut frame_buf[4..])
                    .map_err(|e| CnfNetworkError::ReceiveTimeout(e.to_string()))?;

                let frame = NetworkFrame::deserialize(&frame_buf)?;
                let msg = frame.to_message()?;

                Ok(("remote".to_string(), msg))
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Err(
                CnfNetworkError::ReceiveTimeout("No pending connections".to_string()),
            ),
            Err(e) => Err(CnfNetworkError::SendFailed(e.to_string())),
        }
    }

    /// Broadcast message to all connected nodes
    pub fn broadcast(&self, msg: &CnfMessage) -> Result<(), CnfNetworkError> {
        let frame = NetworkFrame::from_message(msg)?;
        let serialized = frame.serialize()?;

        let mut conns = self.connections.lock().unwrap();
        let mut errors = Vec::new();

        for (_node_id, stream) in conns.iter_mut() {
            if let Err(e) = stream.write_all(&serialized) {
                errors.push(e.to_string());
            }
        }

        if !errors.is_empty() {
            return Err(CnfNetworkError::SendFailed(errors.join("; ")));
        }

        Ok(())
    }

    /// Disconnect from target node
    pub fn disconnect(&self, node_id: &NodeId) -> Result<(), CnfNetworkError> {
        let mut conns = self.connections.lock().unwrap();
        conns
            .remove(node_id)
            .ok_or_else(|| CnfNetworkError::NodeNotFound(node_id.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_frame_roundtrip() {
        let msg = CnfMessage::Heartbeat {
            node_id: "node_a".to_string(),
            vector_clock: VectorClock::new(),
        };

        let frame = NetworkFrame::from_message(&msg).unwrap();
        let serialized = frame.serialize().unwrap();
        let deserialized_frame = NetworkFrame::deserialize(&serialized).unwrap();
        let parsed_msg = deserialized_frame.to_message().unwrap();

        assert_eq!(msg, parsed_msg);
    }

    #[test]
    fn test_network_frame_crc_validation() {
        let msg = CnfMessage::SendBuffer {
            buffer_name: "test_buf".to_string(),
            data: b"some_data".to_vec(),
            vector_clock: VectorClock::new(),
        };

        let frame = NetworkFrame::from_message(&msg).unwrap();
        let mut serialized = frame.serialize().unwrap();

        // Corrupt the payload (before checksum)
        if serialized.len() > 8 {
            serialized[5] ^= 0xFF; // flip bits in payload
        }

        let result = NetworkFrame::deserialize(&serialized);
        assert!(result.is_err());

        match result {
            Err(CnfNetworkError::ChecksumMismatch { .. }) => {}
            _ => panic!("Expected ChecksumMismatch error"),
        }
    }

    #[test]
    fn test_network_frame_crc_valid() {
        let msg = CnfMessage::Ack {
            message_id: "msg_123".to_string(),
            vector_clock: VectorClock::new(),
        };

        let frame = NetworkFrame::from_message(&msg).unwrap();
        let serialized = frame.serialize().unwrap();

        // Deserialize should succeed (no corruption)
        let result = NetworkFrame::deserialize(&serialized);
        assert!(result.is_ok());
    }

    #[test]
    fn test_network_frame_short_data() {
        let data = vec![0u8; 3];
        let result = NetworkFrame::deserialize(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_network_frame_incomplete() {
        let mut data = Vec::new();
        data.extend_from_slice(&100u32.to_be_bytes()); // length = 100
        data.extend_from_slice(b"short"); // but only 5 bytes + 4 crc
        data.extend_from_slice(&0u32.to_be_bytes()); // dummy crc

        let result = NetworkFrame::deserialize(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_cnf_message_send_buffer() {
        let msg = CnfMessage::SendBuffer {
            buffer_name: "output".to_string(),
            data: vec![1, 2, 3, 4, 5],
            vector_clock: VectorClock::new(),
        };

        let frame = NetworkFrame::from_message(&msg).unwrap();
        let recovered = frame.to_message().unwrap();

        match recovered {
            CnfMessage::SendBuffer {
                buffer_name, data, ..
            } => {
                assert_eq!(buffer_name, "output");
                assert_eq!(data, vec![1, 2, 3, 4, 5]);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_cnf_message_remote_call() {
        let msg = CnfMessage::RemoteCall {
            call_id: "call_456".to_string(),
            method: "compress".to_string(),
            args: vec!["buf1".to_string(), "buf2".to_string()],
            vector_clock: VectorClock::new(),
        };

        let frame = NetworkFrame::from_message(&msg).unwrap();
        let recovered = frame.to_message().unwrap();

        match recovered {
            CnfMessage::RemoteCall {
                call_id,
                method,
                args,
                ..
            } => {
                assert_eq!(call_id, "call_456");
                assert_eq!(method, "compress");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_tcp_transport_new() {
        let transport = TcpTransport::new("node_a".to_string());
        assert_eq!(transport.node_id, "node_a");
    }

    #[test]
    fn test_tcp_transport_bind_invalid() {
        let mut transport = TcpTransport::new("node_a".to_string());
        let result = transport.bind("999.999.999.999:9999");
        assert!(result.is_err());
    }

    #[test]
    fn test_tcp_transport_receive_on_unbound() {
        let mut transport = TcpTransport::new("node_a".to_string());
        let result = transport.receive();
        assert!(result.is_err());
    }

    #[test]
    fn test_tcp_transport_send_unknown_node() {
        let transport = TcpTransport::new("node_a".to_string());
        let msg = CnfMessage::Heartbeat {
            node_id: "node_a".to_string(),
            vector_clock: VectorClock::new(),
        };

        let result = transport.send(&"unknown_node".to_string(), &msg);
        assert!(matches!(result, Err(CnfNetworkError::NodeNotFound(_))));
    }

    #[test]
    fn test_tcp_transport_disconnect_unknown() {
        let transport = TcpTransport::new("node_a".to_string());
        let result = transport.disconnect(&"unknown_node".to_string());
        assert!(matches!(result, Err(CnfNetworkError::NodeNotFound(_))));
    }
}
