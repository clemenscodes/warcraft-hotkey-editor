//! Super basic SYLK file parser.
//!
//! See <https://en.wikipedia.org/wiki/Symbolic_Link_(SYLK)> for details.
use std::collections::BTreeMap;

type CellValue = String;
type Columns = BTreeMap<ColumnKey, CellValue>;
type ColumnIndex = BTreeMap<CellValue, ColumnKey>;
type Row = BTreeMap<ColumnKey, CellValue>;
type Rows = BTreeMap<RowKey, Row>;

#[derive(Debug, Default)]
pub struct SlkTable {
    columns: Columns,
    column_index: ColumnIndex,
    rows: Rows,
}

impl SlkTable {
    pub fn new(columns: Columns, rows: Rows) -> Self {
        let mut column_index = ColumnIndex::new();
        for (column_key, column_name) in &columns {
            column_index.insert(column_name.clone(), *column_key);
        }

        Self {
            columns,
            column_index,
            rows,
        }
    }

    pub fn columns(&self) -> &Columns {
        &self.columns
    }

    pub fn rows(&self) -> &Rows {
        &self.rows
    }

    pub fn row(&self, row_key: RowKey) -> Option<&Row> {
        self.rows.get(&row_key)
    }

    pub fn column_name(&self, column_key: ColumnKey) -> Option<&str> {
        self.columns.get(&column_key).map(String::as_str)
    }

    pub fn column_key(&self, column_name: &str) -> Option<ColumnKey> {
        self.column_index.get(column_name).copied()
    }

    pub fn get_by_index(&self, row_key: RowKey, column_key: ColumnKey) -> Option<&str> {
        self.rows
            .get(&row_key)?
            .get(&column_key)
            .map(String::as_str)
    }

    pub fn get(&self, row_key: RowKey, column_name: &str) -> Option<&str> {
        let column_key = self.column_key(column_name)?;
        self.get_by_index(row_key, column_key)
    }

    fn parse_cell_value(raw: &str) -> CellValue {
        if raw.starts_with('"') && raw.ends_with('"') {
            raw[1..raw.len() - 1].to_string()
        } else {
            raw.to_string()
        }
    }
}

impl From<&str> for SlkTable {
    fn from(input: &str) -> Self {
        let mut columns = Columns::new();
        let mut rows = Rows::new();

        let mut current_column: Option<ColumnKey> = None;
        let mut current_row: Option<RowKey> = None;

        for line in input.lines() {
            let line = line.trim();

            if line == "E" {
                break;
            }

            if !line.starts_with("C;") {
                continue;
            }

            let mut parsed_column: Option<ColumnKey> = None;
            let mut parsed_row: Option<RowKey> = None;
            let mut parsed_value: Option<CellValue> = None;

            for field in line.split(';') {
                match field.as_bytes() {
                    [b'X', ..] => {
                        parsed_column = field[1..].parse::<usize>().ok().map(ColumnKey::from)
                    }
                    [b'Y', ..] => parsed_row = field[1..].parse::<usize>().ok().map(RowKey::from),
                    [b'K', ..] => parsed_value = Some(Self::parse_cell_value(&field[1..])),
                    _ => {}
                }
            }

            if let Some(column_key) = parsed_column {
                current_column = Some(column_key);
            }
            if let Some(row_key) = parsed_row {
                current_row = Some(row_key);
            }

            let Some(column_key) = current_column else {
                continue;
            };
            let Some(row_key) = current_row else {
                continue;
            };
            let Some(cell_value) = parsed_value else {
                continue;
            };

            if row_key == 1 {
                columns.insert(column_key, cell_value);
            } else {
                rows.entry(row_key)
                    .or_default()
                    .insert(column_key, cell_value);
            }
        }

        Self::new(columns, rows)
    }
}

impl From<String> for SlkTable {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl Extend<SlkTable> for SlkTable {
    fn extend<Source: IntoIterator<Item = SlkTable>>(&mut self, source: Source) {
        for other in source {
            self.columns.extend(other.columns);
            self.column_index.extend(other.column_index);
            self.rows.extend(other.rows);
        }
    }
}

impl std::ops::Index<RowKey> for SlkTable {
    type Output = Row;

    fn index(&self, row_key: RowKey) -> &Self::Output {
        &self.rows[&row_key]
    }
}

impl<'a> IntoIterator for &'a SlkTable {
    type Item = RowView<'a>;
    type IntoIter = RowIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RowIter {
            table: self,
            inner: self.rows.iter(),
        }
    }
}

impl std::fmt::Display for SlkTable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row_key, row) in &self.rows {
            writeln!(formatter, "[{row_key}]")?;

            for (column_key, cell_value) in row {
                if let Some(column_name) =
                    self.columns.get(column_key).filter(|name| !name.is_empty())
                {
                    writeln!(formatter, "{column_name} = {cell_value}")?;
                }
            }

            writeln!(formatter)?;
        }

        Ok(())
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnKey {
    index: usize,
}

impl ColumnKey {
    pub fn index(&self) -> usize {
        self.index
    }
}

impl PartialEq<usize> for ColumnKey {
    fn eq(&self, other: &usize) -> bool {
        self.index == *other
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowKey {
    index: usize,
}

impl RowKey {
    pub fn index(&self) -> usize {
        self.index
    }
}

impl PartialEq<usize> for RowKey {
    fn eq(&self, other: &usize) -> bool {
        self.index == *other
    }
}

impl From<usize> for ColumnKey {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl From<ColumnKey> for usize {
    fn from(key: ColumnKey) -> Self {
        key.index
    }
}

impl From<usize> for RowKey {
    fn from(index: usize) -> Self {
        Self { index }
    }
}

impl From<RowKey> for usize {
    fn from(key: RowKey) -> Self {
        key.index
    }
}

impl std::fmt::Display for RowKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.index)
    }
}

impl std::fmt::Display for ColumnKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.index)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RowView<'a> {
    table: &'a SlkTable,
    row_key: RowKey,
    row: &'a Row,
}

impl<'a> RowView<'a> {
    pub fn key(&self) -> RowKey {
        self.row_key
    }

    pub fn get(&self, column_name: &str) -> Option<&'a str> {
        let column_key = self.table.column_key(column_name)?;
        self.row.get(&column_key).map(String::as_str)
    }

    pub fn get_by_index(&self, column_key: ColumnKey) -> Option<&'a str> {
        self.row.get(&column_key).map(String::as_str)
    }
}

pub struct RowIter<'a> {
    table: &'a SlkTable,
    inner: std::collections::btree_map::Iter<'a, RowKey, Row>,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = RowView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (row_key, row) = self.inner.next()?;
        let row_view = RowView {
            table: self.table,
            row_key: *row_key,
            row,
        };
        Some(row_view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SlkTableBuilder {
        column_names: Vec<String>,
        data_rows: Vec<Vec<String>>,
    }

    impl SlkTableBuilder {
        fn new() -> Self {
            Self {
                column_names: Vec::new(),
                data_rows: Vec::new(),
            }
        }

        fn column(mut self, column_name: &str) -> Self {
            self.column_names.push(column_name.to_string());
            self
        }

        fn row(mut self, cell_values: impl IntoIterator<Item = &'static str>) -> Self {
            let values = cell_values.into_iter().map(str::to_string).collect();
            self.data_rows.push(values);
            self
        }

        fn build(self) -> SlkTable {
            let mut columns = Columns::new();
            for (column_position, column_name) in self.column_names.into_iter().enumerate() {
                let column_key = ColumnKey::from(column_position + 1);
                columns.insert(column_key, column_name);
            }

            let mut rows = Rows::new();
            for (row_position, row_values) in self.data_rows.into_iter().enumerate() {
                let row_key = RowKey::from(row_position + 2);
                let mut row = Row::new();
                for (cell_position, cell_value) in row_values.into_iter().enumerate() {
                    let column_key = ColumnKey::from(cell_position + 1);
                    row.insert(column_key, cell_value);
                }
                rows.insert(row_key, row);
            }

            SlkTable::new(columns, rows)
        }
    }

    #[test]
    fn parses_empty_input_as_empty_table() {
        let table = SlkTable::from("");
        assert_eq!(table.columns().len(), 0);
        assert_eq!(table.rows().len(), 0);
    }

    #[test]
    fn ignores_input_without_c_records() {
        let input = "ID;P\nB;Y1;X2\nF;P;FG0G\nE\n";
        let table = SlkTable::from(input);
        assert_eq!(table.columns().len(), 0);
        assert_eq!(table.rows().len(), 0);
    }

    #[test]
    fn parses_single_row_table() {
        let input = "ID;P
C;X1;Y1;K\"unitID\"
C;X2;Y1;K\"name\"
C;X1;Y2;K\"hpea\"
C;X2;Y2;K\"Peasant\"
E
";
        let table = SlkTable::from(input);
        assert_eq!(table.columns().len(), 2);
        assert_eq!(table.rows().len(), 1);
        let data_row_key = RowKey::from(2);
        assert_eq!(table.get(data_row_key, "unitID"), Some("hpea"));
        assert_eq!(table.get(data_row_key, "name"), Some("Peasant"));
    }

    #[test]
    fn quoted_values_are_stripped() {
        let input = "ID;P\nC;X1;Y1;K\"col\"\nC;X1;Y2;K\"value\"\nE\n";
        let table = SlkTable::from(input);
        let data_row_key = RowKey::from(2);
        assert_eq!(table.get(data_row_key, "col"), Some("value"));
    }

    #[test]
    fn unquoted_values_are_kept_as_is() {
        let input = "ID;P\nC;X1;Y1;K\"col\"\nC;X1;Y2;K42\nE\n";
        let table = SlkTable::from(input);
        let data_row_key = RowKey::from(2);
        assert_eq!(table.get(data_row_key, "col"), Some("42"));
    }

    #[test]
    fn e_terminator_stops_parsing() {
        let input = "ID;P\nC;X1;Y1;K\"col\"\nC;X1;Y2;K\"keep\"\nE\nC;X1;Y3;K\"ignore\"\n";
        let table = SlkTable::from(input);
        let kept_row_key = RowKey::from(2);
        let dropped_row_key = RowKey::from(3);
        assert_eq!(table.get(kept_row_key, "col"), Some("keep"));
        assert_eq!(table.get(dropped_row_key, "col"), None);
    }

    #[test]
    fn implicit_column_uses_previous_x() {
        // Subsequent C records without ;X reuse the previous column coordinate.
        let input = "ID;P
C;X1;Y1;K\"col\"
C;X1;Y2;K\"first\"
C;Y3;K\"second\"
E
";
        let table = SlkTable::from(input);
        let first_row_key = RowKey::from(2);
        let second_row_key = RowKey::from(3);
        assert_eq!(table.get(first_row_key, "col"), Some("first"));
        assert_eq!(table.get(second_row_key, "col"), Some("second"));
    }

    #[test]
    fn implicit_row_uses_previous_y() {
        // Subsequent C records without ;Y reuse the previous row coordinate.
        let input = "ID;P
C;X1;Y1;K\"a\"
C;X2;Y1;K\"b\"
C;X1;Y2;K\"value-a\"
C;X2;K\"value-b\"
E
";
        let table = SlkTable::from(input);
        let data_row_key = RowKey::from(2);
        assert_eq!(table.get(data_row_key, "a"), Some("value-a"));
        assert_eq!(table.get(data_row_key, "b"), Some("value-b"));
    }

    #[test]
    fn first_record_with_malformed_coordinates_is_skipped() {
        // Neither X nor Y parses, and there's no previous record to inherit
        // coordinates from — the record must not be committed to the table.
        let input = "ID;P\nC;Xabc;Yzzz;K\"ignored\"\nE\n";
        let table = SlkTable::from(input);
        assert_eq!(table.columns().len(), 0);
        assert_eq!(table.rows().len(), 0);
    }

    #[test]
    fn subsequent_malformed_x_inherits_previous_column() {
        // Stickiness: after a valid X;Y record, a record with an invalid X
        // (but a valid Y) reuses the previous column coordinate.
        let input = "ID;P
C;X1;Y1;K\"col\"
C;X1;Y2;K\"first\"
C;Xbad;Y3;K\"second\"
E
";
        let table = SlkTable::from(input);
        assert_eq!(table.get(RowKey::from(2), "col"), Some("first"));
        assert_eq!(table.get(RowKey::from(3), "col"), Some("second"));
    }

    #[test]
    fn column_key_and_name_round_trip() {
        let table = SlkTableBuilder::new()
            .column("first")
            .column("second")
            .build();
        let first_key = table.column_key("first").unwrap();
        let second_key = table.column_key("second").unwrap();
        assert_eq!(table.column_name(first_key), Some("first"));
        assert_eq!(table.column_name(second_key), Some("second"));
        assert_ne!(first_key, second_key);
    }

    #[test]
    fn get_returns_none_for_missing_column() {
        let table = SlkTableBuilder::new().column("col").row(["value"]).build();
        let data_row_key = RowKey::from(2);
        assert_eq!(table.get(data_row_key, "unknown"), None);
    }

    #[test]
    fn get_returns_none_for_missing_row() {
        let table = SlkTableBuilder::new().column("col").row(["value"]).build();
        let missing_row_key = RowKey::from(999);
        assert_eq!(table.get(missing_row_key, "col"), None);
    }

    #[test]
    fn row_view_get_works() {
        let table = SlkTableBuilder::new()
            .column("a")
            .column("b")
            .row(["one", "two"])
            .build();
        let row_view = table.into_iter().next().unwrap();
        assert_eq!(row_view.key(), RowKey::from(2));
        assert_eq!(row_view.get("a"), Some("one"));
        assert_eq!(row_view.get("b"), Some("two"));
        assert_eq!(row_view.get("c"), None);
    }

    #[test]
    fn row_view_get_by_index_works() {
        let table = SlkTableBuilder::new().column("col").row(["value"]).build();
        let row_view = table.into_iter().next().unwrap();
        assert_eq!(row_view.get_by_index(ColumnKey::from(1)), Some("value"));
        assert_eq!(row_view.get_by_index(ColumnKey::from(99)), None);
    }

    #[test]
    fn into_iterator_yields_rows_in_key_order() {
        let input = "ID;P
C;X1;Y1;K\"col\"
C;X1;Y3;K\"third\"
C;X1;Y2;K\"second\"
E
";
        let table = SlkTable::from(input);
        let row_keys: Vec<RowKey> = table.into_iter().map(|view| view.key()).collect();
        assert_eq!(row_keys, vec![RowKey::from(2), RowKey::from(3)]);
    }

    #[test]
    fn extend_merges_columns_and_rows() {
        let left_input = "ID;P\nC;X1;Y1;K\"a\"\nC;X1;Y2;K\"a1\"\nE\n";
        let right_input = "ID;P\nC;X2;Y1;K\"b\"\nC;X2;Y3;K\"b3\"\nE\n";
        let mut left_table = SlkTable::from(left_input);
        let right_table = SlkTable::from(right_input);
        left_table.extend([right_table]);

        assert!(left_table.column_key("a").is_some());
        assert!(left_table.column_key("b").is_some());
        assert_eq!(left_table.get(RowKey::from(2), "a"), Some("a1"));
        assert_eq!(left_table.get(RowKey::from(3), "b"), Some("b3"));
    }

    #[test]
    fn index_operator_returns_row() {
        let table = SlkTableBuilder::new().column("col").row(["value"]).build();
        let row = &table[RowKey::from(2)];
        assert_eq!(
            row.get(&ColumnKey::from(1)).map(String::as_str),
            Some("value")
        );
    }

    #[test]
    fn display_renders_row_header_and_cells() {
        let table = SlkTableBuilder::new().column("col").row(["value"]).build();
        let rendered = format!("{table}");
        assert!(rendered.contains("[2]"));
        assert!(rendered.contains("col = value"));
    }

    #[test]
    fn display_skips_unnamed_columns() {
        // Build a table where column 2 has no column-name cell at Y=1.
        let input = "ID;P\nC;X1;Y1;K\"named\"\nC;X1;Y2;K\"A\"\nC;X2;Y2;K\"B\"\nE\n";
        let table = SlkTable::from(input);
        let rendered = format!("{table}");
        assert!(rendered.contains("named = A"));
        // The unnamed X2 cell should not appear — no `= B` line.
        assert!(!rendered.contains("= B"));
    }

    #[test]
    fn column_key_and_row_key_equal_usize() {
        let column_key = ColumnKey::from(5);
        let row_key = RowKey::from(7);
        assert_eq!(column_key, 5);
        assert_eq!(row_key, 7);
    }

    #[test]
    fn column_key_and_row_key_usize_round_trip() {
        let column_key = ColumnKey::from(3);
        let row_key = RowKey::from(9);
        let column_index: usize = column_key.into();
        let row_index: usize = row_key.into();
        assert_eq!(column_index, 3);
        assert_eq!(row_index, 9);
    }

    #[test]
    fn from_string_delegates_to_from_str() {
        let input = "ID;P\nC;X1;Y1;K\"col\"\nC;X1;Y2;K\"value\"\nE\n".to_string();
        let table = SlkTable::from(input);
        assert_eq!(table.get(RowKey::from(2), "col"), Some("value"));
    }
}
