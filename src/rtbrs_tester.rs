use vst::plugin::{Category, Info, Plugin, HostCallback};

use crate::editor::Editor;

pub struct RtbrsTester {
    editor: Editor,
}

impl Default for RtbrsTester {
    fn default() -> Self {
        RtbrsTester::new(HostCallback::default())
    }
}

impl Plugin for RtbrsTester {
    fn new(_host: HostCallback) -> Self {
        Self {
            editor: Editor::new(),
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "rtbrs-tester".to_string(),
            vendor: "crsaracco".to_string(),
            unique_id: 1147000002, // Make sure this is a unique number across all of your VSTs!
            category: Category::Synth,
            inputs: 0,
            midi_inputs: 1,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }
}