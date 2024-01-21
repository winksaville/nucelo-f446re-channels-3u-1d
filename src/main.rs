#![no_std]
#![no_main]

use nucleo_f446re as _;
use panic_halt as _;
use core::fmt::Write;
use rtt_target::{rtt_init, ChannelMode::BlockIfFull};

fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: {
                size: 512,
                mode: BlockIfFull,
                name: "Up zero"
            }
            1: {
                size: 128,
                name: "Up one",
            }
            2: {
                size: 128,
                name: "Up two",
            }
        }
        down: {
            0: {
                size: 512,
                mode: BlockIfFull,
                name: "Down zero"
            }
        }
    };

    let mut output2 = channels.up.1;
    writeln!(
        output2,
        "Hi! I will turn anything you type on channel 0 into upper case."
    )
    .ok();

    let mut output = channels.up.0;
    let mut log = channels.up.2;
    let mut input = channels.down.0;
    let mut buf = [0u8; 512];
    let mut count: u8 = 0;

    loop {
        let bytes = input.read(&mut buf[..]);
        if bytes > 0 {
            for c in buf.iter_mut() {
                c.make_ascii_uppercase();
            }

            let mut p = 0;
            while p < bytes {
                p += output.write(&buf[p..bytes]);
            }
        }

        writeln!(log, "Messsge no. {}/{}", count, bytes).ok();

        count += 1;

        delay(150_000); // dev build, about 1 second delay
        //delay(2_500_000); // release build, about 1 second delay
    }
}
