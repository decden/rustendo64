enum_from_primitive! {
    #[derive(Debug)]
    pub enum RspOpcode {
        Special = 0b000000,
        J =       0b000010,
        Addi =    0b001000,
        Addiu =   0b001001,
        Andi =    0b001100,
        Ori =     0b001101,
        Xori =    0b001110,
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
        Srl =    0b000010,
        Sra =    0b000011,
        Sllv =   0b000100,
        Srlv =   0b000110,
        Srav =   0b000111,
        Break =  0b001101,
        Add =    0b100000,
        Addu =   0b100001,
        Sub =    0b100010,
        Subu =   0b100011,
        And =    0b100100,
        Or =     0b100101,
        Xor =    0b100110,
        Nor =    0b100111,
    }
}