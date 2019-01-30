use std::fs::File;
use std::sync::Arc;

use log::*;
use vst::plugin::{Category, HostCallback, Info, Plugin};

use crate::editor::Editor;
use crate::parameters::Parameters;

pub struct RtbrsTester {
    host: HostCallback,
    editor: Editor,
    parameters: Arc<Parameters>,
}

impl Default for RtbrsTester {
    fn default() -> Self {
        RtbrsTester::new(HostCallback::default())
    }
}

impl Plugin for RtbrsTester {
    fn new(host: HostCallback) -> Self {
        // Set up a logger so we can see what's going on in the VST
        let mut logger_config = simplelog::Config::default();
        logger_config.time_format = Some("%H:%M:%S%.6f");

        simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
            simplelog::LevelFilter::max(),
            logger_config,
            File::create("/tmp/plugin.log").unwrap(),
        )])
        .unwrap();

        let parameters = Arc::new(Parameters::new());

        Self {
            host,
            editor: Editor::new(parameters.clone()),
            parameters: parameters.clone(),
        }
    }

    fn get_info(&self) -> Info {
        info!("RtbrsTester::get_info()");
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

    fn init(&mut self) {
        info!("RtbrsTester::init()");
        info!(
            "RtbrsTester::init() -- host VST version: {}",
            self.host.vst_version()
        );
    }

    // TODO: return None if the editor couldn't be created
    // (for example, if the connection to the X server couldn't be established)
    fn get_editor(&mut self) -> Option<&mut vst::editor::Editor> {
        //info!("RtbrsTester::get_editor()");
        Some(&mut self.editor)
    }
}
