pub mod usb;

pub trait Identity{
    fn get_device_identity<T>() -> T;
}

//TODO:pub trait send_data
//TODO:pub trait receive_data

#[cfg(test)]
mod tests {
    use rusb::{DeviceHandle, Context};
    use crate::usb;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    //TODO:Run test for can find devices

}
