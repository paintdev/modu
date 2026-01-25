use std::collections::HashMap;
use std::path::PathBuf;
use crate::ast::AST;
use crate::eval::eval;

use modu_ffi::{FFIValue, FFIType};

type FFIFunction = unsafe extern "C" fn(i32, *const FFIValue) -> FFIValue;

pub fn call(mut args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    // (path_to_lib, function_name, arg1, arg2, ...)

    if args.len() < 2 {
        return Err("ffi.call requires at least 2 arguments".to_string());
    }

    let mut path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string() + "/";

    let sys_args = std::env::args().collect::<Vec<String>>();

    if sys_args.len() > 2 {
        if sys_args[1] == "run" {
            let file_path = PathBuf::from(&sys_args[2]);
            let parent = file_path.parent().unwrap();
            let parent_str = parent.to_str().unwrap();
            path = parent_str.to_string() + "/";
        }
    }

    path += match eval(args[0].clone(), context) {
        Ok(AST::String(v)) => v,
        _ => return Err("ffi.call first argument must be a string".to_string()),
    }.as_str();

    let name = match eval(args[1].clone(), context) {
        Ok(AST::String(v)) => v,
        _ => return Err("ffi.call second argument must be a string".to_string()),
    };

    let mut ffi_args = Vec::<FFIValue>::new();
    let mut owned_strings = Vec::<*mut std::ffi::c_char>::new();

    for arg in args.drain(2..) {
        match eval(arg, context)? {
            AST::Null => ffi_args.push(FFIValue::null()),
            AST::String(v) => {
                let c = std::ffi::CString::new(v).unwrap();
                let ptr = c.into_raw();

                owned_strings.push(ptr);
                ffi_args.push(FFIValue::string(ptr));
            }
            AST::Integer(v) => ffi_args.push(FFIValue::integer(v)),
            AST::Float(v) => ffi_args.push(FFIValue::float(v)),
            AST::Boolean(v) => ffi_args.push(FFIValue::boolean(v)),

            _ => return Err("unsupported FFI argument".into()),
        }
    }

    unsafe {
        let lib = match libloading::Library::new(path) {
            Ok(lib) => lib,
            Err(e) => return Err(format!("failed to load library: {}", e)),
        };

        let func: libloading::Symbol<FFIFunction>
            = match lib.get(name.as_bytes()) {
                Ok(func) => func,
                Err(e) => return Err(format!("failed to load function: {}", e)),
            };

        let result = func(ffi_args.len() as i32, ffi_args.as_ptr());

        for ptr in owned_strings {
            modu_ffi::ffi_free_string(ptr);
        }

        match result.ty {
            FFIType::Null => Ok((AST::Null, AST::Null)),
            FFIType::String => {
                let c_str = std::ffi::CStr::from_ptr(result.value.string);
                let str_slice = c_str.to_str().unwrap();
                let string = str_slice.to_string();

                modu_ffi::ffi_free_string(result.value.string);

                Ok((AST::String(string), AST::Null))
            }
            FFIType::Integer => Ok((AST::Integer(result.value.integer), AST::Null)),
            FFIType::Float => Ok((AST::Float(result.value.float), AST::Null)),
            FFIType::Boolean => Ok((AST::Boolean(result.value.boolean), AST::Null)),
        }
    }
}

pub fn get_object() -> HashMap<String, AST> {
	let mut object = HashMap::new();

	object.insert(
        "call".to_string(),
        AST::InternalFunction {
            name: "call".to_string(),
            args: vec!["__args__".to_string()],
            call_fn: call,
        }
    );

	object
}