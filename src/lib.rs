
#[macro_use]
extern crate xplm;

mod test_plugin;
mod probe;

xplane_plugin!(test_plugin::TestPlugin);
