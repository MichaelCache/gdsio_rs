use super::gds_error::gds_err;
use super::gds_record::Record;

use wasm_bindgen::prelude::*;
use wasm_bindgen;

#[derive(Default, Debug)]
pub struct Lib {
    name: String,
    units: f64, //in meter
    precision: f64,
    cells: Vec<Box<Cell>>,
}

impl Lib {}

#[derive(Default, Debug)]
pub struct Cell {
    name: String,
    polygons: Vec<Box<Polygon>>,
    paths: Vec<Box<Path>>,
    refs: Vec<Box<Ref>>,
}

#[derive(Default, Debug)]
pub struct Polygon {
    layer: i16,
    datatype: i16,
    pub points: Vec<(i32, i32)>,
}

#[derive(Default, Debug)]
pub struct Path {
    layer: i16,
    datatype: i16,
    width: i32,
    end_type: i16,
    pub points: Vec<(i32, i32)>,
}

#[derive(Default, Debug)]
pub struct Ref {
    ref_to: String,
    reflection_x: bool,
    abs_magnific: bool,
    abs_angel: bool,
    angle: f64, //measured in degrees and in the counterclockwise direction
    origin: (i32, i32),
}

pub fn parse_gds(records: &[Record]) -> Result<Box<Lib>, Box<dyn std::error::Error>> {
    let mut lib: Box<Lib> = Box::new(Lib::default());
    let rec_len = records.len();
    // let mut next_i: usize;
    let mut i = 0;
    while i < rec_len {
        match records[i] {
            Record::BgnLib(_) => (lib, i) = parse_lib(&records[i..]),
            Record::EndLib => break,
            _ => (),
        }
        i += 1;
    }

    Ok(lib)
}

fn parse_lib(records: &[Record]) -> (Box<Lib>, usize) {
    let mut lib = Box::new(Lib::default());
    let rec_len = records.len();
    let mut i = 0;
    while i < rec_len {
        match &records[i] {
            Record::LibName(s) => lib.name = s.to_string(),
            Record::Units {
                unit_in_meter,
                precision,
            } => {
                lib.units = precision / unit_in_meter;
                lib.precision = *precision;
            }
            Record::BgnStr(_) => {
                let (cell, end_i) = parse_cell(&records[i..]);
                lib.cells.push(cell);
                i += end_i;
            }
            Record::EndLib => {
                break;
            }
            _ => (),
        }
        i += 1;
    }

    (lib, i)
}

fn parse_cell(records: &[Record]) -> (Box<Cell>, usize) {
    let mut cell = Box::new(Cell::default());
    let rec_len = records.len();
    let mut i = 0;
    while i < rec_len {
        match &records[i] {
            Record::StrName(s) => cell.name = s.to_string(),
            Record::Boundary => {
                let (polygon, end_i) = parse_polygon(&records[i..]);
                i += end_i;
                cell.polygons.push(polygon);
            }
            Record::Path => {
                let (path, end_i) = parse_path(&records[i..]);
                i += end_i;
                cell.paths.push(path);
            }
            Record::StrRef => {
                let (sref, end_i) = parse_sref(&records[i..]);
                i += end_i;
                cell.refs.push(sref);
            }
            Record::EndStr => {
                break;
            }
            _ => (),
        }
        i += 1;
    }

    (cell, i)
}

fn parse_polygon(records: &[Record]) -> (Box<Polygon>, usize) {
    let mut polygon = Box::new(Polygon::default());
    let rec_len = records.len();
    let mut i = 0;
    while i < rec_len {
        match &records[i] {
            Record::Layer(l) => polygon.layer = *l,
            Record::DataType(d) => polygon.datatype = *d,
            Record::Points(points) => polygon.points = points.clone(),
            Record::EndElem => break,
            _ => (),
        }
        i += 1;
    }
    (polygon, i)
}

fn parse_path(records: &[Record]) -> (Box<Path>, usize) {
    let mut path = Box::new(Path::default());
    let rec_len = records.len();
    let mut i = 0;
    while i < rec_len {
        match &records[i] {
            Record::Layer(l) => path.layer = *l,
            Record::DataType(d) => path.datatype = *d,
            Record::Width(w) => path.width = *w,
            Record::PathType(t) => path.end_type = *t,
            Record::Points(points) => path.points = points.clone(),
            Record::EndElem => break,
            _ => (),
        }
        i += 1;
    }
    (path, i)
}

fn parse_sref(records: &[Record]) -> (Box<Ref>, usize) {
    let mut sref = Box::new(Ref::default());
    let rec_len = records.len();
    let mut i = 0;
    while i < rec_len {
        match &records[i] {
            Record::StrRefName(s) => sref.ref_to = s.to_string(),
            Record::RefTrans {
                reflection_x,
                absolute_magnification,
                absolute_angle,
            } => {
                sref.reflection_x = *reflection_x;
                sref.abs_magnific = *absolute_magnification;
                sref.abs_angel = *absolute_angle;
            }
            Record::Angle(a) => sref.angle = *a.first().unwrap(),
            Record::Points(points) => sref.origin = *points.first().unwrap(),
            Record::EndElem => break,
            _ => (),
        }
        i += 1;
    }
    (sref, i)
}
