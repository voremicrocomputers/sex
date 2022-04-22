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

pub fn init_one_audio(device: PciDeviceInfo, pcm: &[u8]) -> Result<(),()> {
    // we're just gonna hope and pray that this device is an ac97
    // reset the device
    // resume from cold reset (BAR0 + length + 0x2C) global control register
    commandB(device.bars[0] as u16 + 0x4 + 0x2C, 0x2);

// reset all streams
    commandB(device.bars[0] as u16 + 0x4 + 0x0B, 0x2);
    commandB(device.bars[0] as u16 +  0x4 + 0x1B, 0x2);
    commandB(device.bars[0] as u16 +  0x4 + 0x2B, 0x2);
    commandB(device.bars[0] as u16 +  0x4 + 0x15, 0x0);

    // send anything to the reset register
    commandB(device.bars[0] as u16, 0xFF);
    // set output volume
    commandB(device.bars[0] as u16 +  0x02, 0x0);
    // set pcm volume
    commandB(device.bars[0] as u16 +  0x18, 0x0);
    // load pcm into the Buffer Descriptor List
    let our_sound = BDLEntry{
        addr: pcm.as_ptr() as u32,
        sample_count: pcm.len() as u16,
        idfk: 0,
    };
    let mut bdl = [our_sound; 1];
    // write the bdl to the device (BAR0 + lenght of BAR0 + NAMB register for pcm out + physical address of bdl)
    commandL(device.bars[1] as u16 +  0x10 + 0x00, bdl.as_ptr() as u32);
    // enable extended functions
    commandB(device.bars[0] as u16 +  0x2A, 0x1);
    // sample rate: 44100
    commandW(device.bars[0] as u16 +  0x2C, 44100);
    commandW(device.bars[0] as u16 +  0x2E, 44100);
    commandW(device.bars[0] as u16 +  0x30, 44100);
    commandW(device.bars[0] as u16 +  0x32, 44100);

    // play sound
    commandB(device.bars[1] as u16 +  0x1B, 0x1);
    commandB(device.bars[1] as u16 +  0x16, 0x1C);

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