#[derive(Debug, Default)]
pub struct RegCause {
    bd: bool, // indicates whether the last exception occured while in a branch delay slot
    ce: u8, // coprocessor involved in a copropcessor unusable exception

    ip_timer: bool, // Timer interrup
    ip_external: [bool; 5],
    ip_software: [bool; 2],

    exception_code: u8, // Exception code
}

impl RegCause {
    pub fn setSoftwareInterruptPendingFields(&mut self, value: u32)
    {
        let newValue: RegCause = value.into();
        self.ip_software = newValue.ip_software;
        println!("Setting sw interrupt pending registers to {:?}", self.ip_software);
    }
    pub fn clearTimerInterruptPending(&mut self)
    {
        self.ip_timer = false;
    }

    pub fn to_u32(&self) -> u32 {
        (if self.bd { 1 << 31 } else { 0 }) |
        ((self.ce as u32) << 28) |
        (if self.ip_timer { 1 << 15 } else { 0 }) |
        (if self.ip_external[0] { 1 << 10 } else { 0 }) |
        (if self.ip_external[1] { 1 << 11 } else { 0 }) |
        (if self.ip_external[2] { 1 << 12 } else { 0 }) |
        (if self.ip_external[3] { 1 << 13 } else { 0 }) |
        (if self.ip_external[4] { 1 << 14 } else { 0 }) |
        (if self.ip_software[0] { 1 <<  8 } else { 0 }) |
        (if self.ip_software[1] { 1 <<  9 } else { 0 }) |
        (self.exception_code as u32) << 2
    }
}

impl From<u32> for RegCause {
    fn from(value: u32) -> Self {
        RegCause {
            bd: (value & (1 << 31)) != 0,
            ce: ((value >> 28) & 0x03) as u8,

            ip_timer: (value & (1 << 15)) != 0,
            ip_external: [
                (value & (1 << 10)) != 0,
                (value & (1 << 11)) != 0,
                (value & (1 << 12)) != 0,
                (value & (1 << 13)) != 0,
                (value & (1 << 14)) != 0,
            ],
            ip_software: [
                (value & (1 << 8)) != 0,
                (value & (1 << 9)) != 0,
            ],

            exception_code: ((value >> 2) & 0x1F) as u8,
        }
    }
}