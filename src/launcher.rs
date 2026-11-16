use crate::secret;

use std::{
    error::Error,
    io::Write,
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
    thread::sleep,
    time::Duration,
};

const LAUNCHER: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4646);

pub fn run() -> Result<(), Box<dyn Error>> {
    let totp = secret::load()?;

    loop {
        if let Ok(stream) = TcpStream::connect_timeout(&LAUNCHER.into(), Duration::from_millis(500))
        {
            match send(stream, &totp.generate_current().to_string()) {
                Ok(status) => println!("OTP sent: {status}"),
                Err(e) => eprintln!("Send failed: {e}"),
            }
        }
        sleep(Duration::from_secs(1));
    }
}

fn send(mut stream: TcpStream, otp_code: &str) -> std::io::Result<String> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    write!(
        stream,
        "GET /ffxivlauncher/{otp_code} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        LAUNCHER.ip()
    )?;
    let resp = std::io::read_to_string(stream).unwrap_or_default();
    Ok(resp.lines().next().unwrap_or("no response").into())
}
