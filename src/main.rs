use unicorn_engine::{
    RegisterARM, SECOND_SCALE, unicorn_const::{Arch, Mode, Prot},
};







fn run_simulation(addr:Option<u64>) -> u64{

        let code = [
        0x00, 0x00, 0xa0, 0xe3, // mov r0, #0
        0x00, 0x00, 0xa0, 0xe3, // mov r0, #0
        0x01, 0x00, 0xa0, 0xe3, // mov r0, #1
    ];

    let mut emu = unicorn_engine::Unicorn::new(Arch::ARM, Mode::LITTLE_ENDIAN)
        .expect("Failed to initialize Unicorn engine IN RUN SIMULATION");

    emu.mem_map(0x1000, 0x4000, Prot::ALL)
        .expect("Failed to map memory in RUN SIMULATION");

    emu.mem_write(0x1000, &code)    
        .expect("Failed to write code to memory in RUN SIMULATION");

    let mut input = String::new();        
    std::io::stdin() 
        .read_line(&mut input )
        .unwrap(); 
    let r1 = input.trim().parse().unwrap();
    emu.reg_write(RegisterARM::R1, r1)
        .expect("failed to write the input in R1");

    if let Some(addr) = addr {
        emu.add_code_hook(0x1000, 0x4000, move |emu, address, size|{
    
            println!("Tracing at address {:#x}, size: {}", address, size);
            if address == addr {
                println!("fault injected: skip {} ", address);
                emu.reg_write(RegisterARM::PC, address + size as u64).unwrap();
            }   
        }).expect("Hook failed");
    };

        emu.emu_start(0x1000, 0x1008, 10 * SECOND_SCALE, 2)
        .expect("Failed to start emulation");

    let r0_value = emu.reg_read(RegisterARM::R0)
        .expect("Failed to read R0");

    r0_value



}

fn main() {
    println!("--- Test 1: Normal Execution (No Fault) ---");
    println!("Enter R1 value:");
    let r0_normal = run_simulation(None);
    println!("Normal Result R0 = {:?}", r0_normal);

    println!("--- Test 2: Fault Injection Campaign ---");
    let target_addresses = [0x1000, 0x1004];

    for target in target_addresses {
        println!("Testing Skip on address {:#x}:", target);
        println!("Enter R1 value:");
        let r0_fault = run_simulation(Some(target));
        
        if r0_fault == 1 {
            println!("==> [VULNERABLE] Address {:#x} allowed BYPASS!\n", target);
        } else {
            println!("==> [SAFE] Address {:#x} kept system secure.\n", target);
        }
    }
}