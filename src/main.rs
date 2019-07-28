use std::io;

fn main() -> io::Result<()> {
    tun_tap::new("tun0", tun_tap::Mode::Tun).expect("failed to cr")?;
    Ok(())
}
