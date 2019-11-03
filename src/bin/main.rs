extern crate serde_derive;
extern crate clap;
extern crate toml;

use clap::{App, Arg};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::error::Error;
use csv::StringRecord;

#[derive(Deserialize)]
struct Config {
    input: Input,
    output: Output
}

#[derive(Deserialize)]
struct Input {
    delimiter: String,
    fields: Vec<Field>,
}

#[derive(Deserialize)]
struct Output {
    delimiter: String,
    with_header: bool,
    fields: Vec<OutputField>
}

#[derive(Deserialize, Debug, Clone)]
struct Field {
    name: String,
    column_type: ColumnType,
    pipelines: Vec<Pipeline>
}

#[derive(Deserialize, Debug, Clone)]
struct OutputField {
    source: String,
    renamed: Option<String>,
    pipelines: Vec<Pipeline>
}

#[derive(Deserialize, Debug, Clone)]
enum ColumnType {
    Id,
    Date,
    Value,
    Merchant,
    Category,
    Description
}

#[derive(Deserialize, Debug, Clone)]
enum Pipeline {
    InvertSignal,
    CommaToDot
}

#[derive(Debug)]
struct BankField {
    name: String,
    field: Field,
    value: String
}

#[derive(Debug)]
struct BankRecord {
    records: HashMap<String, BankField>
}

impl BankRecord {
    fn add_field(&mut self, field: BankField) -> () {
        self.records.insert(field.name.clone(), field);
    }

    fn new() -> BankRecord {
        return BankRecord { records: HashMap::new() }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("bank parser")
        .version("0.1")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("The TOML describing the input format and pipelines")
                .takes_value(true)
                .required(true)
        )
        .arg(Arg::with_name("INPUT").required(true).index(1))
        .get_matches();

    let input_path = matches.value_of("INPUT").unwrap();
    let input_pattern = matches.value_of("config").unwrap();


    let config: Config = read_input_toml(input_pattern).unwrap();

    let mapped_records = parse(&config, input_path)?;

    process_records(&config, mapped_records);

    Ok(())
}

fn read_input_toml<T>(path: &str) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
    {
    let input_pattern_contents = fs::read_to_string(path).unwrap();
    let ts: T = toml::from_str(&input_pattern_contents)?;
    Ok(ts)
}

fn parse(config: &Config, path: &str) -> Result<Vec<BankRecord>, Box<dyn Error>> {
   let mut rdr = csv::ReaderBuilder::new()
        .delimiter(config.input.delimiter.as_bytes()[0])
        .from_path(path)
        .unwrap();

    let record_mapper = |record: StringRecord| {
        let mut bank_r: BankRecord = BankRecord::new();
        for (i, field) in config.input.fields.iter().enumerate() {
            bank_r.add_field(BankField {name: field.name.clone(),
                                        field: field.clone(),
                                        value: String::from(record.get(i).unwrap())})

        }

        bank_r
    };

    let mapped_records: Vec<BankRecord> = rdr.records()
    .filter_map(Result::ok)
    .map(record_mapper)
    .collect();

    Ok(mapped_records)
}

fn process_records(config: &Config, records: Vec<BankRecord>) -> Result<(), Box<dyn Error>> {
    let mut tempf = tempfile::NamedTempFile::new()?;

    let path = tempf.path();
    print!("writing to {:?}", path);

    let mut writer = csv::WriterBuilder::new()
    .has_headers(config.output.with_header)
    .delimiter(config.output.delimiter.as_bytes()[0])
    .from_path(path)?;

    if config.output.with_header {
        let field_names: Vec<String> = config.output.fields.iter().map(|f| {f.renamed.clone().unwrap_or(f.source.clone())}).collect();
        writer.write_record(&field_names)?;
    }

    for record in records {
        let vals: Vec<String> = config.output.fields.iter().map(|output| {
            return record.records.get(&output.source).unwrap().value.clone();
        }).collect();
        writer.write_record(vals.as_slice())?;
    }

    writer.flush()?;
    tempf.keep();

    Ok(())
}
