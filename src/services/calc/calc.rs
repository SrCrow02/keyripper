use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive, FromPrimitive};
use crate::utils::utils::HardwareInfo;

pub fn calc(hardware_info: &HardwareInfo, num_machines: Option<u64>) {
    // Total number of bits in the keyspace
    let total_bits: u32 = 130;

    // Number of bits in the block
    let bits: u32 = 60; 

    // Estimate the number of hashes per second based on the provided hardware information
    let hashes_per_second = estimate_hashes_per_second(hardware_info);

    // Calculate the total number of possible keys, which is 2^bits
    let total_keys: BigUint = BigUint::one() << bits;

    println!("\x1b[38;2;0;200;255mTotal number of possible keys:\x1b[0m \x1b[32m{}\x1b[0m", format_number(&total_keys));
    println!("\x1b[38;2;0;200;255mEstimated hashes per second for this hardware:\x1b[0m \x1b[32m{}\x1b[0m", format_number(&BigUint::from(hashes_per_second)));

    // Calculate the total number of hashes per second considering the number of machines 
    let total_hashes_per_second = match num_machines {
        Some(machines) => {
            // Convert hashes per second and number of machines to BigUint
            let hashes_per_second_biguint = BigUint::from(hashes_per_second);
            hashes_per_second_biguint * BigUint::from(machines)
        },
        None => BigUint::from(hashes_per_second),
    };

    // Calculate the number of seconds required to exhaust the keyspace
    let seconds = if total_hashes_per_second.is_zero() {
        BigUint::zero()
    } else {
        total_keys / total_hashes_per_second
    };

    let seconds_per_minute: BigUint = BigUint::from(60u64);
    let seconds_per_hour: BigUint = &seconds_per_minute * 60u64;
    let seconds_per_day: BigUint = &seconds_per_hour * 24u64;
    let seconds_per_year: BigUint = &seconds_per_day * 365u64;

    // Calculate years, days, hours, minutes, and seconds from the total seconds
    let years = &seconds / &seconds_per_year;
    let remaining = &seconds % &seconds_per_year;
    let days = &remaining / &seconds_per_day;
    let remaining = &remaining % &seconds_per_day;
    let hours = &remaining / &seconds_per_hour;
    let remaining = &remaining % &seconds_per_hour;
    let minutes = &remaining / &seconds_per_minute;
    let remaining_seconds = &remaining % &seconds_per_minute;

    println!("\x1b[38;2;0;200;255mEstimated time to exhaust the keyspace:\x1b[0m");

    if years.is_zero() && days.is_zero() && hours.is_zero() && minutes.is_zero() && remaining_seconds.is_zero() {
        println!("\x1b[32mLess than 1 second\x1b[0m");
    } else if years.is_zero() {
        println!(
            "\x1b[32m{} days, {} hours, {} minutes, {} seconds\x1b[0m", 
            format_number(&days),
            format_number(&hours),
            format_number(&minutes),
            format_number(&remaining_seconds)
        );
    } else {
        println!(
            "\x1b[32m{} years, {} days, {} hours, {} minutes, {} seconds\x1b[0m", 
            format_number(&years),
            format_number(&days),
            format_number(&hours),
            format_number(&minutes),
            format_number(&remaining_seconds)
        );
    }

    // Convert the number of years to f64 and print a scientific notation estimate
    let years_f64 = years.to_f64().unwrap_or(f64::INFINITY);

    if years_f64 != 0.00e0 {
        println!("\x1b[32mApproximately {:.2e} years\x1b[0m", years_f64);
    }

    // Calculate the number of blocks needed to cover the keyspace
    let total_blocks = BigUint::one() << (total_bits - bits);
    // Print the number of blocks needed
    println!(
        "\x1b[38;2;0;200;255mNumber of {}-bit blocks required to cover the {}-bit keyspace:\x1b[0m \x1b[32m{}\x1b[0m",
        bits,
        total_bits,
        format_number(&total_blocks)
    );
}

// Function to format BigUint numbers with thousands separators for better readability
fn format_number(num: &BigUint) -> String {
    let mut num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    // Insert commas every 3 digits from the end
    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}

// Function to estimate the number of hashes per second based on hardware information
fn estimate_hashes_per_second(hardware_info: &HardwareInfo) -> u64 {
    // Base number of hashes per core
    let base_hashes_per_core = BigUint::parse_bytes(b"5_000_000_000_000", 10).unwrap(); 

    // Number of logical cores and CPU speed in GHz
    let logical_cores = hardware_info.logical_cores.to_f64().unwrap_or(1.0);
    let cpu_speed_ghz = hardware_info.cpu_speed_ghz;

    // Adjust the base hashes per core based on CPU speed
    let base_hashes_per_core_f64 = base_hashes_per_core.to_f64().unwrap_or(1.0);
    let adjusted_hashes_f64 = base_hashes_per_core_f64 * cpu_speed_ghz;

    // Convert the adjusted value to BigUint
    let adjusted_hashes = BigUint::from_f64(adjusted_hashes_f64).unwrap_or(BigUint::zero());

    // Convert the number of logical cores to BigUint and calculate the total hashes per second
    let logical_cores_biguint = BigUint::from(logical_cores as u64);
    (adjusted_hashes * logical_cores_biguint).to_u64().unwrap_or(0)
}
