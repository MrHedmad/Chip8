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
fn test_opcode_dxny() {
    let mut processor: Chip8Processor = Chip8Processor::new();

    processor.i_register = 0; // Draw the first (0) sprite
    processor.registers[0x0] = 10;
    processor.registers[0x1] = 20; // At (10, 20)
    processor.execute(0xD051); // Draw x=0, 5 rows, y=1

    let mut expected_mem: [bool; DISPLAY_MEM_HEIGHT * DISPLAY_MEM_WIDTH] = [false; DISPLAY_MEM_HEIGHT * DISPLAY_MEM_WIDTH];
    // Draw the 0 manually
    expected_mem[10] = true;
    expected_mem[11] = true;
    expected_mem[12] = true;
    expected_mem[13] = true;

    expected_mem[74] = true;
    expected_mem[77] = true;

    expected_mem[138] = true;
    expected_mem[141] = true;

    expected_mem[202] = true;
    expected_mem[205] = true;
    
    expected_mem[266] = true;
    expected_mem[267] = true;
    expected_mem[268] = true;
    expected_mem[269] = true;
    //assert_eq!(processor.display, expected_mem);

    processor.execute(0xD051); // Draw x=0, 5 rows, y=1

    assert_eq!(processor.display, [false; DISPLAY_MEM_HEIGHT * DISPLAY_MEM_WIDTH]);
    assert_eq!(processor.registers[0xF], 1);
}