use std::cell::RefCell;
use std::rc::Rc;

/// Runs the registered save at most once - whichever comes first: the event loop exiting or `run()` returning.
#[derive(Clone, Default)]
pub(crate) struct SaveOnExit(Rc<RefCell<Option<Box<dyn FnOnce()>>>>);

impl SaveOnExit {
    pub(crate) fn set(&self, save: impl FnOnce() + 'static) {
        *self.0.borrow_mut() = Some(Box::new(save));
    }

    pub(crate) fn run(&self) {
        let save = self.0.borrow_mut().take();
        if let Some(save) = save {
            save();
        }
    }
}

/// Cmd+Q goes through `NSApplication terminate:`, which calls `exit()` without returning from `run()`,
/// so the save has to happen when winit reports that the event loop is exiting.
#[cfg(target_os = "macos")]
pub(crate) fn install_quit_hook(save: &SaveOnExit) -> Result<(), slint::PlatformError> {
    slint::BackendSelector::new().with_winit_custom_application_handler(QuitHandler(save.clone())).select()
}

#[cfg(target_os = "macos")]
struct QuitHandler(SaveOnExit);

#[cfg(target_os = "macos")]
impl slint::winit_030::CustomApplicationHandler for QuitHandler {
    fn exiting(&mut self, _event_loop: &slint::winit_030::winit::event_loop::ActiveEventLoop) -> slint::winit_030::EventResult {
        self.0.run();
        slint::winit_030::EventResult::Propagate
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn runs_save_only_once() {
        let calls = Rc::new(Cell::new(0));
        let save_on_exit = SaveOnExit::default();
        let calls_in_save = Rc::clone(&calls);
        save_on_exit.set(move || calls_in_save.set(calls_in_save.get() + 1));

        save_on_exit.run();
        save_on_exit.run();

        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn run_without_save_does_nothing() {
        SaveOnExit::default().run();
    }
}
