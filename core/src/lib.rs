use lazy_static::lazy_static;
use windows::{core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA};

lazy_static! {
    static ref IMAGE_BASE: usize = unsafe { GetModuleHandleA(PCSTR(std::ptr::null())).0 as usize };
}

#[inline]
pub fn get_address(rva: usize) -> usize {
    *IMAGE_BASE + rva
}
