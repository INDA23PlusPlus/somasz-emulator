use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek},
};

struct Emulator {
    rom: [u8; 65536],
    pc: u16,
}

impl Emulator {
    fn new() -> Emulator {
        Emulator {
            rom: [0; 65536],
            pc: 0,
        }
    }
    fn add(&mut self, a: usize, b: usize) {
        self.suble(a, 65500, self.pc);
        self.suble(65500, b, self.pc);
        self.suble(65500, 65500, self.pc);
    }
    fn jump(&mut self, addr: u16) {
        self.suble(65501, 65501, addr);
    }

    fn suble(&mut self, a: usize, b: usize, c: u16) {
        let mut mem_a: u16 = self.read(a);
        let mut mem_b: u16 = self.read(b);
        mem_b = mem_b.wrapping_add(mem_a.wrapping_neg());
        self.write(b, mem_b);
        if mem_b <= 0 {
            self.pc = c;
        }
    }
    fn read(&mut self, addr: usize) -> u16 {
        let mem: u16 = (self.rom[addr] as u16) | (self.rom[addr + 1] as u16) << 8;
        return mem;
    }

    fn write(&mut self, addr: usize, value: u16) -> bool {
        if addr == 4 {
            println!("{}", value);
            false
        } else {
            self.rom[addr] = (value & 0xff) as u8;
            self.rom[addr + 1] = (value >> 8) as u8;
            true
        }
    }
}

fn main() {
    let mut file = File::open("./bincode.bin").unwrap();
    let mut reader = BufReader::new(file);
    let mut emulator = Emulator::new();
    let mut buf = vec![0; 0];
    reader.read_to_end(&mut buf);
    for i in 0..buf.len() {
        emulator.rom[i] = buf[i];
    }
    for i in 0..10 {
        println!("{}", emulator.rom[i]);
    }
    let mut cond = true;
    while cond {
        emulator.add(emulator.pc as usize, emulator.pc as usize + 2);
        emulator.pc += 4;
        let val = emulator.read(2);
        cond = emulator.write(emulator.pc as usize, val);
    }

    // let a: u16 = rom[pc] | rom[pc + 1] << 8;
    // rom[pc + 3] = a & 0xff;
    // rom[pc + 4] = (a >> 8) & 0xff;
}
