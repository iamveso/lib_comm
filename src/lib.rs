use crate::usb::UsbDeviceIdentity;

pub mod usb;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usb::switch_to_accessory_mode;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    //Tested with huawei y9s
    fn can_verify_device_is_found(){
        let device = usb::UsbDeviceIdentity::new();
        if let Some(i) = device{
            assert_eq!(0x12d1,i.get_vendor_id());
            assert_eq!(0x12d1,i.get_product_id());
            assert_eq!(false,i.is_in_accessory_mode())
        }else {
            assert!(false)
        }
    }
    #[test]
    fn can_switch_device_to_accessory_mode(){
        let data = (
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
            String::from("E"),
            String::from("F"),
            );
        let device = usb::UsbDeviceIdentity::new();
        let device = match device {
            Some(s) => s,
            None => panic!("Failed to create Device"),
        };
        loop {
            if device.is_in_accessory_mode() {
                println!("In Accessory Mode");
                assert!(true)
            }else {
                switch_to_accessory_mode(&device,&data);
            }
        }
    }

}
