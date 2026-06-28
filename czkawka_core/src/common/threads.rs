use std::sync::Mutex;
use std::thread;

use log::debug;

use crate::common::consts::DEFAULT_WORKER_THREAD_SIZE;

static NUMBER_OF_THREADS: std::sync::LazyLock<Mutex<Option<usize>>> = std::sync::LazyLock::new(|| Mutex::new(None));
static ALL_AVAILABLE_THREADS: std::sync::LazyLock<Mutex<Option<usize>>> = std::sync::LazyLock::new(|| Mutex::new(None));

pub fn get_number_of_threads() -> usize {
    let data = NUMBER_OF_THREADS.lock().expect("Cannot fail").expect("Should be set before get");
    if data >= 1 { data } else { get_all_available_threads() }
}

pub fn get_all_available_threads() -> usize {
    let mut available_threads = ALL_AVAILABLE_THREADS.lock().expect("Cannot fail");

    if let Some(available_threads) = *available_threads {
        available_threads
    } else {
        let threads = thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        *available_threads = Some(threads);
        threads
    }
}

pub fn set_number_of_threads(thread_number: usize) {
    *NUMBER_OF_THREADS.lock().expect("Cannot fail") = Some(thread_number);

    let additional_message = if thread_number == 0 {
        format!(
            " (0 - means that all available threads will be used({}))",
            thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
        )
    } else {
        "".to_string()
    };
    debug!("Number of threads set to {thread_number}{additional_message}");

    rayon::ThreadPoolBuilder::new()
        .num_threads(get_number_of_threads())
        .stack_size(DEFAULT_WORKER_THREAD_SIZE)
        .build_global()
        .expect("Cannot set number of threads");
}
