extern crate log;
extern crate rtb_rs;
extern crate simplelog;
extern crate vst;

use vst::plugin_main;

mod editor;
mod rtbrs_tester;

plugin_main!(rtbrs_tester::RtbrsTester);
