use std::collections::HashMap;

use kosem_webapi::phase_control_messages::{Component, ComponentParams};

#[derive(Debug)]
pub struct Phase {
    pub components: Vec<ComponentParams>,
    pub components_names: HashMap<String, usize>,
}

impl Phase {
    pub fn new(components_from_phase: Vec<Component>) -> Self {
        let mut components = Vec::with_capacity(components_from_phase.len());
        let mut components_names = HashMap::new();
        for (index, component) in components_from_phase.into_iter().enumerate() {
            components.push(component.params);
            if let Some(name) = component.name {
                components_names.insert(name, index);
            }
        }
        Self { components, components_names }
    }
}
