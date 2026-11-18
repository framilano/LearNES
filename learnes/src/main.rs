use std::{env, fs};

/**
    Load file into ROM
**/
fn reset(
    rom: &mut [u8; 0x8000],
    program_counter: &mut u16,
    nes_file_path: &String
) {
    let mut headered_rom: Vec<u8> = fs::read(nes_file_path).expect("Failed to read NES ROM file");

    //Copying file bytes into ROM, excluding space from 0x0 to 0x10 (it's just the file header)
    headered_rom.copy_within(0x10.., 0);
    rom.copy_from_slice(headered_rom[0..0x8000].iter().as_slice());

    //Now that rom is loaded into our ROM array, we read 0xFFFC and 0xFFFD, these two together tell us the position where to point the PC
    let pch = rom[rom.len() - 3] as u16;
    let pcl = rom[rom.len() - 4] as u16;

    *program_counter = (pch << 8) | pcl;   //We shift high 8 bits and bitwise OR with low, so we obtain the actual address we need to start from
    print!("ROM loaded and PC set");
}

/**
    Read function, used for reading from RAM and ROM
**/
fn read(
    address: u16,
    ram: &[u8; 0x800],
    rom: &[u8; 0x8000]
) -> u8  {
    let mut actual_address = address;

    //-------------- RAM --------------
    if actual_address < 0x800 { return ram[actual_address as usize] }

    //NES instructions could still try to address in RAM up to 1FFF, so we apply RAM MIRRORING
    //to read only the 11 LSB (I have no idea why 1FFF is "max" value for RAM reading in this scenario and why only 11 bits actually count)
    if actual_address >= 0x800 && actual_address <= 0x1FFF {
        actual_address &= 0x07FF;     //bitwise operation saving only 11 LSB
        return ram[actual_address as usize]
    }


    //-------------- ROM --------------
    if actual_address >= 0x8000 {     //Instruction is asking for address >= 0x8000, we must be in ROM space
        actual_address &= 0xFFF;      //Removing 12 LSB to start from 0 while reading from ROM
        return rom[actual_address as usize]
    }

    1
}

fn main() {

    // Parsing ROM file from ARGS
    let args: Vec<String> = env::args().collect();

    // Declaring LearNES fundamental vars

    let mut program_counter: u16 = 0;
    let mut a: u8 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;

    let mut ram: [u8; 0x800] = [0; 0x800];      // RAM space from $000 to $7FF
    let mut rom: [u8; 0x8000] = [0; 0x8000];    // ROM space from $8000 to $FFFF

    // Resetting ROM and setting Program Counter
    reset(&mut rom, &mut program_counter, &args[1]);

    // Now we can read ROM instructions
}
