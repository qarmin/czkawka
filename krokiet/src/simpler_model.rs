#![allow(dead_code)] // TODO
#![allow(unused)]

use crate::MainListModel;
use slint::{Model, ModelExt, ModelRc, SharedString, VecModel};

#[derive(Clone)]
pub struct SimplerMainListModel {
    pub checked: bool,
    pub filled_header_row: bool,
    pub header_row: bool,
    pub selected_row: bool,
    pub val_int: Vec<i32>,
    pub val_str: Vec<String>,
}

impl From<&MainListModel> for SimplerMainListModel {
    fn from(model: &MainListModel) -> Self {
        Self {
            checked: model.checked,
            filled_header_row: model.filled_header_row,
            header_row: model.header_row,
            selected_row: model.selected_row,
            val_int: model.val_int.iter().map(|e| e).collect(),
            val_str: model.val_str.iter().map(|e| e.to_string()).collect(),
        }
    }
}
impl Into<MainListModel> for SimplerMainListModel {
    fn into(self) -> MainListModel {
        MainListModel {
            checked: self.checked,
            filled_header_row: self.filled_header_row,
            header_row: self.header_row,
            selected_row: self.selected_row,
            val_int: ModelRc::new(VecModel::from(self.val_int)),
            val_str: ModelRc::new(VecModel::from(self.val_str.into_iter().map(|s| s.into()).collect::<Vec<SharedString>>())),
        }
    }
}

pub trait ToSimplerVec {
    fn to_simpler_vec(self) -> Vec<SimplerMainListModel>;
    fn to_simpler_enumerated_vec(self) -> Vec<(usize, SimplerMainListModel)>;
}

impl ToSimplerVec for ModelRc<MainListModel> {
    fn to_simpler_vec(self) -> Vec<SimplerMainListModel> {
        let vec_model = self.as_any().downcast_ref::<VecModel<MainListModel>>().expect("Only VecModel is supported");
        vec_model.iter().map(|model| SimplerMainListModel::from(&model)).collect()
    }
    fn to_simpler_enumerated_vec(self) -> Vec<(usize, SimplerMainListModel)> {
        let vec_model = self.as_any().downcast_ref::<VecModel<MainListModel>>().expect("Only VecModel is supported");
        vec_model.iter().enumerate().map(|(index, model)| (index, SimplerMainListModel::from(&model))).collect()
    }
}

pub trait ToSlintModel {
    fn to_rc_model(self) -> ModelRc<SimplerMainListModel>;
    fn to_vec_model(self) -> Vec<MainListModel>;
}
impl ToSlintModel for Vec<SimplerMainListModel> {
    fn to_rc_model(self) -> ModelRc<SimplerMainListModel> {
        ModelRc::new(VecModel::from(self))
    }
    fn to_vec_model(self) -> Vec<MainListModel> {
        self.into_iter().map(|model| model.into()).collect()
    }
}
