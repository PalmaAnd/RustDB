use log::info;

mod disk;
mod error;
mod page;

fn main() {
    env_logger::init();
    info!("RustDB starting ...");
}
