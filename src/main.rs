#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::panic::PanicInfo;

use core::mem::MaybeUninit;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

mod app;
mod behviour;
mod block;
mod p2p;

#[derive(serde::Serialize)]
struct SerializableData {
    data: String,
    id: u64,
    nonce: u64,
    previous_hash: String,
}

#[entry]
fn main() -> ! {
    // initialize the allocator
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }

    hprintln!("[INFO] starting");

    let mut app = app::App::new();
    app.genesis();
    behviour::print_chain(&app);

    let a: serde_json_core::heapless::String<256> =
        serde_json_core::ser::to_string(&SerializableData {
            data: "block".to_string(),
            id: 0,
            nonce: 0,
            previous_hash: "198d6f2d9f7b588848c3e212eb460626189a0b0fbb252eab42d3b5a4f09b7c83"
                .to_string(),
        })
        .unwrap();
    hprintln!("{}", a.to_string());

    loop {
        behviour::create_block(&mut app);
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    hprintln!("[ERROR] out of memory!");
    loop {}
}
