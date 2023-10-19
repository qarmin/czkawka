use std::rc::Rc;
use slint::{Model, ModelRc, SharedString, VecModel};
slint::include_modules!();
use std::borrow::BorrowMut;

fn main() {
    let app = MainWindow::new().unwrap();//.run().unwrap();
    let row_data: Rc<VecModel<(bool,bool,ModelRc<SharedString>)>> = Rc::new(VecModel::default());

    for r in 0..1000 {
        let items = VecModel::default();

        for c in 0..3 {
            items.push(slint::format!("Item {r}.{c}").into());
        }

        row_data.push((r % 2 == 0, r% 3 == 0, ModelRc::new(items)));
    }
    app.set_empty_folder_model(row_data.into());

    let a = app.as_weak();
    app.on_deleted(move || {
        let app = a.upgrade().unwrap();


        let mut r = app.get_empty_folder_model();
        let m = r.borrow_mut();
        let length_before = m.iter().count();
        let mut s: Vec<_> = m.iter().filter(|(a,_b,_c)|{
            !*a
        }).collect();


        let length_after = s.len();
        if length_before != length_after {
            dbg!(format!("Items to remove {}", length_before - length_after));
            s.iter_mut().for_each(|(_a, selected_row, _)|{
                *selected_row = false;
            });
            let r = ModelRc::new(VecModel::from(s));
            app.set_empty_folder_model(r.into());
        }
    });

    app.run().unwrap();
}