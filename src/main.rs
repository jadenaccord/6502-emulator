// std::env::set_var::("RUST_BACKTRACE", "1");

pub mod cpu;
pub mod bus;
pub mod opcodes;

#[allow(unused_imports)]
use cpu::CPU;
#[allow(unused_imports)]
use cpu::Flags;

#[allow(unused_imports)]
use bus::Bus;

fn main() {
    let bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(bus);
    cpu.write(0x00F1, 0x27);
    cpu.quick_start(vec![0xA9, 0x03, 0xA2, 0x10, 0x75, 0xE1, 0x00]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_imm_flags() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        cpu.quick_start(vec![0xA9, 0x05, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x05);
        assert!(!cpu.get_flag(Flags::Z));
        assert!(!cpu.get_flag(Flags::N));
    }

    #[test]
    fn test_lda_imm_zero_flag() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        cpu.quick_start(vec![0xA9, 0x00, 0x00]);

        assert!(cpu.get_flag(Flags::Z));
    }

    #[test]
    fn test_lda_zp0() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        // write 0x55 to address 0x10
        cpu.write(0x10, 0x55);
        // LDA from address 0x10
        cpu.quick_start(vec![0xA5, 0x10, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x55);
    }

    #[test]
    fn test_add_to_a() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        {   // no carry, no overflow
            println!("\nNo carry, no overflow");
            cpu.set_a_reg(0x0A);
            cpu.add_to_a(0x10);
            println!("{}", cpu.get_a_reg());
            assert_eq!(cpu.get_a_reg(), 0x1A);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // no carry -> carry, no overflow
            println!("\nNo carry -> carry, no overflow");
            cpu.set_a_reg(0xFF);
            cpu.add_to_a(0x01);
            assert_eq!(cpu.get_a_reg(), 0);
            assert!(cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // carry -> no carry, no overflow
            println!("\nCarry -> no carry, no overflow");
            cpu.set_flag(Flags::C, true);
            cpu.set_a_reg(0x0A);
            cpu.add_to_a(0x10);
            assert_eq!(cpu.get_a_reg(), 0x1A + 0x01);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        {   // no carry, no overflow -> overflow
            println!("\nNo carry, no overflow -> overflow");
            cpu.set_a_reg(0x7F);
            cpu.add_to_a(0x04);
            assert_eq!(cpu.get_a_reg(), 0x83);
            assert!(cpu.get_flag(Flags::V));
        }
    }

    #[test]
    fn test_adc_imm() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        // No carry -> no carry
        {
            cpu.quick_start(vec![0xA9, 0x10, 0x69, 0x02, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x12);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        // Carry -> no carry
        {
            // TODO: how to verify carry?
            cpu.set_flag(Flags::C, true);
            cpu.quick_start(vec![0xA9, 0x10, 0x69, 0x02, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x12);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        // No carry -> carry
        {
            cpu.quick_start(vec![0xA9, 0xFE, 0x69, 0x03, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x01);
            assert!(cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::V));
        }

        // Overflow -> no overflow
        {
            cpu.quick_start(vec![]);
        }

        // No overflow -> overflow
    }

    #[test]
    fn test_and_imm() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        // LDA(IMM) with 0x6b, AND(IMM) with 0x2c
        cpu.quick_start(vec![0xA9, 0x6b, 0x29, 0x2c, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x28);
    }

    // write number to memory, lda immediate, ldx immediate, then adc with zpx
    #[test]
    fn test_adc_zpx() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);
        cpu.write(0x00F1, 0x27);
        cpu.quick_start(vec![0xA9, 0x03, 0xA2, 0x10, 0x75, 0xE1, 0x00]);

        assert_eq!(cpu.get_a_reg(), 0x2A);
    }

    #[test]
    fn test_asl_acc() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        {
            cpu.quick_start(vec![0xA9, 0b0010_1000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0b0101_0000);
            assert!(!cpu.get_flag(Flags::C));
            assert!(!cpu.get_flag(Flags::N));
            assert!(!cpu.get_flag(Flags::Z));
        }

        {
            cpu.quick_start(vec![0xA9, 0b1010_0000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0b0100_0000);
            assert!(cpu.get_flag(Flags::C));
            println!("A: {}", cpu.get_a_reg());
        }

        {
            cpu.quick_start(vec![0xA9, 0b1000_0000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0x00);
            assert!(cpu.get_flag(Flags::C));
            assert!(cpu.get_flag(Flags::Z));
            assert!(!cpu.get_flag(Flags::N));
        }

        {
            cpu.quick_start(vec![0xA9, 0b0100_0000, 0x0A, 0x00]);
            assert_eq!(cpu.get_a_reg(), 0b1000_0000);
            assert!(cpu.get_flag(Flags::N));
            assert!(!cpu.get_flag(Flags::Z));
            assert!(!cpu.get_flag(Flags::C));
        }
    }

    #[test]
    fn test_asl_abs() {
        let bus: Bus = Bus::new();
        let mut cpu: CPU = CPU::new(bus);

        // TODO: LDA(IMM): 0b0010_1000, STA(ABS) to addr: 0x3B01, ASL(ABS) to addr: 0x3B01
        {
            cpu.quick_start(vec![0xA9, 0b0010_1000, 0x8D, 0x01, 0x3B, 0x0E, 0x01, 0x3B, 0x00]);
            assert_eq!(cpu.read(0x3B), 0b0101_0000);
        }
    }

    // TODO: test ASL with different mode

    // TODO: test all addressing modes (should be relatively simple, though, might not be necessary)
    // #[test]
    // fn test_addressing_modes() {
    //     let bus: Bus = Bus::new();
    //     let mut cpu: CPU = CPU::new(bus);
    //     use cpu::AddressingMode;
    //     assert_eq!(cpu.get_address(AddressingMode::IMM), cpu.get_pc() - 1);
    // }

    // TODO: test all (?) instructions

    // TODO: test all opcodes (or is it too much?)
}
