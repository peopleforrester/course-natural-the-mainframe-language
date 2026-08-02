// ABOUTME: The sample dataset the course teaches against: DDM definitions plus real
// ABOUTME: fixture rows, rebuilt fresh for every program run so lessons stay repeatable.

use crate::value::{Format, Value};
use rust_decimal::Decimal;
use std::str::FromStr;

/// The EMPLOYEES fixture, embedded rather than read from disk so the same code works in
/// the browser, where there is no filesystem.
const EMPLOYEES_CSV: &str = include_str!("../data/employees.csv");

/// One field of a data definition module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DdmField {
    pub name: String,
    pub format: Format,
}

/// A data definition module: the bridge between a program and a database file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ddm {
    pub name: String,
    pub fields: Vec<DdmField>,
}

impl Ddm {
    pub fn field(&self, name: &str) -> Option<&DdmField> {
        self.fields.iter().find(|f| f.name == name)
    }

    fn position(&self, name: &str) -> Option<usize> {
        self.fields.iter().position(|f| f.name == name)
    }
}

/// One stored record, positional against its DDM's field list.
///
/// Deletion tombstones rather than removing, so that a cursor resolved before the delete
/// keeps pointing at the records it resolved. Removing from the middle of the vector would
/// silently shift every later index out from under an active loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub values: Vec<Value>,
    pub deleted: bool,
}

/// The sample database a program reads and writes.
///
/// A fresh instance is built for every [`crate::Interpreter`], which is what makes a lesson
/// repeatable: a learner who runs a STORE exercise twice sees the same result both times.
/// That is a product requirement, not an implementation convenience.
#[derive(Debug, Clone)]
pub struct Database {
    ddm: Ddm,
    records: Vec<Record>,
}

impl Default for Database {
    fn default() -> Self {
        Self::sample()
    }
}

impl Database {
    /// Builds the EMPLOYEES file from the embedded fixture.
    pub fn sample() -> Self {
        let ddm = Ddm {
            name: "EMPLOYEES".to_string(),
            fields: vec![
                field("PERSONNEL-ID", Format::Alpha { length: 8 }),
                field("NAME", Format::Alpha { length: 20 }),
                field("FIRST-NAME", Format::Alpha { length: 20 }),
                field("CITY", Format::Alpha { length: 20 }),
                field("COUNTRY", Format::Alpha { length: 3 }),
                field("DEPT", Format::Alpha { length: 6 }),
                field("JOB-TITLE", Format::Alpha { length: 25 }),
                field(
                    "SALARY",
                    Format::Packed {
                        int_digits: 9,
                        decimals: 0,
                    },
                ),
            ],
        };

        let mut lines = EMPLOYEES_CSV.lines().filter(|l| !l.trim().is_empty());
        let header = lines.next().unwrap_or_default();
        debug_assert_eq!(
            header.split(',').count(),
            ddm.fields.len(),
            "the fixture header must match the DDM field list"
        );

        let records = lines
            .map(|line| {
                let cells: Vec<&str> = line.split(',').collect();
                let values = ddm
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let cell = cells.get(i).copied().unwrap_or_default().trim();
                        match f.format {
                            Format::Alpha { .. } => Value::Alpha(cell.to_string()),
                            Format::Logical => Value::Logical(cell.eq_ignore_ascii_case("TRUE")),
                            _ => Value::Number(Decimal::from_str(cell).unwrap_or(Decimal::ZERO)),
                        }
                    })
                    .collect();
                Record {
                    values,
                    deleted: false,
                }
            })
            .collect();

        Self { ddm, records }
    }

    pub fn ddm(&self, name: &str) -> Option<&Ddm> {
        (self.ddm.name == name).then_some(&self.ddm)
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    /// Reads one field of one record.
    pub fn value(&self, index: usize, field: &str) -> Option<Value> {
        let position = self.ddm.position(field)?;
        self.records.get(index)?.values.get(position).cloned()
    }

    /// How many records the file holds, ignoring deleted ones.
    pub fn len(&self) -> usize {
        self.records.iter().filter(|r| !r.deleted).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The index of the first live record whose field matches, by text comparison.
    pub fn find(&self, field: &str, wanted: &str) -> Option<usize> {
        let position = self.ddm.position(field)?;
        self.records.iter().enumerate().find_map(|(i, record)| {
            if record.deleted {
                return None;
            }
            let matches = match record.values.get(position) {
                Some(Value::Alpha(text)) => text.trim_end() == wanted,
                Some(Value::Number(n)) => n.to_string() == wanted,
                Some(Value::Logical(b)) => b.to_string().eq_ignore_ascii_case(wanted),
                None => false,
            };
            matches.then_some(i)
        })
    }

    /// Appends a record built from the values a view buffer currently holds.
    pub fn store(&mut self, buffer: &[(String, Value)]) {
        let values = self
            .ddm
            .fields
            .iter()
            .map(|f| {
                buffer
                    .iter()
                    .find(|(name, _)| name == &f.name)
                    .map(|(_, value)| value.clone())
                    .unwrap_or_else(|| f.format.default_value())
            })
            .collect();
        self.records.push(Record {
            values,
            deleted: false,
        });
    }

    /// Writes the view buffer back over an existing record, leaving untouched fields alone.
    pub fn update(&mut self, index: usize, buffer: &[(String, Value)]) {
        for (name, value) in buffer {
            if let Some(position) = self.ddm.position(name)
                && let Some(record) = self.records.get_mut(index)
                && let Some(slot) = record.values.get_mut(position)
            {
                *slot = value.clone();
            }
        }
    }

    pub fn delete(&mut self, index: usize) {
        if let Some(record) = self.records.get_mut(index) {
            record.deleted = true;
        }
    }

    /// The record order a READ produces, optionally sorted by a descriptor.
    ///
    /// Without a BY clause the records come back in stored order, which is what Natural
    /// calls physical sequence.
    pub fn order(&self, by: Option<&str>) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.records.len())
            .filter(|i| !self.records[*i].deleted)
            .collect();
        if let Some(descriptor) = by
            && let Some(position) = self.ddm.position(descriptor)
        {
            indices.sort_by(|a, b| {
                sort_key(&self.records[*a].values[position])
                    .cmp(&sort_key(&self.records[*b].values[position]))
            });
        }
        indices
    }
}

/// Orders values so alphanumeric sorts by text and numeric sorts by magnitude.
fn sort_key(value: &Value) -> (u8, String, Decimal) {
    match value {
        Value::Alpha(text) => (0, text.trim_end().to_string(), Decimal::ZERO),
        Value::Number(n) => (1, String::new(), *n),
        Value::Logical(b) => (2, String::new(), Decimal::from(u8::from(*b))),
    }
}

fn field(name: &str, format: Format) -> DdmField {
    DdmField {
        name: name.to_string(),
        format,
    }
}
