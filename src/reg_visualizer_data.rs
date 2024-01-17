use std::collections::HashMap;
use cpulib::{VecRegName};
use crate::utilities::{Register, ValueType};

pub struct RegVisualizerData {
    // Registers Data
    pub registers: Vec<Vec<Register>>,
    pub vector_regs_type: HashMap<(VecRegName, usize), ValueType>,
    // Animation Data
    pub factor: f32,
    pub min_speed: f32,
    pub max_speed: f32,
}

impl Default for RegVisualizerData {
    fn default() -> Self {
        Self {
            // Registers Data
            registers: vec![vec![]],
            vector_regs_type: HashMap::new(),
            // Animation Data
            factor: 1.0f32,
            min_speed: 1.0f32,
            max_speed: 1000.0f32,
        }
    }
}
