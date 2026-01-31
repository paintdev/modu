use std::ffi::{c_char, c_double, c_int};
use paste::paste;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FFIType {
    Null,
    String,
    Integer,
    Float,
    Boolean,
}

#[repr(C)]
pub union FFIValueUnion {
    pub string: *mut c_char,
    pub integer: c_int,
    pub float: c_double,
    pub boolean: bool,
}

#[repr(C)]
pub struct FFIValue {
    pub ty: FFIType,
    pub value: FFIValueUnion,
}

impl FFIValue {
    pub fn null() -> Self {
        FFIValue {
            ty: FFIType::Null,
            value: unsafe { std::mem::zeroed() },
        }
    }

    pub fn string(ptr: *mut c_char) -> Self {
        FFIValue {
            ty: FFIType::String,
            value: FFIValueUnion { string: ptr },
        }
    }

    pub fn integer(val: i64) -> Self {
        FFIValue {
            ty: FFIType::Integer,
            value: FFIValueUnion { integer: val as c_int },
        }
    }

    pub fn float(val: f64) -> Self {
        FFIValue {
            ty: FFIType::Float,
            value: FFIValueUnion { float: val },
        }
    }

    pub fn boolean(val: bool) -> Self {
        FFIValue {
            ty: FFIType::Boolean,
            value: FFIValueUnion { boolean: val },
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(std::ffi::CString::from_raw(ptr));
        }
    }
}

#[macro_export]
macro_rules! ffi_export {
    ($func_name:ident ( $($arg_name:ident : $arg_type:ty),* ) -> $ret_type:ty $body:block) => {
        pub fn $fn_name( $( $arg_name : $arg_type ),* ) -> $ret $body

        paste! {
            #[no_mangle]
            pub unsafe extern "C" fn [<$fn_name _ffi>](argc: i32, argv: *const FFIValue) -> FFIValue {
                let args = std::slice::from_raw_parts(argv, argc as usize);
                let mut i = 0;
                
                #(
                    let $arg_name: $arg_type = match args[i].ty {
                        FFIType::String => {
                            let c_str = unsafe { std::ffi::CStr::from_ptr(args[i].value.string) };
                            let str_slice = c_str.to_str().unwrap();
                            str_slice.to_string()
                        },
                        FFIType::Integer => unsafe { args[i].value.integer },
                        FFIType::Float => unsafe { args[i].value.float },
                        FFIType::Boolean => unsafe { args[i].value.boolean },
                        FFIType::Null => panic!("Unexpected null argument"),
                    };

                    i += 1;
                )*

                let result = $func_name( $( $arg_name ),* );

                match result {
                    FFIValue::String(s) => FFIValue::string(s),
                    FFIValue::Integer(i) => FFIValue::integer(i),
                    FFIValue::Float(f) => FFIValue::float(f),
                    FFIValue::Boolean(b) => FFIValue::boolean(b),
                    FFIValue::Null => FFIValue::null(),
                }
            }
        }
    };
}