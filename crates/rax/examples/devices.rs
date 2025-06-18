use rax::io::{DeviceFilter, list_devices};

fn main() -> miette::Result<()> {
    let devices = list_devices(DeviceFilter::all)?;
    for d in devices {
        println!("{:#?}", d);
    }
    Ok(())
}
