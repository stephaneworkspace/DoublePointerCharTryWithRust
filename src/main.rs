use cpp::cpp;
use std::os::raw::c_char;
use std::ffi::CStr;

cpp!{{
    #include <iostream>
}}

fn main() {
    let name0 = std::ffi::CString::new("Hello").unwrap();
    let name1 = std::ffi::CString::new("World").unwrap();
    let size = 0 + 1;

    let name_ptr0 = name0.as_ptr();
    let name_ptr1 = name1.as_ptr();
    let mut vec: Vec<*const i8> = Vec::new();
    vec.push(name_ptr0);
    vec.push(name_ptr1);
    let double_ptr = vec.as_ptr();
    let r = unsafe {
            cpp!([name_ptr0 as "const char *"] -> u32 as "int32_t" {
            std::cout << "Hello, " << name_ptr0 << std::endl;
            return 42;
        })
    };
    assert_eq!(r, 42);
    unsafe {
        cpp!([double_ptr as "const char **"] {
            for (int i=0; i<2; i++) {
                std::cout << "Hello Double Pointer " << double_ptr[i] << std::endl;
            }
        });
    }
    let mut vec_string: Vec<String> = Vec::new();
    for i in 0..2 {
        let occurs: *const c_char = unsafe {
            cpp!([double_ptr as "const char **", i as "int32_t"] -> *const c_char as "const char *" {
                return double_ptr[i];
            })
        };
        vec_string.push(unsafe { CStr::from_ptr(occurs).to_str().unwrap_or("???").to_string() });
    }
    println!("{:?}", vec_string.as_slice());
}
