use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek},
};

struct Emulator {
    rom: [u8; 65536],
    pc: u16,
    halt: bool,
}

impl Emulator {
    fn new() -> Emulator {
        Emulator {
            rom: [0; 65536],
            pc: 0,
            halt: false,
        }
    }
    fn main(&mut self) {
        while !self.halt {
            println!("{}", self.pc);
            let mut a: u16 = self.read(self.pc as usize);
            let mut b: u16 = self.read(self.pc as usize + 2);
            let mut c: u16 = self.read(self.pc as usize + 4);
            self.pc += 6;
            self.suble(a as usize, b as usize, c);
        }
    }
    // fn copy(&mut self, src: usize, dest: usize) {
    //     self.suble(dest, dest, self.pc);
    //     self.add(src, dest);
    // }
    // fn add(&mut self, a: usize, b: usize) {
    //     /// Not for use
    //     self.suble(a, 65500, self.pc);
    //     self.suble(65500, b, self.pc);
    //     self.suble(65500, 65500, self.pc);
    // }
    // fn jump(&mut self, addr: u16) {
    //     self.suble(65501, 65501, addr);
    // }

    fn suble(&mut self, a: usize, b: usize, c: u16) {
        let mut mem_a: u16 = self.read(a);
        let mut mem_b: u16 = self.read(b);
        mem_b = mem_b.wrapping_add(mem_a.wrapping_neg());
        self.write(b, mem_b);
        if mem_b <= 0 {
            self.pc = c;
        } else {
            self.pc += 6;
        }
    }
    fn read(&mut self, addr: usize) -> u16 {
        let mem: u16 = (self.rom[addr] as u16) | (self.rom[addr + 1] as u16) << 8;
        return mem;
    }

    fn write(&mut self, addr: usize, value: u16) {
        if addr == 65510 {
            println!("{}", value);
            self.halt = true;
        } else {
            self.rom[addr] = (value & 0xff) as u8;
            self.rom[addr + 1] = (value >> 8) as u8;
            self.halt = false;
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
    println!("");
    emulator.main();
    // let a: u16 = rom[pc] | rom[pc + 1] << 8;
    // rom[pc + 3] = a & 0xff;
    // rom[pc + 4] = (a >> 8) & 0xff;
}
