#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use microbit::board::Board;
// use microbit::hal::prelude::*;
// use microbit::hal::timer::Timer;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // let light_it_all = [
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    //     [1, 1, 1, 1, 1],
    // ];

    let all_lights_off = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    // infinite loop; just so we don't leave this stack frame
    loop {
        // top row
        for i in 0..5 {
            let mut grid_copy = all_lights_off.clone();
            grid_copy[0][i] = 1;
            light_it_up(&mut display, &mut timer, grid_copy);
        }
        // right column except first light
        for i in 1..5 {
            let mut grid_copy = all_lights_off.clone();
            grid_copy[i][4] = 1;
            light_it_up(&mut display, &mut timer, grid_copy);
        }
        // bottom row except last light
        for i in 0..4 {
            let mut grid_copy = all_lights_off.clone();
            let n = 3 - i;
            grid_copy[4][n] = 1;
            light_it_up(&mut display, &mut timer, grid_copy);
        }
        // left column except last and first light
        for i in 0..3 {
            let mut grid_copy = all_lights_off.clone();
            let n = 3 - i;
            grid_copy[n][0] = 1;
            light_it_up(&mut display, &mut timer, grid_copy);
        }
    }
}

fn light_it_up(
    display: &mut Display,
    timer: &mut Timer<microbit::hal::pac::TIMER0>,
    led_display: [[u8; 5]; 5],
) {
    display.show(timer, led_display, 20);
    rprintln!("Light!");
    display.clear();
    rprintln!("Dark!");
    timer.delay_ms(10_u16);
}

// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     let mut board = Board::take().unwrap();

//     let mut timer = Timer::new(board.TIMER0);

//     board.display_pins.col2.set_low().unwrap();
//     let mut row2 = board.display_pins.row2;

//     // infinite loop; just so we don't leave this stack frame
//     loop {
//         row2.set_low().unwrap();
//         rprintln!("Dark!");
//         timer.delay_ms(1_000_u16);
//         row2.set_high().unwrap();
//         rprintln!("Light!");
//         timer.delay_ms(1_000_u16);
//     }
// }
