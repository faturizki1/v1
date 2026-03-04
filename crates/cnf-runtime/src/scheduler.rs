//! Scheduler — Layer-by-layer deterministic execution.
//!
//! Executes each DAG layer fully before moving to the next.
//! Ensures deterministic ordering.

use crate::dag::Dag;

pub struct Scheduler;

impl Scheduler {
    /// Execute all layers in sequence.
    /// Each layer must complete before the next begins.
    pub fn execute_all_layers(
        dag: &Dag,
        executor: &mut dyn FnMut(&str) -> Result<(), String>,
    ) -> Result<(), String> {
        for layer in &dag.layers {
            for instruction in &layer.instructions {
                executor(instruction)?;
            }
        }
        Ok(())
    }

    /// Execute single layer.
    pub fn execute_layer(
        dag: &Dag,
        layer_id: usize,
        executor: &mut dyn FnMut(&str) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(layer) = dag.get_layer(layer_id) {
            for instruction in &layer.instructions {
                executor(instruction)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_executes_layers_in_order() {
        let mut dag = Dag::initialize_layers();
        dag.assign_to_layer(0, "COMPRESS(buffer)".to_string());
        dag.assign_to_layer(1, "VERIFY(buffer)".to_string());

        let mut execution_order = Vec::new();
        let mut executor = |instr: &str| {
            execution_order.push(instr.to_string());
            Ok::<_, String>(())
        };

        Scheduler::execute_all_layers(&dag, &mut executor).unwrap();
        assert_eq!(execution_order.len(), 2);
        assert_eq!(execution_order[0], "COMPRESS(buffer)");
        assert_eq!(execution_order[1], "VERIFY(buffer)");
    }
}
