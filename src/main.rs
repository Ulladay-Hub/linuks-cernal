use std::{thread, time};
use rand::Rng;

fn main() {
    println!("Welcome to the Linux installer!");

    let steps = vec![
        "Ordering partitions...",
        "Formatting partitions...",
        "Creating filesystem...",
        "Mounting partitions...",
        "Setting up swap space...",
        "Installing base system...",
        "Configuring bootloader...",
        "Installing kernel modules...",
        "Setting up init system...",
        "Installing core utilities...",
        "Configuring network interfaces...",
        "Installing additional drivers...",
        "Setting up package manager...",
        "Installing system libraries...",
        "Configuring user accounts...",
        "Setting up system locale...",
        "Configuring time zone...",
        "Installing desktop environment...",
        "Installing applications...",
        "Cleaning up installation files...",
        "Finalizing installation...",
    ];

    let mut rng = rand::thread_rng();

    for step in &steps {
        println!("{}", step);
        let duration = time::Duration::from_secs(rng.gen_range(20..=30));
        thread::sleep(duration);
    }

    // Simulate installation progress
    let total_steps = 20;
    for i in 0..total_steps {
        let progress_bar: String = format!("[{:=>width$}]",
                                            "", width = i + 1);
        println!("{}", progress_bar);
        thread::sleep(time::Duration::from_millis(200));
    }

    // Simulate completion
    println!("Installation complete!");
    println!("Your computer will restart in 5 seconds...");

    // Wait for 5 seconds
    thread::sleep(time::Duration::from_secs(5));

    // Open the web browser to play "Never Gonna Give You Up"
    if open::that("https://www.youtube.com/watch?v=dQw4w9WgXcQ").is_err() {
        eprintln!("A step failed, please try again.");
    }
}
