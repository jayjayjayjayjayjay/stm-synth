use crate::prelude::*;

use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Output;
use embassy_stm32::{
    i2c::{Error as I2CError, I2c, Master},
    mode::Async,
};
use embassy_time::Timer;


use crate::types::errors::GeneralError;

const CS43L22_I2C_ADR: u8 = 0x4A;

const POWER_DOWN_ADR: u8 = 0x2;
const HEADPHONE_POWER_ADR: u8 = 0x4;
const CLOCKING_CONTROL_ADR: u8 = 0x5;
const INTERFACE_CONTROL_1: u8 = 0x6;
const INTERFACE_CONTROL_2: u8 = 0x7;
const BEEP_FREQ_ADR: u8 = 0x1c;
const BEEP_VOLUME_ADR: u8 = 0x1d;
const BEEP_TONE_CONFIG_ADR: u8 = 0x1e;
const STATUS_ADR: u8 = 0x2E;
const PCMA_VOLUME: u8 = 0x1A;
const PCMB_VOLUME: u8 = 0x1B;

// write to 06h
enum CS43L22Error {
    I2CError(I2CError),
    GeneralError(GeneralError),
}
impl From<I2CError> for CS43L22Error {
    fn from(err: I2CError) -> Self {
        CS43L22Error::I2CError(err)
    }
}

trait CustomActions {
    async fn write_simple(&mut self, register_adr: u8, data: u8) -> Result<(), CS43L22Error>;

    async fn write_bit_simple(
        &mut self,
        register_adr: u8,
        bit: bool,
        bit_position: u8,
    ) -> Result<(), CS43L22Error>;

    async fn read_simple(&mut self, register_adr: u8) -> Result<u8, CS43L22Error>;
}

impl CustomActions for I2c<'_, Async, Master> {
    async fn write_simple(&mut self, register_adr: u8, data: u8) -> Result<(), CS43L22Error> {
        self.write(CS43L22_I2C_ADR, &[register_adr, data]).await?;
        Ok(())
    }

    async fn write_bit_simple(
        &mut self,
        register_adr: u8,
        bit: bool,
        bit_position: u8,
    ) -> Result<(), CS43L22Error> {
        if bit_position > 8 {
            return Err(CS43L22Error::GeneralError(GeneralError::InvalidParameter));
        }

        let mut read_data = [0];

        self.write_read(CS43L22_I2C_ADR, &[register_adr], &mut read_data)
            .await?;

        if bit {
            read_data[0] |= 1 << bit_position;
        } else {
            read_data[0] &= !(1 << bit_position);
        }

        self.write(CS43L22_I2C_ADR, &[register_adr, read_data[0]])
            .await?;
        Ok(())
    }

    async fn read_simple(&mut self, register_adr: u8) -> Result<u8, CS43L22Error> {
        let mut read_data = [0];

        self.write_read(CS43L22_I2C_ADR, &[register_adr], &mut read_data)
            .await?;
        Ok(read_data[0])
    }
}

pub async fn initalise(
    i2c: &mut I2c<'static, Async, Master>,
    reset_pin: &mut Output<'static>,
) -> Result<(), CS43L22Error> {
    Timer::after_secs(1).await;
    reset_pin.set_high();
    Timer::after_secs(1).await;
    info!("Init dac");

    i2c.write_simple(0x00, 0x99).await?;

    i2c.write_simple(0x47, 0x80).await?;

    let mut write_data = [0x32];
    let mut read_data = [0x0];
    i2c.write_read(CS43L22_I2C_ADR, &mut write_data, &mut read_data)
        .await?;

    read_data[0] |= 1 << 7;
    let mut data = [0x32, read_data[0]];
    i2c.write(CS43L22_I2C_ADR, &mut data).await?;

    read_data[0] &= !(1 << 7);
    let mut data = [0x32, read_data[0]];
    i2c.write(CS43L22_I2C_ADR, &mut data).await?;
    // let mut write_data = [0x1];
    //let mut read_data = [0x0];
    //i2c.write_read(0x4A, &mut write_data, &mut read_data).await;
    let mut data = [0x0, 0x0];
    i2c.write(CS43L22_I2C_ADR, &mut data).await?;
    Ok(())
}

pub async fn tone_configure(i2c: &mut I2c<'static, Async, Master>) -> Result<(), CS43L22Error> {
    i2c.write_simple(BEEP_TONE_CONFIG_ADR, (0b00 << 6) as u8)
        .await?;
    if let Ok(res) = i2c.read_simple(BEEP_TONE_CONFIG_ADR).await {
        info!("TONE:{}", res);
    }
    // i2c.write_simple(BEEP_VOLUME_ADR, 0b110).await?;
    i2c.write_simple(BEEP_VOLUME_ADR, 0b10100).await?;
    i2c.write_simple(BEEP_FREQ_ADR, 0b0000).await?;
    info!("beep freq:{}", i2c.read_simple(BEEP_FREQ_ADR).await?);
    Ok(())
}

pub async fn tone_play(i2c: &mut I2c<'static, Async, Master>) -> Result<(), CS43L22Error> {
    i2c.write_simple(BEEP_TONE_CONFIG_ADR, (0b11 << 6) as u8)
        .await
}

pub async fn tone_stop(i2c: &mut I2c<'static, Async, Master>) -> Result<(), CS43L22Error> {
    i2c.write_simple(BEEP_TONE_CONFIG_ADR, (0b00 << 6) as u8)
        .await
}

pub async fn power_toggle(
    i2c: &mut I2c<'static, Async, Master>,
    power: bool,
) -> Result<(), CS43L22Error> {
    const POWER_UP: u8 = 0b10011110;
    const POWER_DOWN: u8 = 0b1;
    if power {
        i2c.write_simple(POWER_DOWN_ADR, POWER_UP).await;
    } else {
        i2c.write_simple(POWER_DOWN_ADR, POWER_DOWN).await;
    }
    info!("power tog status{}", i2c.read_simple(POWER_DOWN_ADR).await?);
    Ok(())
}

pub async fn headphone_speaker_toggle(
    i2c: &mut I2c<'static, Async, Master>,
    _power: bool,
) -> Result<(), CS43L22Error> {
    //i2c.write_simple(HEADPHONE_POWER_ADR, 0xAF).await?;
    info!("HEADPHONE:{}", i2c.read_simple(HEADPHONE_POWER_ADR).await?);
    Ok(())
}

pub async fn set_dac_interface_format(
    i2c: &mut I2c<'static, Async, Master>,
) -> Result<(), CS43L22Error> {
    i2c.write_bit_simple(INTERFACE_CONTROL_1, true, 2).await?;
    i2c.write_bit_simple(INTERFACE_CONTROL_1, false, 3).await?;
    i2c.write_bit_simple(INTERFACE_CONTROL_1, true, 1).await?;
    i2c.write_bit_simple(INTERFACE_CONTROL_1, true, 0).await?;
    info!(
        "Set interface format, now {}",
        i2c.read_simple(INTERFACE_CONTROL_1).await?
    );
    Ok(())
}

pub async fn speaker_headphone_toggle(
    i2c: &mut I2c<'static, Async, Master>,
) -> Result<(), CS43L22Error> {
    i2c.write_bit_simple(INTERFACE_CONTROL_2, true, 3).await?;
    info!("Toggled Speaker");
    Ok(())
}

pub async fn log_status(i2c: &mut I2c<'static, Async, Master>) -> Result<(), CS43L22Error> {
    info!("Status:{}", i2c.read_simple(STATUS_ADR).await?);
    Ok(())
}

pub async fn clocking_control(
    i2c: &mut I2c<'static, Async, Master>,
) -> Result<(), CS43L22Error> {
    info!("clock control:{}",i2c.read_simple(CLOCKING_CONTROL_ADR).await?);
/*    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, true, 1).await?;
    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, false, 2).await?;

    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, true, 3).await?;
    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, false, 4).await?;*/

    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, false, 0).await?;
    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, true, 7).await?;
    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, false, 6).await?;
    i2c.write_bit_simple(CLOCKING_CONTROL_ADR, true, 5).await?;
    info!("clock control:{}",i2c.read_simple(CLOCKING_CONTROL_ADR).await?);
    info!("Toggled Speaker");
    Ok(())
}

pub async fn set_pcm_volume(
    i2c: &mut I2c<'static, Async, Master>,
) -> Result<(), CS43L22Error> {
    i2c.write_simple(PCMA_VOLUME,0x18).await?;
    i2c.write_simple(PCMB_VOLUME,0x18).await?;
    Ok(())
}

#[embassy_executor::task]
pub async fn task(
    mut i2c: I2c<'static, Async, Master>,
    mut reset_pin: Output<'static>,
    mut button: ExtiInput<'static, Async>,
) {
    if let Ok(_) = initalise(&mut i2c, &mut reset_pin).await {
        tone_configure(&mut i2c).await;
        headphone_speaker_toggle(&mut i2c, true).await;
        set_dac_interface_format(&mut i2c).await;
        clocking_control(&mut i2c).await;
        set_pcm_volume(&mut i2c).await;
        Timer::after_secs(1).await;
        power_toggle(&mut i2c, true).await;
        info!("Tone should be playing");
        let _button_flag = false;

        loop {
            button.wait_for_rising_edge().await;
            let _ = tone_play(&mut i2c).await;
            button.wait_for_falling_edge().await;
            let _=  tone_stop(&mut i2c).await;
            let _ = log_status(&mut i2c).await;
        }
    } else {
        error!("Error initalising CS43L22");
    }
}
