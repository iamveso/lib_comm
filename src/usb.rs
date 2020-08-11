use rusb::{Context, DeviceHandle, DeviceList, DeviceDescriptor, TransferType, ConfigDescriptor, Interfaces, Interface, InterfaceDescriptors, InterfaceDescriptor, EndpointDescriptor, Direction, EndpointDescriptors, Device};

pub struct UsbDeviceIdentity{
    vendor_id: u16,
    product_id: u16,
    handle: DeviceHandle<Context>,
    //TODO:Endpoints are part of the devices identity
}

impl UsbDeviceIdentity{
    pub fn print_struct(&self){
        println!("Vendor ID is: {}", self.vendor_id);
        println!("Product ID is: {}", self.product_id);
        //TODO:Print out device endpoints
    }
}

struct Endpoint {dir_in: u8, dir_out: u8}

pub fn create_device_identity() -> Result<UsbDeviceIdentity,rusb::Error>{
    let handle = find_connected_devices()?;
    let vendor_id = get_vendor_id(&handle)?;
    let product_id = get_product_id(&handle)?;
    //let endpoints = get_endpoints(&handle)?;
    let device_identity = UsbDeviceIdentity{
        vendor_id,
        product_id,
        handle,
        //endpoint: endpoints,
    };
    Ok(device_identity)
}

///Returns a Result<> of device handle on success
pub fn find_connected_devices() -> Result<DeviceHandle<Context>, rusb::Error>{
    let context = Context::new()?;
    let list = DeviceList::new_with_context(context)?;
    get_device_handle(list)
}

fn get_device_handle(list: DeviceList<Context>) -> Result<DeviceHandle<Context>, rusb::Error>{
    for device in list.iter(){
        let handle = device.open()?;
        //TODO:Get interface number dynamically
        let attached = handle.kernel_driver_active(0)?;
        if !attached{
            return Ok(handle)
        }
    }
    Err(rusb::Error::NotFound)
}

fn get_vendor_id(device_handle: &DeviceHandle<Context>) -> Result<u16,rusb::Error>{
    let desc = get_descriptor(device_handle)?;
    Ok(desc.vendor_id())
}

fn get_product_id(device_handle: &DeviceHandle<Context>) -> Result<u16,rusb::Error>{
    let desc = get_descriptor(device_handle)?;
    Ok(desc.product_id())
}