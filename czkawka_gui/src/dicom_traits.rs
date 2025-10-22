use gtk4::ComboBoxText;
use gtk4::prelude::ComboBoxExtManual;

pub trait ComboBoxTraits {
    fn set_model_and_first<I, S>(&self, models: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;
}

impl ComboBoxTraits for ComboBoxText {
    fn set_model_and_first<I, S>(&self, models: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for item in models {
            self.append_text(item.as_ref());
        }
        self.set_active(Some(0));
    }
}
