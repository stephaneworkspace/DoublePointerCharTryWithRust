use cpp::cpp;

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
        cpp!([double_ptr as "const char **"] -> u32 as "int32_t" {
            for (int i=0; i<2; i++) {
                std::cout << "Hello Double Pointer " << double_ptr[i] << std::endl;
            }
        });
    }
}
