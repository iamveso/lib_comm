use rusb::{Context, DeviceHandle, DeviceList, DeviceDescriptor, TransferType, ConfigDescriptor, Interfaces, Interface, InterfaceDescriptors, InterfaceDescriptor, EndpointDescriptor, Direction, EndpointDescriptors};

struct UsbDeviceIdentity{
    vendor_id: u16,
    product_id: u16,
    handle: DeviceHandle<Context>,
    endpoint: Endpoint,
}

struct Endpoint {dir_in: u8, dir_out: u8}

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

fn get_descriptor(device_handle: &DeviceHandle<Context>) -> Result<DeviceDescriptor,rusb::Error> {
    let device = device_handle.device();
    let descriptor = device.device_descriptor()?;
    Ok(descriptor)
}

fn get_bulk_endpoint_address(endpoint_descriptor: EndpointDescriptors) -> Result<Endpoint,rusb::Error>{
    let mut e_in = Option::None;
    let mut e_out = Option::None;
    for endpoint in endpoint_descriptor{
        if endpoint.transfer_type() != TransferType::Bulk {
            continue;
        }
        if endpoint.direction() == Direction::In{
            e_in = Some(endpoint.address());
        }
        if endpoint.direction() == Direction::Out {
            e_out = Some(endpoint.address());
        }
    }

    let e_in =  match e_in {
        Some(s) => s,
        None => return Err(rusb::Error::Other),
    };

    let e_out = match e_out {
        Some(s) => s,
        None => return Err(rusb::Error::Other),
    };

    return Ok(Endpoint{
        dir_in: e_in,
        dir_out: e_out,
    })
}