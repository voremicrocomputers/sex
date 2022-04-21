use alloc::vec::Vec;
use uefi::proto::console::gop::BltPixel;
use crate::serial::{command, read};
use pci_devices::PciDeviceClass;
use crate::audio::pci_devices::pci_read;
use crate::gfx::draw_box;

mod pci_devices; // many thanks to github.com/trashbyte for doing all the hard work for
                 // pci stuff, i'm lazy so i'm kinda just copying it a bit

struct AudioDevice {
    pub device: u8,
    pub bus: u8,
    pub bar0: u32,
    pub bar1: u32,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
}

fn check_device(bus: u8, device: u8) -> Option<AudioDevice> {
    assert!(device < 32);
    let function: u8 = 0;
    let mut device_id: u16 = 0;
    let mut vendor_id: u16 = 0;
    let temp = pci_read(bus, device, function, 0x00);
    device_id = ((temp >> 16) & 0xFFFF) as u16;
    vendor_id = (temp & 0xFFFF) as u16;
    if vendor_id == 0xFFFF {
        draw_box(0,0,50,50, BltPixel::new(255,255,0));
        return None;
    }
    let mut class: u32 = 0;
    let temp = pci_read(bus, device, 0, 0x8);
    class = (temp >> 16) & 0x0000FFFF;
    let mut class_code: PciDeviceClass = PciDeviceClass::from_u16(class as u16);
    if class_code != PciDeviceClass::Multimedia_AudioController { // we only care about audio controllers
        draw_box(0,0,50,50, BltPixel::new(0,255,255));
        return None;
    }
    /*
    let mut header_type: u8 = 0;
    let temp = pci_read(bus, device, function, 0x0C);
    header_type = ((temp >> 16) & 0xFF) as u8;

     */

    let mut bar0: u32 = 0;
    let mut bar1: u32 = 0;
    let mut interrupt_line: u8 = 0;
    let mut interrupt_pin: u8 = 0;

    // read bar0
    bar0 = pci_read(bus, device, function, 0x10);

    // read bar1
    bar1 = pci_read(bus, device, function, 0x14);

    let last_row = pci_read(bus, device, function, 0x3C);
    interrupt_line = (last_row & 0xFF) as u8;
    interrupt_pin = ((last_row >> 8) & 0xFF) as u8;

    draw_box(0,0,50,50, BltPixel::new(0,255,0));
    Some(AudioDevice {
        device,
        bus,
        bar0,
        bar1,
        interrupt_line,
        interrupt_pin,
    })
}

fn scan_for_audio_devices() -> Vec<AudioDevice> {
    let mut devices: Vec<AudioDevice> = Vec::new();
    for bus in 0u8..=255 {
        for device in 0u8..=31 {
            if let Some(device) = check_device(bus, device) {
                devices.push(device);
                break;
            }
        }
    }
    devices
}

pub fn init_audio() -> Result<u8, ()> {
    let devices = scan_for_audio_devices();

    // return the amount of devices found
    Ok(devices.len() as u8)
}