// use std::collections::HashSet;
use std::{env};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let mut set: HashSet<String> = HashSet::new();
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];

    let mut output_path: &String = &(input_path.to_owned() + ".save");
    if args.len()>2 {
        output_path =  &args[2];
    }
    let mut file = File::create(output_path)?;
    {
        let mut writter = BufWriter::new(&file);
        for result in BufReader::new(File::open(input_path)?).lines() {
            let i = result?;
            if !set.contains(&*i) {
                set.insert((&*i).to_string());
                writter.write_all(i.as_bytes()).expect("TODO: panic message");
                writter.write_all(b"\n").expect("TODO: panic message");
            }
        }
    }
    file.flush()
}
