use alloc::vec::Vec;
use tinypci::*;

pub fn init_audio() -> Result<u8, ()> {
    let devices = brute_force_scan();
    let audio_devices: Vec<PciDeviceInfo>;

    // iterate through devices and find only the ones with PciFullClass::Multimedia_AudioController
    audio_devices = devices
        .iter()
        .filter(|device| device.full_class == PciFullClass::Multimedia_AudioDevice)
        .map(|device| device.clone())
        .collect();

    // return the amount of devices found
    Ok(audio_devices.len() as u8)
}