use slint::{Model, ModelRc, SharedString, VecModel};

use crate::SingleMainListModel;
use crate::common::connect_i32_into_u64;

#[derive(Clone)]
pub struct SimplerSingleMainListModel {
    pub checked: bool,
    pub filled_header_row: bool,
    pub header_row: bool,
    pub selected_row: bool,
    pub val_int: Vec<i32>,
    pub val_str: Vec<String>,
}

impl SimplerSingleMainListModel {
    pub(crate) fn get_size(&self, size_idx: usize) -> u64 {
        connect_i32_into_u64(self.val_int[size_idx], self.val_int[size_idx + 1])
    }
    #[allow(clippy::allow_attributes)]
    #[expect(clippy::print_stdout)]
    #[allow(dead_code)] // TODO - rust with some version shows this
    pub(crate) fn debug_print(&self) {
        println!(
            "SimplerSingleMainListModel: checked: {}, filled_header_row: {}, header_row: {}, selected_row: {}, val_int: {:?}, val_str: {:?}",
            self.checked, self.filled_header_row, self.header_row, self.selected_row, self.val_int, self.val_str
        );
    }
}

impl From<&SingleMainListModel> for SimplerSingleMainListModel {
    fn from(model: &SingleMainListModel) -> Self {
        Self {
            checked: model.checked,
            filled_header_row: model.filled_header_row,
            header_row: model.header_row,
            selected_row: model.selected_row,
            val_int: model.val_int.iter().collect(),
            val_str: model.val_str.iter().map(|e| e.to_string()).collect(),
        }
    }
}
impl From<SimplerSingleMainListModel> for SingleMainListModel {
    fn from(val: SimplerSingleMainListModel) -> Self {
        Self {
            checked: val.checked,
            filled_header_row: val.filled_header_row,
            header_row: val.header_row,
            selected_row: val.selected_row,
            val_int: ModelRc::new(VecModel::from(val.val_int)),
            val_str: ModelRc::new(VecModel::from(val.val_str.into_iter().map(|s| s.into()).collect::<Vec<SharedString>>())),
        }
    }
}

pub trait ToSimplerVec {
    fn to_simpler_enumerated_vec(self) -> Vec<(usize, SimplerSingleMainListModel)>;
}

impl ToSimplerVec for ModelRc<SingleMainListModel> {
    fn to_simpler_enumerated_vec(self) -> Vec<(usize, SimplerSingleMainListModel)> {
        let vec_model = self.as_any().downcast_ref::<VecModel<SingleMainListModel>>().expect("Only VecModel is supported");
        vec_model.iter().enumerate().map(|(index, model)| (index, SimplerSingleMainListModel::from(&model))).collect()
    }
}

pub trait ToSlintModel {
    fn to_vec_model(self) -> Vec<SingleMainListModel>;
}
impl ToSlintModel for Vec<SimplerSingleMainListModel> {
    fn to_vec_model(self) -> Vec<SingleMainListModel> {
        self.into_iter().map(|model| model.into()).collect()
    }
}

pub trait DebugPrintSimplerModel {
    #[expect(dead_code)]
    fn debug_print_simpler_models(&self);
}
impl DebugPrintSimplerModel for Vec<SimplerSingleMainListModel> {
    #[expect(clippy::print_stdout)]
    fn debug_print_simpler_models(&self) {
        println!("=====================START DEBUG PRINT SIMPLER MODELS=====================");
        println!("Simpler Model with {} items", self.len());
        for item in self {
            item.debug_print();
        }
        println!("=====================END DEBUG PRINT SIMPLER MODELS=====================");
    }
}
