#[macro_use]
extern crate rutie;
extern crate lazy_static;

mod async_runtime;
mod sleeper;

use async_runtime::AsyncRuntime;
use rutie::{AnyObject, Class, Integer, NilClass, Object, VM};

const THREAD_COUNT: u8 = 2;

wrappable_struct!(AsyncRuntime, AsyncRuntimeWrapper, ASYNC_RUNTIME_WRAPPER);

class!(AsyncRubyRust);

methods!(
    AsyncRubyRust,
    _rtself,

    fn init_async() -> AnyObject {
        let runtime = AsyncRuntime::new(THREAD_COUNT);

        Class::from_existing("AsyncRubyRust")
            .wrap_data(runtime, &*ASYNC_RUNTIME_WRAPPER)
    }

    fn run_callback_loop() -> NilClass {
        let runtime = _rtself.get_data_mut(&*ASYNC_RUNTIME_WRAPPER);
        runtime.run_callback_loop();

        NilClass::new()
    }

    fn sleep(input: Integer) -> NilClass {
        let duration = input.map_err(VM::raise_ex).unwrap().to_u64();
        let runtime = _rtself.get_data(&*ASYNC_RUNTIME_WRAPPER);

        sleeper::sleep(&runtime.runtime, duration);

        NilClass::new()
    }
);

#[no_mangle]
pub extern "C" fn init_ext() {
    Class::from_existing("AsyncRubyRust").define(|klass| {
        klass.def_self("init", init_async);
        klass.def("sleep", sleep);
        klass.def("run_callback_loop", run_callback_loop);
    });
}
