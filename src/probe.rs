
use xplm;
use xplm::data::*;
use xplm::dataref::DataRef;
use xplm::terrain::Probe;
use xplm::flight_loop::*;
use xplm::position::*;

///
/// Uses a flight loop callback and a terrain probe to periodically display information about
/// the ground below the aircraft
///
pub struct ProbeTestHolder {
    _flight_loop: FlightLoop,
}

impl ProbeTestHolder {
    pub fn new() -> ProbeTestHolder {
        let probe_test = ProbeTest::new();
        let flight_loop = FlightLoop::new(Phase::AfterFlightModel, probe_test);
        flight_loop.schedule(NextCallback::after_seconds(1.0));
        ProbeTestHolder {
            _flight_loop: flight_loop,
        }
    }
}

struct ProbeTest {
    aircraft_x: DataRef<f64, ReadOnly>,
    aircraft_z: DataRef<f64, ReadOnly>,
    aircraft_y: DataRef<f64, ReadOnly>,
    probe: Probe,
}

impl ProbeTest {
    pub fn new() -> ProbeTest {
        ProbeTest {
            aircraft_x: DataRef::find("sim/flightmodel/position/local_x").unwrap(),
            aircraft_y: DataRef::find("sim/flightmodel/position/local_y").unwrap(),
            aircraft_z: DataRef::find("sim/flightmodel/position/local_z").unwrap(),
            probe: Probe::new(),
        }
    }
}

impl FlightLoopCallback for ProbeTest {
    fn callback(&mut self) -> NextCallback {
        let aircraft_position = Local { x: self.aircraft_x.get(),
                                        y: self.aircraft_y.get(),
                                        z: self.aircraft_z.get() };

        match self.probe.probe(&aircraft_position) {
            Some(result) => {
                xplm::debug(&format!("Probe result: {:?}\n", result));
                let ground_local = result.position;
                let ground_lla = local_to_world(&ground_local);
                xplm::debug(&format!("Ground elevation: {} meters\n", ground_lla.altitude));
            },
            None => xplm::debug("Probe failed\n"),
        };

        NextCallback::after_seconds(1.0)
    }
}
