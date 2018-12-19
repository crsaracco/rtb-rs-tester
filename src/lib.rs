extern crate vst;
extern crate rtb_rs;
extern crate log;
extern crate simplelog;

use vst::plugin_main;

mod rtbrs_tester;
mod editor;

plugin_main!(rtbrs_tester::RtbrsTester);