extern crate i2cdev;
extern crate byteorder;

pub use i2cdev::linux::{LinuxI2CDevice,LinuxI2CError};
pub use i2cdev::core::I2CDevice;
use byteorder::{BigEndian,ByteOrder};

pub const DEFAULT_ADS1115_SLAVE_ADDRESS: u16 = 0x48;

const ADS111X_CONVERSION_REGISTER: u8 = 0b00;
const ADS111X_CONFIG_REGISTER: u8 = 0b01;

#[derive(Debug,Copy,Clone)]
/// Only functions on ADS1115
pub enum ADS1115MultiplexerConfig {
	AIN0_AIN1 = 0b000,
	AIN0_AIN3 = 0b001,
	AIN1_AIN3 = 0b010,
	AIN2_AIN3 = 0b011,
	AIN0_GND = 0b100,
	AIN1_GND = 0b101,
	AIN2_GND = 0b110,
	AIN3_GND = 0b111,
}

#[derive(Debug,Copy,Clone)]
/// Only functions on ADS1114 and ADS1115
pub enum ADS11145GainAmplifier {
	FS_6_144V = 0b000,
	FS_4_096V = 0b001,
	FS_2_048V = 0b010,
	FS_1_024V = 0b011,
	FS_0_512V = 0b100,
	FS_0_256V = 0b101,
}

#[derive(Debug,Copy,Clone)]
pub enum ADS111XOperatingMode {
	ContinuousConversion = 0b0,
	SingleShot = 0b1
}

#[derive(Debug,Copy,Clone)]
pub enum ADS111XDataRate {
	DR_8SPS = 0b000,
	DR_16SPS = 0b001,
	DR_32SPS = 0b010,
	DR_64SPS = 0b011,
	DR_128SPS = 0b100,
	DR_250SPS = 0b101,
	DR_475SPS = 0b110,
	DR_860SPS = 0b111,
}

#[derive(Debug,Copy,Clone)]
pub struct ADS111XConfig {
	pub multiplexer_config: ADS1115MultiplexerConfig,
	pub gain_amplifier: ADS11145GainAmplifier,
	pub operating_mode: ADS111XOperatingMode,
	pub data_rate: ADS111XDataRate
}

pub struct ADS111X {
	device: LinuxI2CDevice,
	config: u16,
	continuous: bool,
	gain: f32
}

impl ADS111X {
	pub fn new(device: LinuxI2CDevice, config: ADS111XConfig) -> Result<ADS111X, LinuxI2CError> {
		let mut config_reg: u16 = 0b0;
		config_reg |= (config.multiplexer_config as u16) << 12;
		config_reg |= (config.gain_amplifier as u16) << 9;
		config_reg |= (config.operating_mode as u16) << 8;
		config_reg |= (config.data_rate as u16) << 5;
		

		let continuous = match config.operating_mode {
			ADS111XOperatingMode::ContinuousConversion => true,
			ADS111XOperatingMode::SingleShot => false
		};

		let gain = match config.gain_amplifier {
			ADS11145GainAmplifier::FS_6_144V => 6.144,
			ADS11145GainAmplifier::FS_4_096V => 4.096,
			ADS11145GainAmplifier::FS_2_048V => 2.048,
			ADS11145GainAmplifier::FS_1_024V => 1.024,
			ADS11145GainAmplifier::FS_0_512V => 0.512,
			ADS11145GainAmplifier::FS_0_256V => 0.256
		} / 32767.0;

		let mut ads111x = ADS111X{ device: device, config: config_reg, continuous: continuous, gain: gain };
		try!(ads111x.input_config());
		Ok(ads111x)
	}

	fn input_config(&mut self) -> Result<(), LinuxI2CError> {
		try!(self.device.smbus_write_word_data(ADS111X_CONFIG_REGISTER, self.config));
		Ok(())
	}

	fn read_conversion_register(&mut self) -> Result<i16, LinuxI2CError> {
		let mut buf = [0_u8;2];
		try!(self.device.write(&[ADS111X_CONVERSION_REGISTER]));
		try!(self.device.read(&mut buf));
		let raw = BigEndian::read_i16(&buf);	
		Ok(raw as i16)
	}

	pub fn read_voltage(&mut self) -> Result<f32, LinuxI2CError> {
		if !self.continuous {
			try!(self.input_config());
		}
		let raw = try!(self.read_conversion_register());
		Ok((raw as f32) * self.gain)
	}
}
