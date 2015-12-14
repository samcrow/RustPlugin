use xplane_plugin::{Plugin, PluginInfo};
extern crate xplm;
use xplm::debug;
use xplm::data::*;
use xplm::dataref::*;
use xplm::flight_loop::*;

pub struct TestPlugin {
    dataref: DataRef<f32, ReadOnly>,
    flight_loop: Option<FlightLoop>,
}

impl Plugin for TestPlugin {

    fn start() -> Option<Self> {
        debug("Test Rust plugin starting\n");
        match DataRef::find("sim/time/total_running_time_sec") {
            Ok(dataref) => {
                Some(TestPlugin {
                    dataref: dataref,
                    flight_loop: None,
                })
            },
            Err(e) => {
                debug(&format!("Failed to find dataref: {:?}\n", e));
                None
            }
        }
    }
    /// Called when the plugin is enabled
    fn enable(&mut self) {
        debug("Test Rust plugin enabling\n");
        debug(&format!("running time = {}\n", self.dataref.get()));

        // Get the aircraft tail number
        let tail_number_dataref: DataRef<String, ReadOnly> = DataRef::find("sim/aircraft/view/acf_tailnum").unwrap();
        debug(&format!("Tail number = {}\n", tail_number_dataref.get()));

        let dataref_copy = self.dataref.clone();
        let flight_loop = FlightLoop::new(Phase::AfterFlightModel, move || {
            debug(&format!("Flight loop callback running. time = {}\n", dataref_copy.get()));
            NextCallback::after_seconds(1.0)
        });
        flight_loop.schedule(NextCallback::after_seconds(1.0));
        self.flight_loop = Some(flight_loop);
    }
    /// Called when the plugin is disabled
    fn disable(&mut self) {
        debug("Test Rust plugin disabling\n");
        self.flight_loop = None;
    }

    fn stop(&mut self) {
        debug("Test Rust plugin stopping\n");
    }

    /// Returns information on this plugin
    fn info<'a, 'b, 'c>(&self) -> PluginInfo<'a, 'b, 'c> {
        PluginInfo {
            name: "Rust Plugin",
            signature: "org.samcrow.rustplugin",
            description: "A plugin written in Rust",
        }
    }
}

impl Drop for TestPlugin {
    fn drop(&mut self) {
        debug("Test Rust plugin dropped\n");
    }
}
