use std::fmt;

#[derive(Copy, Clone)]
pub enum Instruction {
    /// Clear the display.
    ClearDisplay,
    /// Return from a subroutine.
    Return,
    /// Jump to location _nnn_.
    JumpToAddress(u16),
    /// Call subroutine at _nnn_.
    CallAddress(u16),
    /// Skip next instruction if V<i>x</i> = _kk_.
    SkipIfVxEqualKk(u8, u8),
    /// Skip next instruction if V<i>x</i> != _kk_.
    SkipIfVxNotEqualKk(u8, u8),
    /// Skip next instruction if V<i>x</i> != V<i>y</i>.
    SkipIfVxEqualVy(u8, u8),
    /// Set V<i>x</i> = _kk_.
    LoadVxKk(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> + _kk_.
    AddVxKk(u8, u8),
    /// Set V<i>x</i> = V<i>y</i>.
    LoadVxVy(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> OR V<i>y</i>.
    OrVxVy(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> AND V<i>y</i>.
    AndVxVy(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> XOR V<i>y</i>.
    XorVxVy(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> + V<i>y</i>, set VF = carry.
    AddVxVy(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> - V<i>y</i>, set VF = NOT borrow.
    SubVxVy(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> SHR 1.
    ShiftRight(u8, u8),
    /// Set V<i>x</i> = V<i>y</i> - V<i>x</i>, set VF = NOT borrow.
    SubNVxVy(u8, u8),
    /// Set V<i>x</i> = V<i>x</i> SHL 1.
    ShiftLeft(u8, u8),
    /// Skip next instruction if V<i>x</i> != V<i>y</i>.
    SkipIfVxNotEqualVy(u8, u8),
    /// Set I = _nnn_.
    LoadAddr(u16),
    /// Jump to location _nnn_ + V0.
    JumpToAddressPlusV0(u16),
    /// Set V<i>x</i> = random _byte_ AND _kk_.
    RandomAnd(u8, u8),
    /// Display _n_-byte sprite starting at memory location I
    /// at (V<i>x</i>, V<i>y</i>), set VF = collision.
    DrawVxVyN(u8, u8, u8),
    /// Skip next instruction if key with the value of V<i>x</i> is pressed.
    SkipIfKeyPressed(u8),
    /// Skip next instruction if key with the value of V<i>x</i> is not pressed.
    SkipIfKeyNotPressed(u8),
    /// Set V<i>x</i> = delay timer value.
    LoadDelayTimer(u8),
    /// Wait for a ke press, store the value of the key in V<i>x</i>.
    WaitForKey(u8),
    /// Set delay timer = V<i>x</i>.
    SetDelayTimer(u8),
    /// Set sound timer = V<i>x</i>.
    SetSoundTimer(u8),
    /// Set I = I + V<i>x</i>.
    AddVxToI(u8),
    /// Set I = location of sprite for digit V<i>x</i>.
    LoadSpriteLocationToI(u8),
    /// Store BCD representation of V<i>x</i> in memory locations I, I+1, and I+2.
    LoadBcdToI(u8),
    /// Store registers V0 through V<i>x</i> in memory starting at location I.
    LoadV0ThroughVxToI(u8),
    /// Read registers V0 through V<i>x</i> from memory starting at location I.
    LoadIToV0ThroughVx(u8),
    /// Invalid or unimplemented instruction.
    Invalid(u8, u8, u8, u8),
}

impl Instruction {
    pub fn parse(data: (u8, u8)) -> Instruction {
        let nibbles = (data.0 >> 4, data.0 & 0x0f, data.1 >> 4, data.1 & 0x0f);
        let nnn = ((data.0 as u16) << 8 | data.1 as u16) & 0x0fff;
        let x = nibbles.1;
        let y = nibbles.2;
        let kk = data.1;

        match nibbles {
            (0, 0, 0xE, 0) => Instruction::ClearDisplay,
            (0, 0, 0xE, 0xE) => Instruction::Return,
            (1, _, _, _) => Instruction::JumpToAddress(nnn),
            (2, _, _, _) => Instruction::CallAddress(nnn),
            (3, _, _, _) => Instruction::SkipIfVxEqualKk(x, kk),
            (4, _, _, _) => Instruction::SkipIfVxNotEqualKk(x, kk),
            (5, _, _, 0) => Instruction::SkipIfVxEqualVy(x, y),
            (6, _, _, _) => Instruction::LoadVxKk(x, kk),
            (7, _, _, _) => Instruction::AddVxKk(x, kk),
            (8, _, _, 0) => Instruction::LoadVxVy(x, y),
            (8, _, _, 1) => Instruction::OrVxVy(x, y),
            (8, _, _, 2) => Instruction::AndVxVy(x, y),
            (8, _, _, 3) => Instruction::XorVxVy(x, y),
            (8, _, _, 4) => Instruction::AddVxVy(x, y),
            (8, _, _, 5) => Instruction::SubVxVy(x, y),
            (8, _, _, 6) => Instruction::ShiftRight(x, y),
            (8, _, _, 7) => Instruction::SubNVxVy(x, y),
            (8, _, _, 0xE) => Instruction::ShiftLeft(x, y),
            (9, _, _, 0) => Instruction::SkipIfVxNotEqualVy(x, y),
            (0xA, _, _, _) => Instruction::LoadAddr(nnn),
            (0xB, _, _, _) => Instruction::JumpToAddressPlusV0(nnn),
            (0xC, _, _, _) => Instruction::RandomAnd(x, kk),
            (0xD, _, _, _) => Instruction::DrawVxVyN(x, y, nibbles.3),
            (0xE, _, 9, 0xE) => Instruction::SkipIfKeyPressed(x),
            (0xE, _, 0xA, 1) => Instruction::SkipIfKeyNotPressed(x),
            (0xF, _, 0, 7) => Instruction::LoadDelayTimer(x),
            (0xF, _, 0, 0xA) => Instruction::WaitForKey(x),
            (0xF, _, 1, 5) => Instruction::SetDelayTimer(x),
            (0xF, _, 1, 8) => Instruction::SetSoundTimer(x),
            (0xF, _, 1, 0xE) => Instruction::AddVxToI(x),
            (0xF, _, 2, 9) => Instruction::LoadSpriteLocationToI(x),
            (0xF, _, 3, 3) => Instruction::LoadBcdToI(x),
            (0xF, _, 5, 5) => Instruction::LoadV0ThroughVxToI(x),
            (0xF, _, 6, 5) => Instruction::LoadIToV0ThroughVx(x),
            (_, _, _, _) => Instruction::Invalid(nibbles.0, x, y, nibbles.3),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::ClearDisplay => write!(f, "CLS"),
            Instruction::Return => write!(f, "RET"),
            Instruction::JumpToAddress(addr) => write!(f, "JP {:03X}", addr),
            Instruction::CallAddress(addr) => write!(f, "CALL {:02X}", addr),
            Instruction::SkipIfVxEqualKk(vx, kk) => write!(f, "SE [{:02X}] {:02X}", vx, kk),
            Instruction::SkipIfVxNotEqualKk(vx, kk) => write!(f, "SNE [{:02X}], {:02X}", vx, kk),
            Instruction::SkipIfVxEqualVy(vx, vy) => write!(f, "SE [{:02X}], [{:02X}]", vx, vy),
            Instruction::LoadVxKk(vx, kk) => write!(f, "LD {:02X}, {:02X}", vx, kk),
            Instruction::AddVxKk(vx, kk) => write!(f, "ADD [{:02X}], {:02X}", vx, kk),
            Instruction::LoadVxVy(vx, vy) => write!(f, "LD {:02X}, [{:02X}]", vx, vy),
            Instruction::OrVxVy(vx, vy) => write!(f, "OR [{:02X}], [{:02X}]", vx, vy),
            Instruction::AndVxVy(vx, vy) => write!(f, "AND [{:02X}], [{:02X}]", vx, vy),
            Instruction::XorVxVy(vx, vy) => write!(f, "XOR [{:02X}], [{:02X}]", vx, vy),
            Instruction::AddVxVy(vx, vy) => write!(f, "ADD [{:02X}], [{:02X}]", vx, vy),
            Instruction::SubVxVy(vx, vy) => write!(f, "SUB [{:02X}], [{:02X}]", vx, vy),
            Instruction::ShiftRight(vx, vy) => write!(f, "SHR [{:02X}], ({:02X})", vx, vy),
            Instruction::SubNVxVy(vx, vy) => write!(f, "SUBN [{:02X}], [{:02X}]", vx, vy),
            Instruction::ShiftLeft(vx, vy) => write!(f, "SHL [{:02X}], ({:02X})", vx, vy),
            Instruction::SkipIfVxNotEqualVy(vx, vy) => write!(f, "SNE [{:02X}], [{:02X}]", vx, vy),
            Instruction::LoadAddr(nnn) => write!(f, "LD I, {:03X}", nnn),
            Instruction::JumpToAddressPlusV0(nnn) => write!(f, "JP [0], {:03X}", nnn),
            Instruction::RandomAnd(vx, kk) => write!(f, "RND  [{:02X}], {:02X}", vx, kk),
            Instruction::DrawVxVyN(vx, vy, n) => {
                write!(f, "DRW [{:02X}], [{:02X}], {:02X}", vx, vy, n)
            }
            Instruction::SkipIfKeyPressed(vx) => write!(f, "SKP [{:02X}]", vx),
            Instruction::SkipIfKeyNotPressed(vx) => write!(f, "SKNP [{:02X}]", vx),
            Instruction::LoadDelayTimer(vx) => write!(f, "LD [{:02X}], DT", vx),
            Instruction::WaitForKey(vx) => write!(f, "LD [{:02X}], K", vx),
            Instruction::SetDelayTimer(vx) => write!(f, "LD DT, [{:02X}]", vx),
            Instruction::SetSoundTimer(vx) => write!(f, "LD ST, [{:02X}]", vx),
            Instruction::AddVxToI(vx) => write!(f, "ADD I, [{:02X}]", vx),
            Instruction::LoadSpriteLocationToI(vx) => write!(f, "LD I, S[{:02X}]", vx),
            Instruction::LoadBcdToI(vx) => write!(f, "LD B, [{:02X}]", vx),
            Instruction::LoadV0ThroughVxToI(vx) => write!(f, "LD [I], [{:02X}]", vx),
            Instruction::LoadIToV0ThroughVx(vx) => write!(f, "LD [{:02X}], [I]", vx),
            Instruction::Invalid(_, _, _, _) => write!(f, "-"),
        }
    }
}
