mod gds_error;
mod gds_model;
mod gds_reader;
mod gds_record;
pub use gds_record::Record;
pub use gds_model::*;

pub use gds_model::parse_gds;

use std::fs::read;
use std::path::Path;

pub fn read_gdsii<T: AsRef<Path>>(gds_file: T) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let buff = read(gds_file)?;
    let byte_len = buff.len();
    if byte_len < 4usize {
        return Result::Err(Box::new(gds_error::gds_err("not valid gds file")));
    }
    
    if let gds_record::HEADER = &buff[2..4] {
    } else {
        return Result::Err(Box::new(gds_error::gds_err("not valid gds file")));
    }

    let mut idx: usize = 0;
    let mut record_len: usize = 0;
    let mut records: Vec<Record> = Vec::new();
    while idx < byte_len {
        record_len = u16::from_be_bytes(buff[idx..idx + 2].try_into().unwrap()) as usize;
        if record_len == 0 {
            break;
        }
        let r = gds_reader::record_type(&buff[idx..idx + record_len])
            .expect(format!("parse error at {:#08x}", idx).as_str());
        records.push(r);

        if let Record::EndLib = records.last().unwrap() {
            break;
        }

        idx += record_len;
    }

    if records.len() == 0 {
        // TODO:
        // return costum error
        // return gds_error::GDSIIError::GeneralError("sds");
        return Result::Err(Box::new(gds_error::gds_err("not valid gds file")));
    }
    Ok(records)
}
