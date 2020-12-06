use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::{BufWriter,Write};
use rand::Rng;

fn main() {
    let number: u32;
    let size: u32;
    let output: String;

    let app = App::new("make_binry")
        .version("1.0.0")
        .author("gedatsu")
        .about("make binary files")
        .arg(Arg::with_name("number")
            .value_name("number")
            .index(1)
            .required(true)
        )
        .arg(Arg::with_name("size")
            .value_name("size")
            .index(2)
            .required(true)
        )
        .arg(Arg::with_name("output_file")
            .value_name("OUTPUT FILE")
            .index(3)
            .required(true)
        )
        .get_matches();

    number = app.value_of("number").unwrap().parse().unwrap();
    output = app.value_of("output_file").unwrap().to_string();
    run(number, output);

}

fn run(number: u32, output: String) {
    //let f = File::open(output).unwrap();
    let mut f = BufWriter::new(File::create(output).unwrap());
    let mut rng = rand::thread_rng();
    for _ in 0..number {
        let x: f32 = rng.gen();
        f.write(&x.to_ne_bytes()).unwrap();
    }
    

}
