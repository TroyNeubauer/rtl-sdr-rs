use seify_rtlsdr::{error::Result, RtlSdr};
use std::sync::atomic::{AtomicBool, Ordering};

const DEFAULT_BUF_LENGTH: usize = 16 * 16384;

const DEVICE_INDEX: usize = 0;
const SAMPLE_RATE: u32 = 2_048_000;

fn main() -> Result<()> {
    let devices = seify_rtlsdr::enumerate()?;
    println!("devices: {devices:?}");

    // Create shutdown flag and set it when ctrl-c signal caught
    static SHUTDOWN: AtomicBool = AtomicBool::new(false);
    ctrlc::set_handler(|| {
        SHUTDOWN.swap(true, Ordering::Relaxed);
    })
    .expect("Failed to set shutdown handler");

    // Open device
    let mut sdr = RtlSdr::open(DEVICE_INDEX).expect("Unable to open SDR device!");
    // println!("{:#?}", sdr);

    let gains = sdr.get_tuner_gains()?;
    println!(
        "Supported gain values ({}): {:?}",
        gains.len(),
        gains
            .iter()
            .map(|g| { *g as f32 / 10.0 })
            .collect::<Vec<_>>()
    );

    // Set sample rate
    sdr.set_sample_rate(SAMPLE_RATE)?;
    println!("Sampling at {} S/s", sdr.get_sample_rate());

    // Enable test mode
    println!("Enable test mode");
    sdr.set_testmode(true)?;

    // Reset the endpoint before we try to read from it (mandatory)
    println!("Reset buffer");
    sdr.reset_buffer()?;

    println!("Reading samples in sync mode...");
    let mut buf: [u8; DEFAULT_BUF_LENGTH] = [0; DEFAULT_BUF_LENGTH];
    loop {
        if SHUTDOWN.load(Ordering::Relaxed) {
            break;
        }
        let n = sdr.read_sync(&mut buf);
        if n.is_err() {
            println!("Read error: {n:#?}");
        } else {
            let n = n.unwrap();
            if n < DEFAULT_BUF_LENGTH {
                println!("Short read ({n:#?}), samples lost, exiting!");
                break;
            }
        }
        // println!("read {} samples!", n.unwrap());
    }

    println!("Close");
    sdr.close()?;
    Ok(())
}
