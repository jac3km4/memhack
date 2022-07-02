use once_cell::sync::Lazy;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

static IMAGE_BASE: Lazy<usize> =
    Lazy::new(|| unsafe { GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 as usize });

#[inline]
pub fn resolve_rva(rva: usize) -> usize {
    *IMAGE_BASE + rva
}
