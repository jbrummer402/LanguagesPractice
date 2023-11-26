use std::{error::Error, env, io, process, File, Record};

fn read_data() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect()?;
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        // match record.get(0) {
        //     Some(c) => println!("{}", c),
        //     None => println!("None"),
        // }
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_data() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
