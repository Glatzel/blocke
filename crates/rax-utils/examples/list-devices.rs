use clerk::tracing::level_filters::LevelFilter;
use rax_utils::devices::{DeviceFilter, list_devices};
use test_utils::init_log_with_level;

fn main() -> miette::Result<()> {
    init_log_with_level(LevelFilter::TRACE);
    let devices = list_devices(DeviceFilter::all)?;
    for d in devices {
        println!("{:#?}", d);
    }
    Ok(())
}
