use rayon::prelude::*;
use slint::{ModelRc, SharedString, VecModel};

use crate::SingleMainListModel;

pub(crate) type RowContent = (Vec<SharedString>, Vec<i32>);

#[derive(Debug, PartialEq)]
pub(crate) struct PlainRow {
    filled_header_row: Option<bool>,
    content: RowContent,
}

impl PlainRow {
    pub(crate) fn item(content: RowContent) -> Self {
        Self { filled_header_row: None, content }
    }

    pub(crate) fn header(content: RowContent) -> Self {
        Self {
            filled_header_row: Some(true),
            content,
        }
    }

    pub(crate) fn empty_header() -> Self {
        Self {
            filled_header_row: Some(false),
            content: (Vec::new(), Vec::new()),
        }
    }
}

pub(crate) fn flat_rows<T: Send>(entries: Vec<T>, prepare: impl Fn(T) -> RowContent + Sync + Send) -> Vec<PlainRow> {
    entries.into_par_iter().map(|entry| PlainRow::item(prepare(entry))).collect()
}

pub(crate) fn grouped_rows<T: Send>(groups: Vec<(Option<T>, Vec<T>)>, prepare: impl Fn(T) -> RowContent + Sync + Send) -> Vec<PlainRow> {
    let group_rows = |(reference, entries): (Option<T>, Vec<T>)| {
        let header = match reference {
            Some(reference) => PlainRow::header(prepare(reference)),
            None => PlainRow::empty_header(),
        };
        std::iter::once(header)
            .chain(entries.into_iter().map(|entry| PlainRow::item(prepare(entry))))
            .collect::<Vec<_>>()
    };
    groups.into_par_iter().flat_map_iter(group_rows).collect()
}

pub(crate) fn rows_into_model(rows: Vec<PlainRow>) -> ModelRc<SingleMainListModel> {
    let rows: Vec<SingleMainListModel> = rows
        .into_iter()
        .map(|row| SingleMainListModel {
            checked: false,
            header_row: row.filled_header_row.is_some(),
            filled_header_row: row.filled_header_row.unwrap_or(false),
            focused_row: false,
            val_str: ModelRc::new(VecModel::from(row.content.0)),
            val_int: ModelRc::new(VecModel::from(row.content.1)),
        })
        .collect();
    ModelRc::new(VecModel::from(rows))
}

#[cfg(test)]
mod tests {
    use slint::Model;

    use super::*;

    fn content(name: &str, value: i32) -> RowContent {
        (vec![name.into()], vec![value])
    }

    fn prepare(entry: (&'static str, i32)) -> RowContent {
        content(entry.0, entry.1)
    }

    fn read_row(model: &ModelRc<SingleMainListModel>, idx: usize) -> (bool, bool, Vec<String>, Vec<i32>) {
        let row = model.row_data(idx).unwrap();
        (
            row.header_row,
            row.filled_header_row,
            row.val_str.iter().map(|s| s.to_string()).collect(),
            row.val_int.iter().collect(),
        )
    }

    #[test]
    fn test_flat_rows_keep_order_for_large_input() {
        let entries: Vec<(&str, i32)> = (0..10_000).map(|i| ("a", i)).collect();
        let rows = flat_rows(entries, prepare);
        let expected: Vec<PlainRow> = (0..10_000).map(|i| PlainRow::item(content("a", i))).collect();
        assert_eq!(rows, expected);
    }

    #[test]
    fn test_grouped_rows_put_header_before_each_group() {
        let groups = vec![(Some(("ref", 1)), vec![("a", 2), ("b", 3)]), (None, vec![("c", 4)])];
        let rows = grouped_rows(groups, prepare);
        let expected = vec![
            PlainRow::header(content("ref", 1)),
            PlainRow::item(content("a", 2)),
            PlainRow::item(content("b", 3)),
            PlainRow::empty_header(),
            PlainRow::item(content("c", 4)),
        ];
        assert_eq!(rows, expected);
    }

    #[test]
    fn test_rows_into_model_sets_header_flags_and_content() {
        let rows = vec![PlainRow::header(content("ref", 1)), PlainRow::item(content("a", 2)), PlainRow::empty_header()];
        let model = rows_into_model(rows);

        assert_eq!(model.row_count(), 3);
        assert_eq!(read_row(&model, 0), (true, true, vec!["ref".to_string()], vec![1]));
        assert_eq!(read_row(&model, 1), (false, false, vec!["a".to_string()], vec![2]));
        assert_eq!(read_row(&model, 2), (true, false, vec![], vec![]));
    }

    #[test]
    fn test_rows_into_model_is_vec_model_with_unchecked_rows() {
        let model = rows_into_model(vec![PlainRow::item(content("a", 1))]);

        assert!(model.as_any().downcast_ref::<VecModel<SingleMainListModel>>().is_some());
        let row = model.row_data(0).unwrap();
        assert!(!row.checked);
        assert!(!row.focused_row);
        assert!(row.val_str.as_any().downcast_ref::<VecModel<SharedString>>().is_some());
    }

    #[test]
    fn test_simpler_model_round_trip_keeps_content() {
        use crate::simpler_model::{SimplerSingleMainListModel, ToSimplerVec, ToSlintModel};

        let model = rows_into_model(vec![PlainRow::header(content("ref", 1)), PlainRow::item(content("a", 2))]);
        let simpler = model.to_simpler_enumerated_vec();
        assert_eq!(simpler.iter().map(|(idx, _)| *idx).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(simpler[1].1.val_str, vec![SharedString::from("a")]);

        let back = ModelRc::new(VecModel::from(
            simpler.into_iter().map(|(_, m)| m).collect::<Vec<SimplerSingleMainListModel>>().to_vec_model(),
        ));
        assert_eq!(read_row(&back, 0), (true, true, vec!["ref".to_string()], vec![1]));
        assert_eq!(read_row(&back, 1), (false, false, vec!["a".to_string()], vec![2]));
    }
}
