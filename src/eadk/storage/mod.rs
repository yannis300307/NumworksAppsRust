#[cfg(target_os = "none")]
use core::ffi::CStr;

#[cfg(target_os = "none")]
use alloc::{ffi, string::String, vec::Vec};

#[cfg(not(target_os = "none"))]
use std::fs;

#[cfg(target_os = "none")]
pub fn file_write(filename: &str, content: &[u8]) -> Option<()> {
    let c_string = ffi::CString::new(filename).unwrap();
    let result = unsafe { extapp_fileWrite(c_string.as_ptr(), content.as_ptr(), content.len()) };
    if result { Some(()) } else { None }
}

#[cfg(not(target_os = "none"))]
pub fn file_write(filename: &str, content: &[u8]) -> Option<()> {
    if !fs::exists("simulator/storage").ok()? {
        fs::create_dir_all("simulator/storage").ok()?;
    }
    fs::write(format!("simulator/storage/{}", filename), content).ok()
}

#[cfg(target_os = "none")]
pub fn file_exists(filename: &str) -> bool {
    let c_string = ffi::CString::new(filename).unwrap();
    unsafe { extapp_fileExists(c_string.as_ptr()) }
}

#[cfg(not(target_os = "none"))]
pub fn file_exists(filename: &str) -> bool {
    fs::exists(format!("simulator/storage/{}", filename)).unwrap()
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
pub fn file_read(filename: &str) -> Option<Vec<u8>> {
    fs::read(format!("simulator/storage/{}", filename)).ok()
}

#[cfg(target_os = "none")]
pub fn read_file_slice(filename: &str, start: usize, mut slice_lenght: usize) -> Option<Vec<u8>> {
    let c_string = ffi::CString::new(filename).unwrap();
    let mut lenght: usize = 0;
    let array_pointer = unsafe {
        extapp_fileRead(
            c_string.as_ptr().offset(slice_lenght as isize),
            &mut lenght as *mut usize,
        )
    };

    if array_pointer.is_null() {
        return None;
    }

    if lenght < slice_lenght {
        slice_lenght = lenght;
    }

    Some(unsafe { core::slice::from_raw_parts(array_pointer, slice_lenght).to_vec() })
}

#[cfg(not(target_os = "none"))]
pub fn read_file_slice(filename: &str, start: usize, slice_lenght: usize) -> Option<Vec<u8>> {
    fs::read(format!("simulator/storage/{}", filename))
        .map(|v| v[start..(start + slice_lenght)].to_vec())
        .ok()
}

#[cfg(target_os = "none")]
pub fn file_erase(filename: &str) -> Option<()> {
    let c_string = ffi::CString::new(filename).unwrap();
    let result = unsafe { extapp_fileErase(c_string.as_ptr()) };
    if result { Some(()) } else { None }
}

#[cfg(not(target_os = "none"))]
pub fn extapp_file_erase(filename: &str) -> Option<()> {
    fs::remove_file(format!("simulator/storage/{}", filename)).ok()
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
pub fn file_list_with_extension(max_records: usize, extension: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for entry in fs::read_dir("simulator/storage").unwrap() {
        if files.len() > max_records {
            let entry = entry.unwrap();
            let name = entry.file_name().into_string().unwrap();
            if name.ends_with(extension) {
                files.push(name);
            }
        }
    }

    files
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
