use anyhow::{Result, Context};

use serial::SerialPort;

use pir2c::I2CConn;

/** hal
 */
#[derive(argh::FromArgs)]
struct Args {
    #[argh(positional)]
    /// serial port path
    tty: String,
}

fn main() -> Result<()> {

    let args: Args = argh::from_env();

    let mut p = serial::open(&args.tty).context("opening {args.tty}")?;
    p.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        Ok(())
    })?;

    let mut conn = I2CConn::new(p)?;

    conn.test()?;

    Ok(())
}
