use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;
use tinypci::*;
use crate::serial::*;

struct BDLEntry {
    addr: u32,
    sample_count: u16,
    idfk: u16,
}

// returns true if bar is memory mapped, false if it is I/O mapped
pub fn check_bar_type(bar: u32) -> bool {
    let mut bar_type = false;
    if bar & 0x1 == 0 {
        bar_type = true;
    }
    bar_type
}

pub fn write_to_barB(bar: u32, bar_type: bool, data: u8) {
    if bar_type { // memory mapped
        let address = bar & 0xFFFFFFF0;
        unsafe {
            let ptr = address as *mut u8;
            *ptr = data;
        }
    } else { // io mapped
        let address = bar & 0xFFFFFFFC;
        let port = address as u16;
        commandB(port, data);
    }
}

pub fn read_from_barB(bar: u32, bar_type: bool) -> u8 {
    if bar_type { // memory mapped
        let address = bar & 0xFFFFFFF0;
        unsafe {
            let ptr = address as *mut u8;
            *ptr
        }
    } else { // io mapped
        let address = bar & 0xFFFFFFFC;
        let port = address as u16;
        readB(port)
    }
}

pub fn write_to_barW(bar: u32, bar_type: bool, data: u16) {
    if bar_type { // memory mapped
        let address = bar & 0xFFFFFFF0;
        unsafe {
            let ptr = address as *mut u16;
            *ptr = data;
        }
    } else { // io mapped
        let address = bar & 0xFFFFFFFC;
        let port = address as u16;
        commandW(port, data);
    }
}

pub fn read_from_barW(bar: u32, bar_type: bool) -> u16 {
    if bar_type { // memory mapped
        let address = bar & 0xFFFFFFF0;
        unsafe {
            let ptr = address as *mut u16;
            *ptr
        }
    } else { // io mapped
        let address = bar & 0xFFFFFFFC;
        let port = address as u16;
        readW(port)
    }
}

pub fn write_to_barD(bar: u32, bar_type: bool, data: u32) {
    if bar_type { // memory mapped
        let address = bar & 0xFFFFFFF0;
        unsafe {
            let ptr = address as *mut u32;
            *ptr = data;
        }
    } else { // io mapped
        let address = bar & 0xFFFFFFFC;
        let port = address as u16;
        commandL(port, data);
    }
}

pub fn read_from_barD(bar: u32, bar_type: bool) -> u32 {
    if bar_type { // memory mapped
        let address = bar & 0xFFFFFFF0;
        unsafe {
            let ptr = address as *mut u32;
            *ptr
        }
    } else { // io mapped
        let address = bar & 0xFFFFFFFC;
        let port = address as u16;
        readL(port)
    }
}

pub fn init_one_audio(device: PciDeviceInfo, pcm: &[u8]) -> Result<(),()> {
    // we're just gonna hope and pray that this device is an ac97
    let bar0 = device.bars[0];
    let bar1 = device.bars[1];

    // check if bars are a memory mapped register or a port mapped register
    let bar0_type = check_bar_type(bar0);
    let bar1_type = check_bar_type(bar1);

    // send 0x2 to the nabm global control register
    write_to_barD(bar1 + 0x2C, bar1_type, 0x2);

    // reset all streams
    write_to_barB(bar1 + 0x0B, bar1_type, 0x2);
    write_to_barB(bar1 + 0x1B, bar1_type, 0x2);
    write_to_barB(bar1 + 0x2B, bar1_type, 0x2);

    write_to_barB(bar1 + 0x15, bar1_type, 0x0);

    // reset device
    write_to_barW(bar0, bar0_type, 0xFF);

    // set volume to max
    write_to_barW(bar1 + 0x02, bar1_type, 0xFFFF);
    write_to_barW(bar1 + 0x18, bar1_type, 0xFFFF);

    // start constructing a buffer descriptor list entry
    let bdl_entry = BDLEntry {
        addr: pcm.as_ptr() as u32,
        sample_count: 100, // hardcoded for now
        idfk: 0,
    };

    // make it an array of BDLEntry
    let bdl: [BDLEntry; 1] = [bdl_entry];

    // set reset bit NABM 0x1B to 0x2 and wait for it to be cleared
    write_to_barB(bar1 + 0x1B, bar1_type, 0x2);
    while read_from_barB(bar1 + 0x1B, bar1_type) & 0x2 == 0x2 {
        continue;
    }

    // write address of buffer descriptor list to NABM 0x10
    write_to_barD(bar1 + 0x10, bar1_type, bdl.as_ptr() as u32);
    // write address of entry 0 to NABM 0x15
    write_to_barD(bar1 + 0x15, bar1_type, &bdl[0] as *const BDLEntry as u32);

    // set transfer bit
    write_to_barB(bar1 + 0x1B, bar1_type, 0x1);

    Ok(())
}

pub fn init_all_audio() -> Result<u8, ()> {
    let devices = brute_force_scan();
    let audio_devices: Vec<PciDeviceInfo>;

    // iterate through devices and find only the ones with PciFullClass::Multimedia_AudioController
    audio_devices = devices
        .iter()
        .filter(|device| device.full_class == PciFullClass::Multimedia_AudioController)
        .map(|device| device.clone())
        .collect();

    let audio = include_bytes!("../../vox2.wav");
    let device = PciDeviceInfo{
        device: audio_devices[0].device,
        bus: audio_devices[0].bus,
        device_id: 0,
        vendor_id: 0,
        full_class: PciFullClass::Unclassified_NonVgaCompatible,
        header_type: 0,
        bars: [1,1,1,1,1,1],
        supported_fns: [true, true, true, true, true, true, true, true],
        interrupt_line: 0,
        interrupt_pin: 0
    };
    init_one_audio(device, audio);

    // return the amount of devices found
    Ok(audio_devices.len() as u8)
}