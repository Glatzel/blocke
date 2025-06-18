use rax::io::devices::{self, DeviceFilter};

fn main() -> miette::Result<()> {
    let devices = devices::list_devices(DeviceFilter::all)?;
    for d in devices {
        println!("{:#?}", d);
    }
    Ok(())
}
