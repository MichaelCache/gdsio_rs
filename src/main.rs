mod gdsio;
use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();
    let file_path = match args.next(){
        Some(arg) => arg,
        None =>{
            eprintln!("please input with gds file");
            process::exit(1);
        },
    };
    // let root_dir = env!("CARGO_MANIFEST_DIR");
    let r = gdsio::read_gdsii(&file_path).unwrap_or_else(|err| {
        eprintln!("can not read file {}: {}", file_path, err);
        process::exit(1);
    });

    // println!("{:#?}", r);

    let model = gdsio::parse_gds(&r).unwrap_or_else(|err|{
        eprintln!("can not parse file {}: {}", file_path, err);
        process::exit(1);
    });

    println!("{:#?}", model);

    
}
