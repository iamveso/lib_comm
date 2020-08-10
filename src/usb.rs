use rusb::{Context, DeviceHandle, DeviceList, DeviceDescriptor, TransferType, ConfigDescriptor, Interfaces, Interface, InterfaceDescriptors, InterfaceDescriptor, EndpointDescriptor, Direction, EndpointDescriptors, Device};

struct UsbDeviceIdentity{
    vendor_id: u16,
    product_id: u16,
    handle: DeviceHandle<Context>,
    endpoint: Endpoint,
}

struct Endpoint {dir_in: u8, dir_out: u8}

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
        let interace_number = get_interface_number(&device)?;
        let attached = handle.kernel_driver_active(interace_number)?;
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

fn get_interface_number(device: &Device<Context>) -> Result<u8, rusb::Error>{
    let config_descriptor = get_config_descriptor(device)?;
    let interfaces = get_interfaces(&config_descriptor);
    for i in interfaces{
        let desc = get_interface_descriptor(i.descriptors())?;
        let output = desc.interface_number();
        return Ok(output);
    }
    Err(rusb::Error::Other)
}

fn get_config_descriptor(device: &Device<Context>) -> Result<ConfigDescriptor,rusb::Error>{
    device.active_config_descriptor()
}

fn get_interfaces(configuration: &ConfigDescriptor) -> Interfaces {
    configuration.interfaces()
}

fn get_interface_descriptor(interface_descriptors: InterfaceDescriptors) -> Result<InterfaceDescriptor,rusb::Error>{
    for i in interface_descriptors{
        let e_descriptor = i.endpoint_descriptors();
        let is_bulk = get_bulk_endpoint_address(e_descriptor);
        match is_bulk {
            Ok(s) => {
                return Ok(i);
            },
            Err(_) => continue
        }
    }
    Err(rusb::Error::BadDescriptor)
}

fn get_bulk_endpoint_address(endpoint_descriptor: EndpointDescriptors) -> Result<Endpoint,rusb::Error>{
    let mut e_in = Option::None;
    let mut e_out = Option::None;
    for endpoint in endpoint_descriptor{
        if is_bulk_transfer_type(&endpoint) {
            continue;
        }
        if endpoint.direction() == Direction::In{
            e_in = Some(endpoint.address());
        }
        if endpoint.direction() == Direction::Out {
            e_out = Some(endpoint.address());
        }
    }

    let e_in = get_endpoint_address(e_in)?;
    let e_out = get_endpoint_address(e_out)?;

    return Ok(add_endpoints(e_in,e_out))
}

fn is_bulk_transfer_type(endpoint: &EndpointDescriptor) -> bool{
    if endpoint.transfer_type() == TransferType::Bulk{
        return true;
    }
    false
}

fn get_endpoint_address(endpoint: Option<u8>) -> Result<u8,rusb::Error>{
    return match endpoint{
        Some(s) => Ok(s),
        None => Err(rusb::Error::Other),
    }
}

fn add_endpoints(endpoint_in: u8, endpoint_out: u8) -> Endpoint{
    Endpoint{
        dir_in: endpoint_in,
        dir_out: endpoint_out,
    }
}