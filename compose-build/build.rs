#[macro_use]
extern crate log;
use log::Level;

fn main() {
    env_logger::init();

    info!("Calling compile protos...");
    tonic_build::compile_protos("proto/hellotonic/hellotonic.proto").unwrap();
}
