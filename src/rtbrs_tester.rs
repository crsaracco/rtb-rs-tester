use vst::plugin::{Category, Info, Plugin, HostCallback};
use log::*;
use std::fs::File;

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
        // Set up a logger so we can see what's going on in the VST
        let mut logger_config = simplelog::Config::default();
        logger_config.time_format = Some("%H:%M:%S%.6f");

        simplelog::CombinedLogger::init(
            vec![
                simplelog::WriteLogger::new(
                    simplelog::LevelFilter::max(),
                    logger_config,
                    File::create("/tmp/plugin.log").unwrap()
                ),
            ]
        ).unwrap();

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