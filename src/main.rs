extern crate ads111x;

use ads111x::*;
use std::thread::sleep_ms;

fn main() {
	let config = ADS111XConfig {
		multiplexer_config: ADS1115MultiplexerConfig::AIN0_GND,
		gain_amplifier: ADS11145GainAmplifier::FS_6_144V,
		operating_mode: ADS111XOperatingMode::SingleShot,
		data_rate: ADS111XDataRate::DR_128SPS
	};

	let device = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_ADS1115_SLAVE_ADDRESS).unwrap();
	let mut ads111x = ADS111X::new(device, config).unwrap();

	loop {
		println!("voltage: {}", ads111x.read_voltage().unwrap());
		sleep_ms(100);
	}
}