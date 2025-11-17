#[cfg(target_os = "none")]
use core::ffi::CStr;

#[cfg(target_os = "none")]
use alloc::{ffi, string::String, vec::Vec};

#[cfg(target_os = "none")]
pub fn file_write(filename: &str, content: &[u8]) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileWrite(c_string.as_ptr(), content.as_ptr(), content.len()) }
}

#[cfg(not(target_os = "none"))]
pub fn file_write(_filename: &str, _content: &[u8]) -> bool {
    true
}

#[cfg(target_os = "none")]
pub fn file_exists(filename: &str) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileExists(c_string.as_ptr()) }
}

#[cfg(not(target_os = "none"))]
pub fn file_exists(_filename: &str) -> bool {
    false
}

#[cfg(target_os = "none")]
pub fn file_read(filename: &str) -> Option<Vec<u8>> {
    let c_string = ffi::CString::new(filename).unwrap();
    let mut lenght: usize = 0;
    let array_pointer = unsafe { extapp_fileRead(c_string.as_ptr(), &mut lenght as *mut usize) };

    if array_pointer.is_null() {
        return None;
    }

    Some(unsafe { core::slice::from_raw_parts(array_pointer, lenght).to_vec() })
}

#[cfg(not(target_os = "none"))]
pub fn file_read(_filename: &str) -> Option<Vec<u8>> {
    None
}

// TODO: implement read_file_slice

#[cfg(target_os = "none")]
pub fn file_erase(filename: &str) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileErase(c_string.as_ptr()) }
}

#[cfg(not(target_os = "none"))]
pub fn extapp_file_erase(_filename: &str) -> bool {
    true
}

#[cfg(target_os = "none")]
pub fn file_list_with_extension(max_records: usize, extension: &str) -> Vec<String> {
    let mut filenames: Vec<*mut u8> = Vec::with_capacity(max_records);
    let c_string = ffi::CString::new(extension).unwrap();

    unsafe {
        let final_len = extapp_fileListWithExtension(
            filenames.as_mut_slice().as_mut_ptr(),
            max_records as isize,
            c_string.as_ptr(),
        );
        filenames.set_len(final_len as usize);

        let mut files: Vec<String> = Vec::new();
        for name_ptr in filenames {
            if !name_ptr.is_null() {
                let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();
                files.push(name);
            }
        }

        files
    }
}

#[cfg(not(target_os = "none"))]
pub fn file_list_with_extension(_max_records: usize, _extension: &str) -> Vec<String> {
    Vec::new()
}

pub enum CalculatorModel {
    Unknown,
    N0110N0115,
    N0120,
    Simulator,
}

#[cfg(target_os = "none")]
pub fn get_calculator_model() -> CalculatorModel {
    match unsafe { extapp_calculatorModel() } {
        _ => CalculatorModel::Unknown,
        1 => CalculatorModel::N0110N0115,
        2 => CalculatorModel::N0120,
    }
}

#[cfg(not(target_os = "none"))]
pub fn get_calculator_model() -> CalculatorModel {
    CalculatorModel::Simulator
}

#[cfg(target_os = "none")]
unsafe extern "C" {
    fn extapp_fileWrite(filename: *const u8, content: *const u8, len: usize) -> bool;
    fn extapp_fileExists(filename: *const u8) -> bool;
    fn extapp_fileRead(filename: *const u8, len: *mut usize) -> *const u8;
    fn extapp_fileErase(filename: *const u8) -> bool;
    fn extapp_fileListWithExtension(
        filename: *mut *mut u8,
        maxrecord: isize,
        extension: *const u8,
    ) -> isize;
    fn extapp_calculatorModel() -> u8;
}
