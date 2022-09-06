use std::collections::HashMap;

use kosem_webapi::phase_control_messages::Component;

#[derive(Debug)]
pub struct Phase {
    pub components: Vec<Component>,
    pub components_names: HashMap<String, usize>,
}

impl Phase {
    pub fn new(components: Vec<Component>) -> Self {
        let mut components_names = HashMap::new();
        for (index, component) in components.iter().enumerate() {
            if let Some(ref name) = component.name {
                components_names.insert(name.to_owned(), index);
            }
        }
        Self {
            components,
            components_names,
        }
    }
}
