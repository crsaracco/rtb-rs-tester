use std::sync::Arc;
use log::*;

use crate::parameters::Parameters;

pub struct EventHandler {
    parameters: Arc<Parameters>,
}

impl EventHandler {
    pub fn new(parameters: Arc<Parameters>) -> Self {
        Self {
            parameters,
        }
    }
}

impl rtb_rs::event::EventHandler for EventHandler {
    fn handle(&self, event: rtb_rs::Event) {
        info!("Event: {:?}", event);
    }
}