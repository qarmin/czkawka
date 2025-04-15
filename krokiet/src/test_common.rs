use slint::{ModelRc, VecModel};

use crate::MainListModel;

pub fn get_main_list_model() -> MainListModel {
    MainListModel {
        selected_row: false,
        val_int: Default::default(),
        checked: false,
        filled_header_row: false,
        header_row: false,
        val_str: Default::default(),
    }
}
pub fn get_model_vec(items: usize) -> Vec<MainListModel> {
    (0..items).map(|_| get_main_list_model()).collect::<Vec<_>>()
}
pub fn create_model_from_model_vec<T: Clone + 'static>(model_vec: &[T]) -> ModelRc<T> {
    ModelRc::new(VecModel::from(model_vec.to_owned()))
}
