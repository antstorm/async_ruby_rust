#[macro_use]
extern crate rutie;

mod sleeper;

use rutie::{Module, Integer, NilClass, Object, VM};

class!(AsyncRubyRust);

methods!(
    AsyncRubyRust,
    _rtself,
    fn sleep(input: Integer) -> NilClass {
        let duration = input.map_err(VM::raise_ex).unwrap().to_u64();
        sleeper::sleep(duration);

        NilClass::new()
    }
);

#[no_mangle]
pub extern "C" fn init_ext() {
    Module::from_existing("AsyncRubyRust").define(|module| {
        module.def_self("sleep", sleep);
    });

    println!("Extention initialized");
}
