//! Simple code with tests that shows an issue I'm having with deserializing open-ended fields via flatten.
//! Format of test.xls is:
//!
//!     name   other1   other2  other3
//!     Dan    one      two     three
//!
//! Just one header row and one normal row. My row() test below fails, and I don't beleive it should.

fn main() {}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    struct Row {
        name: String,

        #[serde(flatten)]
        other: HashMap<String, String>,
    }

    #[derive(Deserialize)]
    struct ExplicitRow {
        name: String,
        other1: String,
        other2: String,
        other3: String,
    }

    use calamine::{RangeDeserializerBuilder, Reader, Xls, open_workbook};

    #[test]
    fn row() {
        let mut workbook = open_workbook::<Xls<_>, _>("test.xls").unwrap();
        let range = workbook.worksheet_range("Sheet1").unwrap();
        let range_iter: calamine::RangeDeserializer<'_, _, Row> =
            RangeDeserializerBuilder::with_deserialize_headers::<Row>()
                .from_range(&range)
                .unwrap();

        let rows: Result<Vec<_>, _> = range_iter.collect();
        rows.unwrap();
    }

    #[test]
    fn explicit_row() {
        let mut workbook = open_workbook::<Xls<_>, _>("test.xls").unwrap();
        let range = workbook.worksheet_range("Sheet1").unwrap();
        let range_iter: calamine::RangeDeserializer<'_, _, ExplicitRow> =
            RangeDeserializerBuilder::with_deserialize_headers::<ExplicitRow>()
                .from_range(&range)
                .unwrap();

        let rows: Result<Vec<_>, _> = range_iter.collect();
        rows.unwrap();
    }
}
