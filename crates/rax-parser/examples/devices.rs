use rax::devices::{DeviceFilter, list_devices};

fn main() -> miette::Result<()> {
    test_utils::init_log();
    let devices = list_devices(DeviceFilter::all)?;
    for d in devices {
        println!("{:#?}", d);
    }
    Ok(())
}
