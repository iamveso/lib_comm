use rusb::{Context, DeviceHandle, DeviceList, DeviceDescriptor};

//Struct containing usb device identity
struct UsbDeviceIdentity{
    vendor_id: u16,
    product_id: u16,
    handle: DeviceHandle<Context>,
    endpoints: Endpoint,
}

//Struct for Endpoint
struct Endpoint { dir_in: u8, dir_out: u8}

//Find connected devices
pub fn find_connected_devices() -> Result<DeviceHandle<Context>, rusb::Error>{
    let context = create_device_context()?;
    let list = DeviceList::new_with_context(context)?;
    get_device_handle(list)
}
//Create usb context
pub fn create_device_context() -> Result<Context,rusb::Error>{
    Context::new()
}

//Get device handle
fn get_device_handle(list: DeviceList<Context>) -> Result<DeviceHandle<Context>, rusb::Error>{
    for device in list.iter(){
        let handle = device.open()?;
        //TODO:Get interface number dynamically
        if handle.kernel_driver_active(0)?{
            return Ok(handle)
        }
    }
    Err(rusb::Error::NotFound)
}

fn get_vendor_id(device: &DeviceHandle<Context>) -> Result<u16,rusb::Error>{
    let desc = get_descriptor(device)?;
    Ok(desc.vendor_id())
}

fn get_product_id(device: &DeviceHandle<Context>) -> Result<u16,rusb::Error>{
    let desc = get_descriptor(device)?;
    Ok(desc.product_id())
}

fn get_descriptor(device: &DeviceHandle<Context>) -> Result<DeviceDescriptor,rusb::Error> {
    let device = device.device();
    let descriptor = device.device_descriptor()?;
    Ok(descriptor)
}