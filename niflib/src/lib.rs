use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("lib.h");

        fn rust_GetNifTextureFilepaths(nifFilePath: &CxxString) -> UniquePtr<CxxVector<CxxString>>;
        fn rust_test();
    }
}

pub fn get_nif_texture_filepaths(nif_file_path: &str) -> Vec<String> {
    let mut texture_filepaths = Vec::new();

    let_cxx_string!(nif_file_path_cxx = nif_file_path);

    // ffi::rust_GetNifTextureFilepaths(&nif_file_path_cxx).as_ref().map(|vec| {
    //     for i in 0..vec.len() {
    //         if let Some(cxx_string) = vec.get(i) {
    //             texture_filepaths.push(cxx_string.to_string_lossy().into_owned());
    //         }
    //     }
    // });

    ffi::rust_test();


    texture_filepaths
}
