use crate::serial::{command, read};

#[derive(PartialEq)]
pub enum PciDeviceClass {
    Multimedia_AudioController = 0x0401,

    Unknown = 0xFFFF,
}

impl PciDeviceClass {
    pub fn from_u16(n: u16) -> PciDeviceClass {
        match n {
            0x0401 => PciDeviceClass::Multimedia_AudioController,

            _ => PciDeviceClass::Unknown
        }
    }
    //pub fn as_u16(&self) -> u16 { *self as u16 }
}

pub fn pci_read(bus: u8, device: u8, func: u8, offset: u8) -> u32 {
    let bus = bus as u32;
    let device = device as u32;
    let func = func as u32;
    let offset = offset as u32;

    let address = ((bus << 16) | (device << 11) | (func << 8) | (offset & 0xFC) | 0x80000000) as u32;

    command(0xCF8, address);
    read(0xCFC)
}

pub fn pci_write(bus: u8, device: u8, func: u8, offset: u8, value: u32) {
    let bus = bus as u32;
    let device = device as u32;
    let func = func as u32;
    let offset = offset as u32;

    let address = ((bus << 16) | (device << 11) | (func << 8) | (offset & 0xFC) | 0x80000000) as u32;

    command(0xCF8, address);
    command(0xCFC, value);
}