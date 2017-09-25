enum_from_primitive! {
    #[derive(Debug)]
    pub enum RspOpcode {
        Special = 0b000000,
        J =       0b000010,
        Addi =    0b001000,
        Ori =     0b001101,
        Lui =     0b001111,
        Lw =      0b100011,
        Sh =      0b101001,
        Sw =      0b101011,
    }

}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum RspSpecialOpcode {
        Sll =    0b000000,
        Add =    0b100000,
        Break =  0b001101,
    }
}