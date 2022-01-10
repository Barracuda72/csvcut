use std::io;

fn main() {
    let mut columns: Vec<i32> = std::env::args().skip(1).map(|x| x.parse().unwrap()).collect();

    if columns.len() < 1 {
        columns.push(0);
    }

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let mut errors = 0;

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = match result {
            Ok(record) => record,
            Err(e) => { eprintln!("Malformed CSV: {}!", e); errors += 1; continue; },
        };
        wtr.write_record(&record).expect("Failed to write data!");
    }

    eprintln!("Total errors: {}", errors);
}
