#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    gpio::{Level, Output},
    time::{Duration, Instant, Timer},
};

const ROW_COUNT: usize = 5;
const COL_COUNT: usize = 5;

const INTER_PATTERN_PAUSE: Duration = Duration::from_millis(200);
const PATTERNS: &[[[u8; COL_COUNT]; ROW_COUNT]] = &[
    [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0],
    ],
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
];

#[ariel_os::task(autostart, peripherals)]
async fn led_matrix(peripherals: pins::LedPeripherals) {
    let mut led_cols = [
        Output::new(peripherals.led_col1, Level::Low),
        Output::new(peripherals.led_col2, Level::Low),
        Output::new(peripherals.led_col3, Level::Low),
        Output::new(peripherals.led_col4, Level::Low),
        Output::new(peripherals.led_col5, Level::Low),
    ];

    let mut led_rows = [
        Output::new(peripherals.led_row1, Level::Low),
        Output::new(peripherals.led_row2, Level::Low),
        Output::new(peripherals.led_row3, Level::Low),
        Output::new(peripherals.led_row4, Level::Low),
        Output::new(peripherals.led_row5, Level::Low),
    ];

    loop {
        for pattern in PATTERNS {
            let now = Instant::now();

            loop {
                for (row, led_row) in pattern.iter().zip(led_rows.iter_mut()) {
                    if row.contains(&1) {
                        // Source current.
                        led_row.set_high();
                    } else {
                        led_row.set_low();
                    }

                    for (col, led_col) in row.iter().zip(led_cols.iter_mut()) {
                        if *col == 1 {
                            // Sink current.
                            led_col.set_low();
                        } else {
                            led_col.set_high();
                        }
                    }

                    Timer::after_millis(1).await;

                    // Stop sourcing current.
                    led_row.set_low();
                }

                if now.elapsed() > INTER_PATTERN_PAUSE {
                    break;
                }
            }
        }
    }
}
