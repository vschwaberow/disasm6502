extern crate disasm6502;
use disasm6502::instruction::*;

// 6502 disassembler example
fn main()
{
    let bytes = vec![0x05, 0x0B, 0x6C, 0x01, 0x02,
                     0x0A, 0xA2, 0xFF, 0x20, 0x02,
                     0xFD, 0x78, 0xD0, 0xFC, 0x1D,
                     0x05, 0x1E, 0x04, 0x15, 0x02,
                     0x96, 0xAB, 0x58, 0x61, 0x01,
                     0x91, 0xFB];

    // disassemble...
    let instructions = disasm6502::from_array(&bytes).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // ...and print!
    for i in instructions.iter() {
        println!("{}", format_strings(&i));
    }

    println!("${:04X}: .END", bytes.len());
}


// helper string formatter
fn format_strings(instruction: &Instruction) -> String {
    // format cycle count
    let cc = if instruction.extra_cycle {
        format!("(*{}) ", instruction.cycles)
    } else {
        format!("({})  ", instruction.cycles)
    };
    
    // format read registers
    let rr = if let Some(ref reg_read) = instruction.registers_read {
        let mut r_str = String::from(" Reads:[");
        for r in reg_read.iter() {
            r_str.push_str(format!("{}", r).as_str());
        }
        r_str.push_str("]");
        r_str.to_owned()
    } else {
        String::from("          ")
    };

    // format written registers
    let rw = if let Some(ref reg_written) = instruction.registers_written {
        let mut r_str = String::from("   Writes:[");
        for r in reg_written.iter() {
            r_str.push_str(format!("{}", r).as_str());
        }
        r_str.push_str("]");
        r_str.to_owned()
    } else {
        String::from("")
    };

    // add extra spacing for better looks!
    let mut spacing = String::new();
    for _ in 0..(32 - format!("{}", instruction).len()) {
        spacing.push_str(" ");
    }

    format!("{}{}{}{}{}", instruction, spacing, cc, rr, rw)
}