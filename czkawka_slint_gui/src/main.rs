use std::rc::Rc;
use slint::{Model, ModelRc, SharedString, StandardListViewItem, VecModel};
slint::include_modules!();

fn main() {
    let app = MainWindow::new().unwrap();//.run().unwrap();
    let row_data: Rc<VecModel<ModelRc<SharedString>>> = Rc::new(VecModel::default());

    for r in 0..10000000 {
        let items = VecModel::default();

        for c in 0..3 {
            items.push(slint::format!("Item {r}.{c}").into());
        }

        row_data.push(ModelRc::new(items));
    }
    app.set_empty_folder_model(row_data.into());

    app.run().unwrap();
}