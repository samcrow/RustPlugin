use xplane_plugin::{Plugin, PluginInfo};
extern crate xplm;
use xplm::debug;
use xplm::data::*;
use xplm::data::dataref::*;
use xplm::flight_loop::*;
use xplm::ui::Rect;
use xplm::ui::widget::{Widget, Window, Pane, Button, CheckBox};

use probe::ProbeTestHolder;

pub struct TestPlugin {
    dataref: DataRef<f32, ReadOnly>,
    flight_loop: Option<FlightLoop>,
    probe_test: Option<ProbeTestHolder>,
    window: Option<Window>,
}

impl Plugin for TestPlugin {

    fn start() -> Option<Self> {
        xplm::enable_debug_logging();
        debug("Test Rust plugin starting\n");
        match DataRef::find("sim/time/total_running_time_sec") {
            Ok(dataref) => {
                Some(TestPlugin {
                    dataref: dataref,
                    flight_loop: None,
                    probe_test: None,
                    window: None,
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

        self.probe_test = Some(ProbeTestHolder::new());

        let button_listener = || {
            debug("Button pressed\n");
        };
        let checkbox_listener = |checked: bool| {
            debug(&format!("Check box value changed: checked = {}\n", checked));
        };

        let mut window = Window::new("Rust Window", &Rect{ left: 100, top: 400, right: 600, bottom: 100 });
        let pane = Pane::new("Pane", &Rect{ left: 120, top: 370, right: 580, bottom: 120 });

        let button = Button::new("Eat Pie", &Rect{ left: 130, top: 140, right: 200, bottom: 130 }, button_listener);
        let checkbox = CheckBox::new(&Rect{ left: 130, top: 160, right: 200, bottom: 150 }, checkbox_listener);


        window.add_child(Box::new(pane));
        window.add_child(Box::new(button));
        window.add_child(Box::new(checkbox));
        window.set_visible(true);
        self.window = Some(window);
    }
    /// Called when the plugin is disabled
    fn disable(&mut self) {
        debug("Test Rust plugin disabling\n");
        self.flight_loop = None;
        self.probe_test = None;
        self.window = None;
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
