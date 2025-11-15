use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <pty_device> <message>", args[0]);
        eprintln!("Example: {} /dev/pts/15 \"Hello from TIOCSTI!\"", args[0]);
        std::process::exit(1);
    }

    let pty_device = &args[1];
    let message = &args[2];

    println!("📌 Target: {}", pty_device);
    println!("📤 Injecting: {}\n", message);

    let pty = OpenOptions::new().write(true).open(pty_device)?;
    let fd = pty.as_raw_fd();
    const TIOCSTI: libc::c_ulong = 0x5412;

    for byte in message.as_bytes() {
        unsafe {
            if libc::ioctl(fd, TIOCSTI, byte as *const u8) < 0 {
                return Err(anyhow::anyhow!("TIOCSTI failed - may be disabled in kernel"));
            }
        }
    }

    unsafe {
        let newline: u8 = b'\n';
        libc::ioctl(fd, TIOCSTI, &newline as *const u8);
    }

    println!("✅ Injected as keyboard input!");
    Ok(())
}
