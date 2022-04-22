#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput, PixelFormat};
use crate::audio::init_all_audio;
use crate::gfx::draw_box;

mod gfx;
mod audio;
mod serial;

pub struct SexInfo {
    pub page_two: Mutex<Vec<BltPixel>>,
    pub width: Mutex<usize>,
    pub height: Mutex<usize>,
    pub pixel_width: Mutex<usize>,
    pub pitch: Mutex<usize>,
    pub colour_type: Mutex<u8>,
}

lazy_static! {
    pub static ref SEXINFO: SexInfo = SexInfo {
        page_two: Mutex::new(Vec::new()),
        width: Mutex::new(0),
        height: Mutex::new(0),
        pixel_width: Mutex::new(0),
        pitch: Mutex::new(0),
        colour_type: Mutex::new(0),
    };
}

// for updating after drawing to the temporary buffer
pub fn update_videobuffer(gop: &mut GraphicsOutput) {
    let mut page_two = SEXINFO.page_two.lock();
    let page_two = page_two.as_mut_slice();

    let res = gop.current_mode_info().resolution();

    // we're going to assume that the gop has been set up correctly
    gop.blt(BltOp::BufferToVideo {
        buffer: page_two,
        src: BltRegion::Full,
        dest: (0, 0),
        dims: (res.0, res.1)
    });
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // first things first, initialise SEXINFO with the GOP
    let mut gop = system_table.boot_services().locate_protocol::<GraphicsOutput>().unwrap();
    let mut gop = unsafe { &mut *gop.get() };

    // querymode to do stuff idk
    let mode = gop.query_mode(0).unwrap();

    // allocate a buffer for the page two
    let mut page_two = Vec::with_capacity(gop.current_mode_info().resolution().0 * gop.current_mode_info().resolution().1);
    page_two.resize(gop.current_mode_info().resolution().0 *gop.current_mode_info().resolution().1, BltPixel::new(0, 0, 0));
    *SEXINFO.page_two.lock() = page_two;

    // set the width and height
    *SEXINFO.width.lock() = gop.current_mode_info().resolution().0;
    *SEXINFO.height.lock() = gop.current_mode_info().resolution().1;

    // set the pixel width
    let pixel_width = gop.current_mode_info().pixel_format();
    match pixel_width {
        PixelFormat::Rgb => {
            *SEXINFO.pixel_width.lock() = 4;
            *SEXINFO.colour_type.lock() = 1;
        },
        PixelFormat::Bgr => {
            *SEXINFO.pixel_width.lock() = 4;
            *SEXINFO.colour_type.lock() = 0;
        },
        _ => panic!("unsupported pixel format"), // todo: handle this better (todo: write a better comment)
    }

    // set the pitch
    *SEXINFO.pitch.lock() = gop.current_mode_info().stride() * *SEXINFO.pixel_width.lock();

    draw_box(50,50,100,100,BltPixel::new(255,255,255));

    let audio_devices = init_all_audio().unwrap();
    if audio_devices > 0 {
        draw_box(50,150,100,100,BltPixel::new(0,255,0));
    } else {
        draw_box(50,150,100,100,BltPixel::new(255,0,0));
    }

    // update the videobuffer
    update_videobuffer(gop);

    // pause
    system_table.boot_services().stall(1000000000000);

    Status::SUCCESS
}