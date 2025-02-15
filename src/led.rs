use smart_leds::{SmartLedsWrite, RGB8};
use std::time::{Duration, Instant};
use std::{thread, time};
use ws281x_rpi::Ws2812Rpi;

const PIN: i32 = 18; // SPI GPIO 10 Pin, PCM_DOUT GPIO 21 Pin
const NUM_LEDS: usize = 60 * 5; // 60 LEDs per meter, 5 meters
const DELAY: time::Duration = time::Duration::from_millis(1000);

pub fn blink() {
    println!("blink start...");
    let mut ws = Ws2812Rpi::new(NUM_LEDS as i32, PIN).unwrap();

    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];

    // Blink the LED's in a blue-green-red-white pattern.
    for led in data.iter_mut().step_by(4) {
        led.b = 255;
    }

    if NUM_LEDS > 1 {
        for led in data.iter_mut().skip(1).step_by(4) {
            led.g = 255;
        }
    }

    if NUM_LEDS > 2 {
        for led in data.iter_mut().skip(2).step_by(4) {
            led.r = 255;
        }
    }

    if NUM_LEDS > 3 {
        for led in data.iter_mut().skip(3).step_by(4) {
            led.r = 32;
            led.g = 32;
            led.b = 32;
        }
    }

    loop {
        // On
        println!("blink(): LEDS on");
        ws.write(data.iter().cloned()).unwrap();
        thread::sleep(DELAY);

        // Off
        println!("blink(): LEDS off");
        ws.write(empty.iter().cloned()).unwrap();
        thread::sleep(DELAY);
    }
}

pub fn timed_blink(duration_ms: u64) {
    println!("timed_blink start...");

    let mut ws = Ws2812Rpi::new(NUM_LEDS as i32, PIN).unwrap();

    let mut red_data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    for led in red_data.iter_mut() {
        led.r = 255;
    }
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];

    let start_time = Instant::now();
    let duration = Duration::from_millis(duration_ms);

    let mut delay_ms: u64 = 500; // Initial delay
    let min_delay_ms: u64 = 50; // Minimum delay

    while start_time.elapsed() < duration {
        let remaining_ms = duration_ms - start_time.elapsed().as_millis() as u64;

        // Decrease delay gradually
        delay_ms = delay_ms
            .saturating_sub((delay_ms as f64 * 0.025) as u64)
            .max(min_delay_ms); //Smooth decrease, avoid underflow

        // Last 2 seconds solid red
        if remaining_ms <= 2000 {
            println!("timed_blink(): LEDS solid red");
            ws.write(red_data.iter().cloned()).unwrap();
            thread::sleep(Duration::from_millis(remaining_ms)); //Sleep for the remaining time
            break; //Exit the loop since the remaining time is handled.
        }

        // On
        println!("timed_blink(): LEDS on, delay: {}", delay_ms);
        ws.write(red_data.iter().cloned()).unwrap();
        thread::sleep(Duration::from_millis(delay_ms));

        // Off
        println!("timed_blink(): LEDS off, delay: {}", delay_ms);
        ws.write(empty.iter().cloned()).unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
    println!("timed_blink finished.");
}
