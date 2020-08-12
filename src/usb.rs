#![allow(dead_code)]


use rusb::{Context, DeviceHandle, DeviceList, DeviceDescriptor};
use std::time::Duration;
use std::thread::sleep;

enum UsbType{
    Accessory,
    Other,
}

const ACCESSORY_VENDOR_ID :u16 = 0x18D1;
const ACCESSORY_PRODUCT_ID :(u16,u16) = (0x2D00,0x2D01);

pub struct UsbDeviceIdentity{
    vendor_id: u16,
    product_id: u16,
    handle: DeviceHandle<Context>,
    //TODO:Endpoints are part of the devices identity
}

//Methods
impl UsbDeviceIdentity{
    pub fn show(&self){
        println!("Vendor ID is: {}", self.vendor_id);
        println!("Product ID is: {}", self.product_id);
        //TODO:Print out device endpoints
    }
    pub fn get_vendor_id(&self) -> u16{
        self.vendor_id
    }
    pub fn get_product_id(&self) -> u16{
        self.product_id
    }
    pub fn send_data(&self, data: &str) -> bool{unimplemented!()}
    pub fn receive_data(&self) -> String{unimplemented!()}
    pub fn is_in_accessory_mode(&self) -> bool{
        if self.vendor_id == ACCESSORY_VENDOR_ID &&
            (self.product_id == ACCESSORY_PRODUCT_ID.0 ||
                self.product_id == ACCESSORY_PRODUCT_ID.1)  {
            return true;
        }
        false
    }
}

impl UsbDeviceIdentity{
    pub fn new() -> Option<UsbDeviceIdentity>{
        let device_identity = create_device_identity();
        return match device_identity {
            Ok(s) => Some(s),
            Err(e) => {
                println!("FAILED TO CREATE DEVICE IDENTITY WITH ERROR: {}", e);
                None
            },
        }
    }
}

fn create_device_identity() -> Result<UsbDeviceIdentity,rusb::Error>{
    let handle = find_connected_devices()?;
    let vendor_id = get_vendor_id(&handle)?;
    let product_id = get_product_id(&handle)?;
    let device_identity = UsbDeviceIdentity{
        vendor_id,
        product_id,
        handle,
    };
    Ok(device_identity)
}

///Returns a Result<> of device handle on success
fn find_connected_devices() -> Result<DeviceHandle<Context>, rusb::Error>{
    let context = Context::new()?;
    let list = DeviceList::new_with_context(context)?;
    get_device_handle(list)
}

fn get_device_handle(list: DeviceList<Context>) -> Result<DeviceHandle<Context>, rusb::Error>{
    for device in list.iter(){
        let mut handle = device.open()?;
        //TODO:Get interface number dynamically
        let attached = handle.kernel_driver_active(0)?;
        if !attached{
            handle.claim_interface(0)?;
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

fn get_descriptor(device_handle: &DeviceHandle<Context>) -> Result<DeviceDescriptor,rusb::Error>{
    let device = device_handle.device();
    let descriptor = device.device_descriptor()?;
    Ok(descriptor)
}

fn get_protocol_version(handle: &DeviceHandle<Context>) -> Result<usize,rusb::Error>{
    let mut data:[u8;2] = [0,0];
    let time = Duration::new(0,0);
    let version = handle.read_control(0xC0,51,0,0,&mut data,time)?;
    Ok(version)
}

fn is_protocol_version_supported(version: usize) -> bool{
    if version != 1 && version != 2{
        return false;
    }
    true
}

pub fn switch_to_accessory_mode(device: &UsbDeviceIdentity, control_strings: &(String,String,String,String,String,String)) -> Result<(),rusb::Error>{
    let timeout = Duration::new(0,0);
    let sleep_time = Duration::new(1,500000);
    let a = get_protocol_version(&device.handle)?;
    if !is_protocol_version_supported(a){
        return Err(rusb::Error::NotSupported);
    }
    sleep(sleep_time);
    device.handle.write_control(0x40,52,0,0,control_strings.0.as_bytes(),timeout)?;
    device.handle.write_control(0x40,52,0,1,control_strings.1.as_bytes(),timeout)?;
    device.handle.write_control(0x40,52,0,2,control_strings.2.as_bytes(),timeout)?;
    device.handle.write_control(0x40,52,0,3,control_strings.3.as_bytes(),timeout)?;
    device.handle.write_control(0x40,52,0,4,control_strings.4.as_bytes(),timeout)?;
    device.handle.write_control(0x40,52,0,5,control_strings.5.as_bytes(),timeout)?;
    device.handle.write_control(0x40,53,0,0,String::new().as_bytes(),timeout)?;
    sleep(sleep_time);
    Ok(())

}