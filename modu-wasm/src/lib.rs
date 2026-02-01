use wasm_bindgen::prelude::*;
use std::sync::Mutex;

static STDOUT: Mutex<String> = Mutex::new(String::new());
static STDERR: Mutex<String> = Mutex::new(String::new());

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// for smaller binary size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn eval_modu(code: &str) -> String {
    let mut context = modu::utils::create_context();
    
    modu::parser::parse(code, "<browser>", &mut context);

    let string = format!("{}{}", STDOUT.lock().unwrap().as_str(), STDERR.lock().unwrap().as_str());

    STDOUT.lock().unwrap().clear();
    STDERR.lock().unwrap().clear();

    string
}

#[wasm_bindgen]
pub fn modu_version() -> String {
    modu::VERSION.to_string()
}

#[unsafe(no_mangle)]
pub extern "C" fn _modu_print(ptr: *const u8, len: usize) {
    let text = unsafe {
        std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap()
    };

    let mut output = STDOUT.lock().unwrap();
    output.push_str(text);
}

#[unsafe(no_mangle)]
pub extern "C" fn _modu_eprint(ptr: *const u8, len: usize) {
    let text = unsafe {
        std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap()
    };

    let mut stderr = STDERR.lock().unwrap();
    stderr.push_str(text);
}
