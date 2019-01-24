use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};
use std::mem;

const HASH_BLK_SIZE: u64 = 65536;

fn create_hash(file: File, fsize: u64) -> Result<String, std::io::Error> {
    let mut buf = [0u8; 8];
    let mut word: u64;
    let mut hash_val: u64 = fsize;  // seed hash with file size
    let iterations = HASH_BLK_SIZE /  8;
    let mut reader = BufReader::with_capacity(HASH_BLK_SIZE as usize, file);
    for _ in 0..iterations {
        try!(reader.read(&mut buf));
        unsafe { word = mem::transmute(buf); };
        hash_val = hash_val.wrapping_add(word);
    }
    try!(reader.seek(SeekFrom::Start(fsize - HASH_BLK_SIZE)));
    for _ in 0..iterations {
        try!(reader.read(&mut buf));
        unsafe { word = mem::transmute( buf); };
        hash_val = hash_val.wrapping_add(word);
    }
    let hash_string = format!("{:01$x}", hash_val, 16);
    Ok(hash_string)
}

fn main_exec() {
    let fname = "aa.mkv";
    let fsize = fs::metadata(fname).unwrap().len();
    if fsize>HASH_BLK_SIZE {
        let file = File::open(fname).unwrap();
        let fhash = create_hash(file, fsize).unwrap();
        println!("Hash for {} is  {}", fname, fhash);
    }
}
