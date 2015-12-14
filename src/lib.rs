
#[macro_use]
extern crate xplane_plugin;
use xplane_plugin::*;
extern crate xplm;

mod test_plugin;
mod probe;

xplane_plugin!(test_plugin::TestPlugin);
