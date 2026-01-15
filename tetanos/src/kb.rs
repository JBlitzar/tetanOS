use x86_64::instructions::port::Port;

pub fn read_scancode() -> Option<u8> {
    unsafe {
        let mut status_port = Port::new(0x64);
        let status: u8 = status_port.read();
        if status & 0x01 == 0 {
            return None;
        }
        let mut port = Port::new(0x60);
        return Some(port.read());
    }
}