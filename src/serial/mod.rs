use core::arch::asm;

pub fn commandB(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al", in("al") data, in("dx") port);
    }
}

pub fn readB(port: u16) -> u8 {
    let mut data: u8;
    unsafe {
        asm!("in al, dx", out("al") data, in("dx") port);
    }
    data
}

pub fn commandW(port: u16, data: u16) {
    unsafe {
        asm!("out dx, ax", in("ax") data, in("dx") port);
    }
}

pub fn readW(port: u16) -> u16 {
    let mut data: u16;
    unsafe {
        asm!("in ax, dx", out("ax") data, in("dx") port);
    }
    data
}

pub fn commandL(port: u16, data: u32) {
    unsafe {
        asm!("out dx, eax", in("eax") data, in("dx") port);
    }
}

pub fn readL(port: u16) -> u32 {
    let mut data: u32;
    unsafe {
        asm!("in eax, dx", out("eax") data, in("dx") port);
    }
    data
}