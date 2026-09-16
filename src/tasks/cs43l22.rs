use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals::I2C1;
use embassy_stm32::{
    i2c::{Error as I2CError, I2c, Master},
    mode::Async,
};
use embassy_time::Timer;

use defmt::{error, info};

const CS43L22_I2C_ADR: u8 = 0x4A;

const POWER_DOWN_ADR: u8 = 0x2;
const HEADPHONE_POWER_ADR: u8 = 0x4;
const BEEP_FREQ_ADR: u8 = 0x1c;
const BEEP_VOLUME_ADR: u8 = 0x1d;
const BEEP_TONE_CONFIG_ADR: u8 = 0x1e;

trait CustomActions {
    async fn write_simple(&mut self, register_adr: u8, data: u8) -> Result<(), I2CError>;

    async fn read_simple(&mut self, register_adr: u8) -> Result<u8, I2CError>;
}

impl CustomActions for I2c<'_, Async, Master> {
    async fn write_simple(&mut self, register_adr: u8, data: u8) -> Result<(), I2CError> {
        self.write(CS43L22_I2C_ADR, &[register_adr, data]).await
    }

    async fn read_simple(&mut self, register_adr: u8) -> Result<u8, I2CError> {
        let mut read_data = [0];

        self.write_read(CS43L22_I2C_ADR, &[register_adr], &mut read_data)
            .await?;
        Ok(read_data[0])
    }
}

pub async fn initalise(
    i2c: &mut I2c<'static, Async, Master>,
    reset_pin: &mut Output<'static>,
) -> Result<(), I2CError> {
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

    let mut data = [0x32, read_data[0]];
    i2c.write(CS43L22_I2C_ADR, &mut data).await?;
    // let mut write_data = [0x1];
    //let mut read_data = [0x0];
    //i2c.write_read(0x4A, &mut write_data, &mut read_data).await;
    let mut data = [0x0, 0x0];
    i2c.write(CS43L22_I2C_ADR, &mut data).await?;
    Ok(())
}

pub async fn tone_configure(i2c: &mut I2c<'static, Async, Master>) -> Result<(), I2CError> {
    i2c.write_simple(BEEP_TONE_CONFIG_ADR, (0b11 << 6) as u8)
        .await?;
    if let Ok(res) = i2c.read_simple(BEEP_TONE_CONFIG_ADR).await {
        info!("TONE:{}", res);
    }
    i2c.write_simple(BEEP_VOLUME_ADR, 0b110).await?;
    i2c.write_simple(BEEP_FREQ_ADR, 0b10000).await?;
    Ok(())
}

pub async fn power_toggle(
    i2c: &mut I2c<'static, Async, Master>,
    power: bool,
) -> Result<(), I2CError> {
    const POWER_UP: u8 = 0b10011110;
    const POWER_DOWN: u8 = 0b1;
    if power {
        i2c.write_simple(POWER_DOWN_ADR, POWER_UP).await
    } else {
        i2c.write_simple(POWER_DOWN_ADR, POWER_DOWN).await
    }
}

pub async fn headphone_speaker_toggle(
    i2c: &mut I2c<'static, Async, Master>,
    power: bool,
) -> Result<(), I2CError> {
    i2c.write_simple(HEADPHONE_POWER_ADR, 0x0).await?;
    info!("HEADPHONE:{}", i2c.read_simple(HEADPHONE_POWER_ADR).await?);
    Ok(())
}
//Todo setup i2s

#[embassy_executor::task]
pub async fn task(mut i2c: I2c<'static, Async, Master>, mut reset_pin: Output<'static>, mut mclk_pin: Output<'static>) {
    if let Ok(_) = initalise(&mut i2c, &mut reset_pin).await {
        tone_configure(&mut i2c).await;
        headphone_speaker_toggle(&mut i2c, true).await;
            Timer::after_secs(1).await;
        power_toggle(&mut i2c, true).await;
        info!("Tone should be playing");

        loop {
            Timer::after_nanos(40).await;
            mclk_pin.toggle();
        }
    } else {
        error!("Error initalising CS43L22");
    }
}
