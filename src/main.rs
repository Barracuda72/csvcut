use std::io;
use std::collections::HashSet;

fn main() {
    let mut columns: HashSet<usize> = std::env::args().skip(1).map(|x| x.parse().unwrap()).collect();

    if columns.is_empty() {
        columns.insert(0);
    }

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true) // Number of columns can be different each line
        .has_headers(false) // We're not interested in treating first line as header
        .from_reader(io::stdin());
    let mut wtr = csv::WriterBuilder::new()
        .flexible(true) // Number of columns can be different each line
        .has_headers(false) // We're not interested in treating first line as header
        .from_writer(io::stdout());

    let mut errors = 0;

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = match result {
            Ok(record) => record,
            Err(e) => { eprintln!("Malformed CSV: {}!", e); errors += 1; continue; },
        };
        
        let record = record
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| columns.contains(&i))
            .map(|(_, e)| e);

        wtr.write_record(record).expect("Failed to write data!");
    }

    eprintln!("Total errors: {}", errors);
}
