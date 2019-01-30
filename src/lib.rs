extern crate log;
extern crate rtb_rs;
extern crate simplelog;
extern crate vst;

use vst::plugin_main;

mod rtbrs_tester;
mod editor;
mod event_handler;
mod parameters;
mod atomic_float;

plugin_main!(rtbrs_tester::RtbrsTester);
