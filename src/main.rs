use unicorn_engine::{
    RegisterARM, 
    SECOND_SCALE, 
    unicorn_const::{Arch, Mode, Prot},
};


fn main(){

    let code = [
        0x00, 0x00, 0xa0, 0xe3, // mov r0, #0
        0x01, 0x00, 0xa0, 0xe3, // mov r0, #1
    ];

    let mut emu = unicorn_engine::Unicorn::new(Arch::ARM, Mode::LITTLE_ENDIAN)
        .expect("Failed to initialize Unicorn engine");

    emu.mem_map(0x1000, 0x4000, Prot::ALL)
        .expect("Failed to map memory");

    emu.mem_write(0x1000, &code)
        .expect("Failed to write code to memory");

    emu.add_code_hook(0x1000, 0x2000, |emu, address, size| {
        println!("Tracing at address {:#x}, size: {}", address, size);


        if address == 0x1000 {
            println!("fault injected: skip {} ", address);
            emu.reg_write(RegisterARM::PC, address + size as u64).unwrap();

        }


    }).expect("hook failed");

    emu.emu_start(0x1000, 0x1008, 10 * SECOND_SCALE, 2)
        .expect("Failed to start emulation");

    let r0_value = emu.reg_read(RegisterARM::R0)
        .expect("Failed to read R0");

    println!("Execution finished! Final R0 = {}", r0_value);

}