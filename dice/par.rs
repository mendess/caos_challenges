extern crate memmap;
extern crate rand;

use std::fs::File;
use memmap::MmapOptions;
use rand::Rng;
use std::time::Instant;
use std::thread;

fn do_it(line :&[u8]) {
    if line.len() < 1 { return; }
    let mut rolls :u64 = 0;
    let mut faces :u64 = 0;
    let mut pre_d = true;
    for c in line {
        if *c == b'd' {
            pre_d = !pre_d;
            continue;
        }
        if pre_d {
            rolls = (rolls * 10) + (*c - b'0') as u64;
        }else{
            faces = (faces * 10) + (*c - b'0') as u64;
        }
    }
    let mut sum :u64 = 0;
    let mut rng = rand::thread_rng();
    for _i in 0..rolls {
        sum += 1 + rng.gen::<u64>() % faces;
    }
    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();
    let file = File::open("input.txt")?;
    let memmap = unsafe { MmapOptions::new().map(&file)? };

    let mid_time = Instant::now();

    // ============== do ==================
    let chunk_size = 2_usize.pow(19);
    let mut threads = Vec::with_capacity(memmap.len() / chunk_size);
    let mut index :usize = chunk_size;
    let mut last_index :usize = 0;
    while index < memmap.len() {
        while memmap[index] != b'\n' { index += 1; }
        let slice: &'static [u8] = unsafe { &*(&memmap[last_index..index] as *const _) };
        let runnable = move || {
            slice
                .split(|c| *c == b'\n')
                .for_each(|line| do_it(line))
        };
        threads.push(thread::spawn(runnable));
        last_index = index;
        index = index + chunk_size;
    }
    memmap[last_index..]
        .split(|c| *c == b'\n')
        .for_each(|line| do_it(line));
    for t in threads { t.join().unwrap(); }
    // ============ done ================
    let end_time = Instant::now();
    eprintln!("Loading file:  {:?}", mid_time.duration_since(start_time));
    eprintln!("Doing it:      {:?}", end_time.duration_since(mid_time));
    Ok(())
}
