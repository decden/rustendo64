enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        Special = 0b000000,
        RegImm =  0b000001,

        J =       0b000010,
        Jal =     0b000011,

        Blez =    0b000110,

        Addi =    0b001000,
        Addiu =   0b001001,
        Slti =    0b001010,
        Sltiu =   0b001011,

        Andi =    0b001100,
        Ori =     0b001101,
        Xori =    0b001110,

        Lui =     0b001111,
        Cop0 =    0b010000,
        Cop1 =    0b010001,

        Beq =     0b000100,
        Bne =     0b000101,
        Bgtz =    0b000111,

        Beql =    0b010100,
        Bnel =    0b010101,
        Blezl =   0b010110,

        Daddi =   0b011000,
        Daddiu =  0b011001,

        Lb =      0b100000,
        Lh =      0b100001,
        Lwl =     0b100010,
        Lw =      0b100011,
        Lbu =     0b100100,
        Lhu =     0b100101,
        Lwr =     0b100110,
        Lwu =     0b100111,


        Sb =      0b101000,
        Sh =      0b101001,
        Swl =     0b101010,
        Sw =      0b101011,
        Swr =     0b101110,

        Cache =   0b101111,

        Lwc1 =    0b110001,
        Ldc1 =    0b110101,

        Ld =      0b110111,

        Swc1 =    0b111001,
        Sdc1 =    0b111101,

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
        Jalr =   0b001001,

        Sync =   0b001111,

        Multu =  0b011001,

        Mfhi =   0b010000,
        Mthi =   0b010001,
        Mflo =   0b010010,
        Mtlo =   0b010011,

        Dsllv =  0b010100,
        Dsrlv =  0b010110,
        Dsrav =  0b010111,

        Mult =   0b011000,

        Div =    0b011010,
        Divu =   0b011011,
        Dmult =  0b011100,
        Dmultu = 0b011101,
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
        Bgez =   0b00001,
        Bgezl =  0b00011,
        Bgezal = 0b10001,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum Cop0Opcode {
        Mfc0 =    0b00000,
        Mtc0 =    0b00100,
        Co =      0b10000,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum Cop0CoOpcode {
        Tlbwi =   0b000010,
        Eret =    0b011000,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum Cop1Opcode {
        Add =    0b000000,
        Sub =    0b000001,
        Mul =    0b000010,
        Div =    0b000011,
        Sqrt =   0b000100,
        Abs =    0b000101,
        Mov =    0b000110,
        Neg =    0b000111,
        RoundL = 0b001000,
        TruncL = 0b001001,
        CeilL =  0b001010,
        FloorL = 0b001011,
        RoundW = 0b001100,
        TruncW = 0b001101,
        CeilW =  0b001110,
        FloorW = 0b001111,

        CvtS =   0b100000,
        CvtD =   0b100001,
        CvtW =   0b100100,
        CvtL =   0b100101,

        // 17.6 FPU Instruction Opcode Bit Encoding
        Cf =     0b110000,
        Cun =    0b110001,
        Ceq =    0b110010,
        Cueq =   0b110011,
        Colt =   0b110100,
        Cult =   0b110101,
        Cole =   0b110110,
        Cule =   0b110111,
        Csf =    0b111000,
        Cngle =  0b111001,
        Cseq =   0b111010,
        Cngl =   0b111011,
        Clt =    0b111100,
        Cnge =   0b111101,
        Cle =    0b111110,
        Cngt =   0b111111,
    }
}
