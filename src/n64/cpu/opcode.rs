enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        Special = 0b000000,
        RegImm =  0b000001,

        J =       0b000010,
        Jal =     0b000011,

        Addi =    0b001000,
        Addiu =   0b001001,
        Slti =    0b001010,

        Andi =    0b001100,
        Ori =     0b001101,
        Xori =    0b001110,

        Lui =     0b001111,
        Mtc0 =    0b010000,

        Beq =     0b000100,
        Bne =     0b000101,
        Bgtz =    0b000111,

        Beql =    0b010100,
        Bnel =    0b010101,
        Blezl =   0b010110,

        Daddi =   0b011000,
        Daddiu =  0b011001,

        Lb =      0b100000,
        Lwl =     0b100010,
        Lw =      0b100011,
        Lbu =     0b100100,
        Lwr =     0b100110,
        Lwu =     0b100111,

        Sb =      0b101000,
        Sw =      0b101011,

        Cache =   0b101111,

        Ld =      0b110111,
        Sd =      0b111111,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum SpecialOpcode {
        Sll =    0b000000,

        Srl =    0b000010,
        Sra =    0b000011,
        Sllv =   0b000100,

        Srlv =   0b000110,
        Srav =   0b000111,

        Jr =     0b001000,

        Sync =   0b001111,

        Multu =  0b011001,

        Mfhi =   0b010000,
        Mflo =   0b010010,

        Dsllv =  0b010100,
        Dsrlv =  0b010110,
        Dsrav =  0b010111,

        Div =    0b011010,
        Divu =   0b011011,
        Dmult =  0b011100,
        Ddiv =   0b011110,
        Ddivu =  0b011111,

        Add =    0b100000,
        Addu =   0b100001,

        Sub =    0b100010,
        Subu =   0b100011,
        And =    0b100100,
        Or =     0b100101,
        Xor =    0b100110,
        Nor =    0b100111,

        Slt =    0b101010,
        Sltu =   0b101011,

        Dadd =   0b101100,
        Daddu =  0b101101,
        Dsub =   0b101110,
        Dsubu =  0b101111,

        Dsll =   0b111000,
        Dsrl =   0b111010,
        Dsra =   0b111011,
        Dsll32 = 0b111100,
        Dsrl32 = 0b111110,
        Dsra32 = 0b111111,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum RegImmOpcode {
        Bltz =   0b00000,
        Bgezl =  0b00011,
        Bgezal = 0b10001,
    }
}
