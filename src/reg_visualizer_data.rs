use std::collections::HashMap;
use cpulib::{GPRName, VecRegName};
use crate::utilities::{Register, ValueType};

pub struct RegVisualizerData {
    // Registers Data
    pub registers: Vec<Vec<Register>>,
    pub vector_regs_type: HashMap<(VecRegName, usize), ValueType>,
    // Animation Data
    pub velocity: f32,
}

impl Default for RegVisualizerData {
    fn default() -> Self {
        Self {
            // Registers Data
            registers: vec![vec![]],
            vector_regs_type: HashMap::new(),
            // Animation Data
            velocity: 10f32,
        }
    }
}
