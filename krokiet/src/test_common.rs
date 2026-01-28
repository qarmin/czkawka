use crate::SingleMainListModel;

pub(crate) fn get_main_list_model() -> SingleMainListModel {
    SingleMainListModel {
        checked: false,
        filled_header_row: false,
        header_row: false,
        selected_row: false,
        val_int: Default::default(),
        val_str: Default::default(),
    }
}
pub(crate) fn get_model_vec(items: usize) -> Vec<SingleMainListModel> {
    (0..items).map(|_| get_main_list_model()).collect::<Vec<_>>()
}
