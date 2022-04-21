use core::arch::asm;

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
pub fn command(port: u16, data: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") data, options(nomem, nostack, preserves_flags));
    }
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
pub fn read(port: u16) -> u32 {
    let mut data: u32;
    unsafe {
        asm!("in eax, dx", out("eax") data, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    data
}