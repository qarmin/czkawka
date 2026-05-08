use std::rc::Rc;

use gtk4::prelude::*;
use regex::Regex;

use crate::flg;

#[derive(Clone)]
pub struct GuiSelectDialog {
    pub dialog: gtk4::Window,

    // Action radio group
    pub action_all_except: gtk4::CheckButton,
    pub action_select_one: gtk4::CheckButton,
    pub action_select_all: gtk4::CheckButton,
    pub action_unselect_all: gtk4::CheckButton,
    pub action_reverse: gtk4::CheckButton,
    pub action_mark_same_size: gtk4::CheckButton,
    pub action_custom_select: gtk4::CheckButton,
    pub action_custom_unselect: gtk4::CheckButton,

    // Criterion radio group
    pub criterion_date: gtk4::CheckButton,
    pub criterion_path: gtk4::CheckButton,
    pub criterion_size: gtk4::CheckButton,

    // Direction for Date criterion
    pub direction_oldest: gtk4::CheckButton,
    pub direction_newest: gtk4::CheckButton,

    // Path filter (for SelectOne + Date)
    pub path_filter_any: gtk4::CheckButton,
    pub path_filter_longest: gtk4::CheckButton,
    pub path_filter_shortest: gtk4::CheckButton,

    // Direction for Path criterion
    pub path_direction_shortest: gtk4::CheckButton,
    pub path_direction_longest: gtk4::CheckButton,

    // Direction for Size criterion
    pub size_direction_smallest: gtk4::CheckButton,
    pub size_direction_biggest: gtk4::CheckButton,

    // Conditions (always visible, radio group)
    pub cond_none: gtk4::CheckButton,
    pub cond_same_size: gtk4::CheckButton,
    pub cond_same_path: gtk4::CheckButton,

    // Custom filter widgets
    pub custom_check_name: gtk4::CheckButton,
    pub custom_check_path: gtk4::CheckButton,
    pub custom_check_regex: gtk4::CheckButton,
    pub custom_check_case_sensitive: gtk4::CheckButton,
    pub custom_check_all_in_group: gtk4::CheckButton,
    pub custom_entry_name: gtk4::Entry,
    pub custom_entry_path: gtk4::Entry,
    pub custom_entry_regex: gtk4::Entry,

    // Frames for show/hide
    pub frame_action: gtk4::Frame,
    pub frame_criterion: gtk4::Frame,
    pub frame_direction_date: gtk4::Frame,
    pub frame_direction_path: gtk4::Frame,
    pub frame_direction_size: gtk4::Frame,
    pub frame_path_filter: gtk4::Frame,
    pub frame_conditions: gtk4::Frame,
    pub frame_custom: gtk4::Frame,

    pub button_apply: gtk4::Button,
    pub button_cancel: gtk4::Button,
}

impl GuiSelectDialog {
    pub fn create(window_main: &gtk4::Window) -> Self {
        let dialog = gtk4::Window::builder()
            .title(flg!("select_dialog_title"))
            .transient_for(window_main)
            .modal(true)
            .resizable(false)
            .hide_on_close(true)
            .build();

        let main_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(8)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        // ── Action ──────────────────────────────────────────────────────────
        let frame_action = gtk4::Frame::builder().label(flg!("select_dialog_action")).build();
        let action_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(2)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let action_all_except = gtk4::CheckButton::builder().label(flg!("select_dialog_action_all_except")).build();
        let action_select_one = gtk4::CheckButton::builder().label(flg!("select_dialog_action_one")).group(&action_all_except).build();
        let action_select_all = gtk4::CheckButton::builder().label(flg!("select_dialog_action_select_all")).group(&action_all_except).build();
        let action_unselect_all = gtk4::CheckButton::builder().label(flg!("select_dialog_action_unselect_all")).group(&action_all_except).build();
        let action_reverse = gtk4::CheckButton::builder().label(flg!("select_dialog_action_reverse")).group(&action_all_except).build();
        let action_mark_same_size = gtk4::CheckButton::builder().label(flg!("select_dialog_action_mark_same_size")).group(&action_all_except).build();
        let action_custom_select = gtk4::CheckButton::builder().label(flg!("select_dialog_action_custom_select")).group(&action_all_except).build();
        let action_custom_unselect = gtk4::CheckButton::builder().label(flg!("select_dialog_action_custom_unselect")).group(&action_all_except).build();
        action_all_except.set_active(true);

        for w in [
            &action_all_except,
            &action_select_one,
            &action_select_all,
            &action_unselect_all,
            &action_reverse,
            &action_mark_same_size,
            &action_custom_select,
            &action_custom_unselect,
        ] {
            action_box.append(w);
        }
        frame_action.set_child(Some(&action_box));

        // ── Criterion ────────────────────────────────────────────────────────
        let frame_criterion = gtk4::Frame::builder().label(flg!("select_dialog_criterion")).build();
        let criterion_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let criterion_date = gtk4::CheckButton::builder().label(flg!("select_dialog_criterion_date")).build();
        let criterion_path = gtk4::CheckButton::builder().label(flg!("select_dialog_criterion_path")).group(&criterion_date).build();
        let criterion_size = gtk4::CheckButton::builder().label(flg!("select_dialog_criterion_size")).group(&criterion_date).build();
        criterion_date.set_active(true);

        for w in [&criterion_date, &criterion_path, &criterion_size] {
            criterion_box.append(w);
        }
        frame_criterion.set_child(Some(&criterion_box));

        // ── Direction – Date ──────────────────────────────────────────────────
        let frame_direction_date = gtk4::Frame::builder().label(flg!("select_dialog_direction")).build();
        let dir_date_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let direction_oldest = gtk4::CheckButton::builder().label(flg!("select_dialog_direction_oldest")).build();
        let direction_newest = gtk4::CheckButton::builder().label(flg!("select_dialog_direction_newest")).group(&direction_oldest).build();
        direction_oldest.set_active(true);

        dir_date_box.append(&direction_oldest);
        dir_date_box.append(&direction_newest);
        frame_direction_date.set_child(Some(&dir_date_box));

        // ── Path filter (SelectOne + Date only) ──────────────────────────────
        let frame_path_filter = gtk4::Frame::builder().label(flg!("select_dialog_path_filter")).build();
        let path_filter_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let path_filter_any = gtk4::CheckButton::builder().label(flg!("select_dialog_path_filter_any")).build();
        let path_filter_longest = gtk4::CheckButton::builder().label(flg!("select_dialog_path_filter_longest")).group(&path_filter_any).build();
        let path_filter_shortest = gtk4::CheckButton::builder().label(flg!("select_dialog_path_filter_shortest")).group(&path_filter_any).build();
        path_filter_any.set_active(true);

        for w in [&path_filter_any, &path_filter_longest, &path_filter_shortest] {
            path_filter_box.append(w);
        }
        frame_path_filter.set_child(Some(&path_filter_box));

        // ── Direction – Path ─────────────────────────────────────────────────
        let frame_direction_path = gtk4::Frame::builder().label(flg!("select_dialog_direction")).build();
        let dir_path_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let path_direction_shortest = gtk4::CheckButton::builder().label(flg!("select_dialog_direction_shortest")).build();
        let path_direction_longest = gtk4::CheckButton::builder().label(flg!("select_dialog_direction_longest")).group(&path_direction_shortest).build();
        path_direction_shortest.set_active(true);

        dir_path_box.append(&path_direction_shortest);
        dir_path_box.append(&path_direction_longest);
        frame_direction_path.set_child(Some(&dir_path_box));

        // ── Direction – Size ─────────────────────────────────────────────────
        let frame_direction_size = gtk4::Frame::builder().label(flg!("select_dialog_direction")).build();
        let dir_size_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let size_direction_smallest = gtk4::CheckButton::builder().label(flg!("select_dialog_direction_smallest")).build();
        let size_direction_biggest = gtk4::CheckButton::builder().label(flg!("select_dialog_direction_biggest")).group(&size_direction_smallest).build();
        size_direction_smallest.set_active(true);

        dir_size_box.append(&size_direction_smallest);
        dir_size_box.append(&size_direction_biggest);
        frame_direction_size.set_child(Some(&dir_size_box));

        // ── Conditions ───────────────────────────────────────────────────────
        let frame_conditions = gtk4::Frame::builder().label(flg!("select_dialog_conditions")).build();
        let cond_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let cond_none = gtk4::CheckButton::builder().label(flg!("select_dialog_cond_none")).build();
        let cond_same_size = gtk4::CheckButton::builder().label(flg!("select_dialog_cond_same_size")).group(&cond_none).build();
        let cond_same_path = gtk4::CheckButton::builder().label(flg!("select_dialog_cond_same_path")).group(&cond_none).build();
        cond_none.set_active(true);

        for w in [&cond_none, &cond_same_size, &cond_same_path] {
            cond_box.append(w);
        }
        frame_conditions.set_child(Some(&cond_box));

        // ── Custom filter ────────────────────────────────────────────────────
        let frame_custom = gtk4::Frame::builder().label(flg!("select_dialog_custom")).build();
        let custom_grid = gtk4::Grid::builder()
            .row_homogeneous(true)
            .column_homogeneous(true)
            .row_spacing(4)
            .column_spacing(8)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(8)
            .margin_end(8)
            .build();

        let custom_check_name = gtk4::CheckButton::builder()
            .label(flg!("popover_custom_regex_name_label"))
            .tooltip_text(flg!("popover_custom_name_check_button_entry_tooltip"))
            .build();
        let custom_check_path = gtk4::CheckButton::builder()
            .label(flg!("popover_custom_regex_path_label"))
            .tooltip_text(flg!("popover_custom_path_check_button_entry_tooltip"))
            .build();
        let custom_check_regex = gtk4::CheckButton::builder()
            .label(flg!("popover_custom_regex_regex_label"))
            .tooltip_text(flg!("popover_custom_regex_check_button_entry_tooltip"))
            .build();
        let custom_check_case_sensitive = gtk4::CheckButton::builder()
            .label(flg!("popover_custom_case_sensitive_check_button"))
            .tooltip_text(flg!("popover_custom_case_sensitive_check_button_tooltip"))
            .active(false)
            .build();
        let custom_check_all_in_group = gtk4::CheckButton::builder()
            .label(flg!("popover_custom_all_in_group_label"))
            .tooltip_text(flg!("popover_custom_not_all_check_button_tooltip"))
            .active(true)
            .build();

        let custom_entry_name = gtk4::Entry::builder().tooltip_text(flg!("popover_custom_name_check_button_entry_tooltip")).build();
        let custom_entry_path = gtk4::Entry::builder().tooltip_text(flg!("popover_custom_path_check_button_entry_tooltip")).build();
        let custom_entry_regex = gtk4::Entry::builder()
            .tooltip_text(flg!("popover_custom_regex_check_button_entry_tooltip"))
            .sensitive(false)
            .build();
        let custom_label_regex_valid = gtk4::Label::new(None);

        // Regex validity feedback
        {
            let label = custom_label_regex_valid.clone();
            custom_entry_regex.connect_changed(move |entry| {
                let text = entry.text().to_string();
                let msg = if text.is_empty() {
                    String::new()
                } else {
                    match Regex::new(&text) {
                        Ok(_) => flg!("popover_valid_regex"),
                        Err(_) => flg!("popover_invalid_regex"),
                    }
                };
                label.set_text(&msg);
            });
        }

        // Regex toggle disables path/name
        {
            let cp = custom_check_path.clone();
            let cn = custom_check_name.clone();
            let ep = custom_entry_path.clone();
            let en = custom_entry_name.clone();
            let er = custom_entry_regex.clone();
            custom_check_regex.connect_toggled(move |cb| {
                let active = cb.is_active();
                cp.set_sensitive(!active);
                cn.set_sensitive(!active);
                ep.set_sensitive(!active);
                en.set_sensitive(!active);
                er.set_sensitive(active);
            });
        }

        custom_grid.attach(&custom_check_name, 0, 0, 1, 1);
        custom_grid.attach(&custom_entry_name, 1, 0, 1, 1);
        custom_grid.attach(&custom_check_path, 0, 1, 1, 1);
        custom_grid.attach(&custom_entry_path, 1, 1, 1, 1);
        custom_grid.attach(&custom_check_regex, 0, 2, 1, 1);
        custom_grid.attach(&custom_entry_regex, 1, 2, 1, 1);
        custom_grid.attach(&custom_label_regex_valid, 0, 3, 2, 1);
        custom_grid.attach(&custom_check_case_sensitive, 0, 4, 2, 1);
        custom_grid.attach(&custom_check_all_in_group, 0, 5, 2, 1);
        frame_custom.set_child(Some(&custom_grid));

        // ── Bottom buttons ────────────────────────────────────────────────────
        let button_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .halign(gtk4::Align::End)
            .spacing(8)
            .build();

        let button_cancel = gtk4::Button::builder().label(flg!("general_close_button")).build();
        let button_apply = gtk4::Button::builder().label(flg!("general_ok_button")).build();
        button_box.append(&button_cancel);
        button_box.append(&button_apply);

        {
            let d = dialog.clone();
            button_cancel.connect_clicked(move |_| d.set_visible(false));
        }

        // ── Initial visibility ────────────────────────────────────────────────
        frame_direction_path.set_visible(false);
        frame_direction_size.set_visible(false);
        frame_path_filter.set_visible(false);
        frame_custom.set_visible(false);
        // criterion + direction_date + conditions start visible (AllExcept + Date is default)

        // ── Assemble ──────────────────────────────────────────────────────────
        for w in [
            frame_action.upcast_ref::<gtk4::Widget>(),
            frame_criterion.upcast_ref(),
            frame_direction_date.upcast_ref(),
            frame_path_filter.upcast_ref(),
            frame_direction_path.upcast_ref(),
            frame_direction_size.upcast_ref(),
            frame_conditions.upcast_ref(),
            frame_custom.upcast_ref(),
            button_box.upcast_ref(),
        ] {
            main_box.append(w);
        }
        dialog.set_child(Some(&main_box));

        // ── Internal signal wiring (visibility updates) ───────────────────────
        let sd = GuiSelectDialog {
            dialog,
            action_all_except,
            action_select_one,
            action_select_all,
            action_unselect_all,
            action_reverse,
            action_mark_same_size,
            action_custom_select,
            action_custom_unselect,
            criterion_date,
            criterion_path,
            criterion_size,
            direction_oldest,
            direction_newest,
            path_filter_any,
            path_filter_longest,
            path_filter_shortest,
            path_direction_shortest,
            path_direction_longest,
            size_direction_smallest,
            size_direction_biggest,
            cond_none,
            cond_same_size,
            cond_same_path,
            custom_check_name,
            custom_check_path,
            custom_check_regex,
            custom_check_case_sensitive,
            custom_check_all_in_group,
            custom_entry_name,
            custom_entry_path,
            custom_entry_regex,
            frame_action,
            frame_criterion,
            frame_direction_date,
            frame_direction_path,
            frame_direction_size,
            frame_path_filter,
            frame_conditions,
            frame_custom,
            button_apply,
            button_cancel,
        };

        sd.wire_visibility_signals();
        sd
    }

    fn wire_visibility_signals(&self) {
        // Build a shared closure that reads current state and updates visibility.
        // GTK widgets are GObjects (reference-counted), so cloning is cheap.
        let update = {
            let fc = self.frame_criterion.clone();
            let fdd = self.frame_direction_date.clone();
            let fdp = self.frame_direction_path.clone();
            let fds = self.frame_direction_size.clone();
            let fpf = self.frame_path_filter.clone();
            let fcond = self.frame_conditions.clone();
            let fcu = self.frame_custom.clone();
            let cond_none = self.cond_none.clone();

            let a_ae = self.action_all_except.clone();
            let a_one = self.action_select_one.clone();
            let a_cs = self.action_custom_select.clone();
            let a_cu = self.action_custom_unselect.clone();

            let c_date = self.criterion_date.clone();
            let c_path = self.criterion_path.clone();
            let c_size = self.criterion_size.clone();

            let pf_any = self.path_filter_any.clone();

            let ccu = self.custom_check_all_in_group.clone();

            Rc::new(move || {
                let needs_criterion = a_ae.is_active() || a_one.is_active();
                let is_custom = a_cs.is_active() || a_cu.is_active();
                let is_select = a_cs.is_active();

                fc.set_visible(needs_criterion);
                fcu.set_visible(is_custom);
                // "All in group" only makes sense for custom select (not unselect)
                ccu.set_visible(is_select);

                if needs_criterion {
                    let is_date = c_date.is_active();
                    let is_path = c_path.is_active();
                    let is_size = c_size.is_active();
                    let is_one = a_one.is_active();

                    fdd.set_visible(is_date);
                    fdp.set_visible(is_path);
                    fds.set_visible(is_size);
                    // Path filter only for SelectOne + Date
                    fpf.set_visible(is_one && is_date);

                    // Conditions sensitive when SelectOne + (Date + path_filter=Any) or SelectOne + Path
                    let cond_ok = is_one && ((is_date && pf_any.is_active()) || is_path);
                    fcond.set_sensitive(cond_ok);
                    if !cond_ok {
                        cond_none.set_active(true);
                    }
                } else {
                    fdd.set_visible(false);
                    fdp.set_visible(false);
                    fds.set_visible(false);
                    fpf.set_visible(false);
                    fcond.set_sensitive(false);
                    cond_none.set_active(true);
                }
            })
        };

        // Connect all action radios
        for btn in [
            &self.action_all_except,
            &self.action_select_one,
            &self.action_select_all,
            &self.action_unselect_all,
            &self.action_reverse,
            &self.action_mark_same_size,
            &self.action_custom_select,
            &self.action_custom_unselect,
        ] {
            let u = update.clone();
            btn.connect_toggled(move |_| u());
        }

        // Connect criterion radios
        for btn in [&self.criterion_date, &self.criterion_path, &self.criterion_size] {
            let u = update.clone();
            btn.connect_toggled(move |_| u());
        }

        // Connect path filter radios
        for btn in [&self.path_filter_any, &self.path_filter_longest, &self.path_filter_shortest] {
            let u = update.clone();
            btn.connect_toggled(move |_| u());
        }
    }

    pub fn update_language(&self) {
        self.dialog.set_title(Some(&flg!("select_dialog_title")));
        self.frame_action.set_label(Some(&flg!("select_dialog_action")));
        self.frame_criterion.set_label(Some(&flg!("select_dialog_criterion")));
        self.frame_direction_date.set_label(Some(&flg!("select_dialog_direction")));
        self.frame_direction_path.set_label(Some(&flg!("select_dialog_direction")));
        self.frame_direction_size.set_label(Some(&flg!("select_dialog_direction")));
        self.frame_path_filter.set_label(Some(&flg!("select_dialog_path_filter")));
        self.frame_conditions.set_label(Some(&flg!("select_dialog_conditions")));
        self.frame_custom.set_label(Some(&flg!("select_dialog_custom")));
        self.button_apply.set_label(&flg!("general_ok_button"));
        self.button_cancel.set_label(&flg!("general_close_button"));
        self.action_all_except.set_label(Some(&flg!("select_dialog_action_all_except")));
        self.action_select_one.set_label(Some(&flg!("select_dialog_action_one")));
        self.action_select_all.set_label(Some(&flg!("select_dialog_action_select_all")));
        self.action_unselect_all.set_label(Some(&flg!("select_dialog_action_unselect_all")));
        self.action_reverse.set_label(Some(&flg!("select_dialog_action_reverse")));
        self.action_mark_same_size.set_label(Some(&flg!("select_dialog_action_mark_same_size")));
        self.action_custom_select.set_label(Some(&flg!("select_dialog_action_custom_select")));
        self.action_custom_unselect.set_label(Some(&flg!("select_dialog_action_custom_unselect")));
        self.criterion_date.set_label(Some(&flg!("select_dialog_criterion_date")));
        self.criterion_path.set_label(Some(&flg!("select_dialog_criterion_path")));
        self.criterion_size.set_label(Some(&flg!("select_dialog_criterion_size")));
        self.direction_oldest.set_label(Some(&flg!("select_dialog_direction_oldest")));
        self.direction_newest.set_label(Some(&flg!("select_dialog_direction_newest")));
        self.path_filter_any.set_label(Some(&flg!("select_dialog_path_filter_any")));
        self.path_filter_longest.set_label(Some(&flg!("select_dialog_path_filter_longest")));
        self.path_filter_shortest.set_label(Some(&flg!("select_dialog_path_filter_shortest")));
        self.path_direction_shortest.set_label(Some(&flg!("select_dialog_direction_shortest")));
        self.path_direction_longest.set_label(Some(&flg!("select_dialog_direction_longest")));
        self.size_direction_smallest.set_label(Some(&flg!("select_dialog_direction_smallest")));
        self.size_direction_biggest.set_label(Some(&flg!("select_dialog_direction_biggest")));
        self.cond_none.set_label(Some(&flg!("select_dialog_cond_none")));
        self.cond_same_size.set_label(Some(&flg!("select_dialog_cond_same_size")));
        self.cond_same_path.set_label(Some(&flg!("select_dialog_cond_same_path")));
        self.custom_check_name.set_label(Some(&flg!("popover_custom_regex_name_label")));
        self.custom_check_path.set_label(Some(&flg!("popover_custom_regex_path_label")));
        self.custom_check_regex.set_label(Some(&flg!("popover_custom_regex_regex_label")));
        self.custom_check_case_sensitive.set_label(Some(&flg!("popover_custom_case_sensitive_check_button")));
        self.custom_check_all_in_group.set_label(Some(&flg!("popover_custom_all_in_group_label")));
    }

    /// Called when showing the dialog for a tool — adjusts which actions/criteria are available.
    pub fn set_available_modes(&self, arr: &[crate::helpers::enums::PopoverTypes]) {
        use crate::helpers::enums::PopoverTypes;

        let has_all = arr.contains(&PopoverTypes::All);
        let has_date = arr.contains(&PopoverTypes::Date);
        let has_path = arr.contains(&PopoverTypes::PathLength);
        let has_size = arr.contains(&PopoverTypes::Size);
        let has_rev = arr.contains(&PopoverTypes::Reverse);
        let has_custom = arr.contains(&PopoverTypes::Custom);

        self.action_select_all.set_visible(has_all);
        self.action_unselect_all.set_visible(has_all);
        self.action_reverse.set_visible(has_rev);
        self.action_custom_select.set_visible(has_custom);
        self.action_custom_unselect.set_visible(has_custom);

        // AllExcept / SelectOne need at least one valid criterion
        let has_criterion = has_date || has_path || has_size;
        self.action_all_except.set_visible(has_criterion);
        self.action_select_one.set_visible(has_date); // SelectOne only makes sense for Date

        // Criteria visibility
        self.criterion_date.set_visible(has_date);
        self.criterion_path.set_visible(has_path);
        self.criterion_size.set_visible(has_size);

        // MarkSameSize visibility — needs a color column (duplicate-like tools)
        // We use has_date as proxy (same tools that have date have color column)
        self.action_mark_same_size.set_visible(has_date);

        // Reset to a safe default if the currently active action is now hidden
        self.reset_to_valid_default();
    }

    fn reset_to_valid_default(&self) {
        // Find first visible action and activate it
        for btn in [
            &self.action_all_except,
            &self.action_select_one,
            &self.action_select_all,
            &self.action_unselect_all,
            &self.action_reverse,
            &self.action_mark_same_size,
            &self.action_custom_select,
            &self.action_custom_unselect,
        ] {
            if btn.is_visible() {
                // Only switch if the currently active button is hidden
                let current_active_is_visible = [
                    &self.action_all_except,
                    &self.action_select_one,
                    &self.action_select_all,
                    &self.action_unselect_all,
                    &self.action_reverse,
                    &self.action_mark_same_size,
                    &self.action_custom_select,
                    &self.action_custom_unselect,
                ]
                .iter()
                .any(|b| b.is_active() && b.is_visible());
                if !current_active_is_visible {
                    btn.set_active(true);
                }
                break;
            }
        }

        // Same for criterion
        for btn in [&self.criterion_date, &self.criterion_path, &self.criterion_size] {
            if btn.is_visible() {
                let ok = [&self.criterion_date, &self.criterion_path, &self.criterion_size]
                    .iter()
                    .any(|b| b.is_active() && b.is_visible());
                if !ok {
                    btn.set_active(true);
                }
                break;
            }
        }
    }
}
