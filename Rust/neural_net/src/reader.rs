use std::{error::Error, io, process};

fn read() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}