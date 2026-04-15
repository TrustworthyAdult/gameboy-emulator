use crate::instruction::Opcode;

#[cfg(feature = "opcode-coverage")]
#[test]
fn opcode_coverage() {
    let mut count = 0;

    for opcode in 0x00u8..=0xFF {
        if Opcode::try_from(opcode).is_ok() {
            count += 1;
        } else {
            println!("❌ Missing: 0x{:02X}", opcode);
        }
    }

    println!("✅ Implemented opcodes: {}/256", count);
    assert_eq!(count, 256, "Still missing opcodes!");
}
