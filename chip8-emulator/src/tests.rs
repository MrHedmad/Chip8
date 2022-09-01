use rand::{thread_rng, Rng};

use crate::*;

#[test]
fn test_opcode_0000() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_00e0() {
    let mut processor = Chip8Processor::new();

    // Add something to the screen
    let mut new_display = [true; DISPLAY_MEM_HEIGHT * DISPLAY_MEM_WIDTH];
    thread_rng().fill(&mut new_display);

    processor.display = new_display;

    processor.execute(0x00E0);

    let expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_00ee_2nnn() {
    let mut processor = Chip8Processor::new();

    // Simulate a jump in memory
    processor.execute(0x2210); // Jump to subroutine @ pos. 210 
    
    assert_eq!(processor.stack, [0x200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(processor.program_counter, 0x210);

    processor.execute(0x00EE); // Return
    assert_eq!(processor.stack, [0; 16]);
    assert_eq!(processor.program_counter, START_ADDRESS);

    // Do it again but jump twice
    processor.execute(0x2210); // Jump to subroutine @ pos. 210 
    processor.execute(0x2230); // Jump to subroutine @ pos. 230 

    assert_eq!(processor.stack, [0x200, 0x210, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(processor.program_counter, 0x230);

    processor.execute(0x00EE); // Return
    assert_eq!(processor.stack, [0x200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(processor.program_counter, 0x210);

    processor.execute(0x00EE); // Return
    assert_eq!(processor.stack, [0; 16]);
    assert_eq!(processor.program_counter, START_ADDRESS);

}


#[test]
fn test_opcode_1nnn() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x1300);
    assert_eq!(processor.program_counter, 0x300);

    processor.execute(0x1353);
    assert_eq!(processor.program_counter, 0x353);
}


#[test]
fn test_opcode_3xkk() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x3500); // V5 == 0, skip 2

    assert_eq!(processor.program_counter, START_ADDRESS + 2);

    processor.execute(0x3523); // Should do nothing.
    assert_eq!(processor.program_counter, START_ADDRESS + 2);
}

#[test]
fn test_opcode_4xkk() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x4500);
    assert_eq!(processor.program_counter, START_ADDRESS);

    processor.execute(0x4210);
    assert_eq!(processor.program_counter, START_ADDRESS + 2);
}

#[test]
fn test_opcode_5xy0() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x5F00);
    assert_eq!(processor.program_counter, START_ADDRESS + 2);

    processor.registers[0xF] = 10;

    processor.execute(0x5F00);
    assert_eq!(processor.program_counter, START_ADDRESS + 2);
}

#[test]
fn test_opcode_6xkk() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x601F);
    assert_eq!(processor.registers[0x0], 0x1F);

    processor.execute(0x6F88);
    assert_eq!(processor.registers[0xF], 0x88);

    assert_eq!(processor.registers[0x5], 0);
}

#[test]
fn test_opcode_7xkk() {
    let mut processor = Chip8Processor::new();

    processor.registers[0x0] += 0x10;
    processor.execute(0x7025);

    assert_eq!(processor.registers[0x0], 0x10 + 0x25);

    processor.execute(0x7F44);
    assert_eq!(processor.registers[0xF], 0x44);
}

#[test]
fn test_opcode_8xy0() {
    let mut processor = Chip8Processor::new();

    processor.registers[0x3] = 0x23;
    processor.execute(0x83F0);

    assert_eq!(processor.registers[0x3], processor);
}

#[test]
fn test_opcode_8xy1() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_8xy2() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_8xy3() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_8xy4() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_8xy5() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_8xy6() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_8xy7() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_8xye() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_9xy0() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_annn() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_bnnn() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_cxkk() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_dxyn() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_ex9e() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_exa1() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}


#[test]
fn test_opcode_fx07() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx0a() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx15() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx18() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx1e() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx29() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx33() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx55() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}

#[test]
fn test_opcode_fx65() {
    let mut processor = Chip8Processor::new();

    processor.execute(0x0000);

    let mut expected_state = Chip8Processor::new();

    assert_eq!(processor, expected_state);
}