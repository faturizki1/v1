//! DAG — 8-layer Directed Acyclic Graph.
//!
//! Provides structural representation of execution dependencies.
//! Layers represent stages of processing.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DagLayer {
    pub layer_id: usize,
    pub instructions: Vec<String>, // instruction identifiers
}

#[derive(Debug, Clone)]
pub struct Dag {
    pub layers: Vec<DagLayer>,
    pub dependency_map: HashMap<String, usize>, // instruction → layer
}

impl Dag {
    pub fn new() -> Self {
        Dag {
            layers: vec![],
            dependency_map: HashMap::new(),
        }
    }

    /// Create 8 empty layers.
    pub fn initialize_layers() -> Self {
        let mut dag = Dag::new();
        for i in 0..8 {
            dag.layers.push(DagLayer {
                layer_id: i,
                instructions: Vec::new(),
            });
        }
        dag
    }

    /// Assign instruction to layer.
    pub fn assign_to_layer(&mut self, layer_id: usize, instruction: String) {
        if layer_id < self.layers.len() {
            self.layers[layer_id].instructions.push(instruction.clone());
            self.dependency_map.insert(instruction, layer_id);
        }
    }

    pub fn get_layer(&self, layer_id: usize) -> Option<&DagLayer> {
        self.layers.get(layer_id)
    }

    pub fn total_layers(&self) -> usize {
        self.layers.len()
    }
}

impl Default for Dag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_initializes_8_layers() {
        let dag = Dag::initialize_layers();
        assert_eq!(dag.layers.len(), 8);
    }

    #[test]
    fn test_dag_assigns_instruction_to_layer() {
        let mut dag = Dag::initialize_layers();
        dag.assign_to_layer(0, "COMPRESS(buffer)".to_string());
        assert_eq!(dag.layers[0].instructions.len(), 1);
    }
}
