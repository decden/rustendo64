enum_from_primitive! {
    #[derive(Debug)]
    pub enum RspOpcode {
        J =       0b000010,
        Ori =     0b001101,
        Sh =      0b101001,

        Unknown = 0b000000,
    }
}