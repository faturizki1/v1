use crate::assertion::{HoareAnnotation, Predicate, SecurityLevel};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HoareTriple {
    pub pre: Predicate,
    pub body_description: String,
    pub post: Predicate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferState {
    pub length: usize,
    pub security_level: SecurityLevel,
    pub is_empty: bool,
}

#[derive(Debug)]
pub struct HoareContext {
    pub triples: Vec<HoareTriple>,
    pub annotations: Vec<HoareAnnotation>,
    pub buffer_states: HashMap<String, BufferState>,
}

impl HoareContext {
    pub fn new() -> Self {
        Self {
            triples: Vec::new(),
            annotations: Vec::new(),
            buffer_states: HashMap::new(),
        }
    }

    pub fn add_annotation(&mut self, ann: HoareAnnotation) {
        self.annotations.push(ann);
    }

    pub fn set_buffer_state(&mut self, name: String, state: BufferState) {
        self.buffer_states.insert(name, state);
    }

    pub fn collect_triples(&mut self) -> Vec<HoareTriple> {
        let mut triples = Vec::new();
        let mut i = 0;
        while i < self.annotations.len() {
            if let Some(pre) = self.annotations.get(i) {
                if matches!(pre.kind, crate::assertion::AssertionKind::PreCondition) {
                    if let Some(post) = self.annotations.get(i + 1) {
                        if matches!(post.kind, crate::assertion::AssertionKind::PostCondition) {
                            triples.push(HoareTriple {
                                pre: pre.predicate.clone(),
                                body_description: format!("procedure at {}", pre.source_location),
                                post: post.predicate.clone(),
                            });
                            i += 2; // skip both
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
        self.triples = triples.clone();
        triples
    }
}

impl Default for HoareContext {
    fn default() -> Self {
        Self::new()
    }
}
