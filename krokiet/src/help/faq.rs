use super::{answer, general_section, question, tab};
use crate::{HelpSection, HelpTab, flk};

fn faq_general() -> HelpTab {
    tab(
        flk!("help_tab_general"),
        vec![
            question(flk!("help_faq_general_q1")),
            answer(flk!("help_faq_general_a2")),
            question(flk!("help_faq_general_q3")),
            answer(flk!("help_faq_general_a4")),
            question(flk!("help_faq_general_q5")),
            answer(flk!("help_faq_general_a6")),
            question(flk!("help_faq_general_q7")),
            answer(flk!("help_faq_general_a8")),
            question(flk!("help_faq_general_q9")),
            answer(flk!("help_faq_general_a10")),
            question(flk!("help_faq_general_q11")),
            answer(flk!("help_faq_general_a12")),
            question(flk!("help_faq_general_q13")),
            answer(flk!("help_faq_general_a14")),
            question(flk!("help_faq_general_q15")),
            answer(flk!("help_faq_general_a16")),
            question(flk!("help_faq_general_q17")),
            answer(flk!("help_faq_general_a18")),
        ],
    )
}

fn faq_scanning() -> HelpTab {
    tab(
        flk!("help_tab_scanning"),
        vec![
            question(flk!("help_faq_scanning_q1")),
            answer(flk!("help_faq_scanning_a2")),
            question(flk!("help_faq_scanning_q3")),
            answer(flk!("help_faq_scanning_a4")),
            question(flk!("help_faq_scanning_q5")),
            answer(flk!("help_faq_scanning_a6")),
            question(flk!("help_faq_scanning_q7")),
            answer(flk!("help_faq_scanning_a8")),
            question(flk!("help_faq_scanning_q9")),
            answer(flk!("help_faq_scanning_a10")),
            question(flk!("help_faq_scanning_q11")),
            answer(flk!("help_faq_scanning_a12")),
            question(flk!("help_faq_scanning_q13")),
            answer(flk!("help_faq_scanning_a14")),
            question(flk!("help_faq_scanning_q15")),
            answer(flk!("help_faq_scanning_a16")),
            question(flk!("help_faq_scanning_q17")),
            answer(flk!("help_faq_scanning_a18")),
        ],
    )
}

fn faq_results() -> HelpTab {
    tab(
        flk!("help_tab_results"),
        vec![
            question(flk!("help_faq_results_q1")),
            answer(flk!("help_faq_results_a2")),
            question(flk!("help_faq_results_q3")),
            answer(flk!("help_faq_results_a4")),
            question(flk!("help_faq_results_q5")),
            answer(flk!("help_faq_results_a6")),
            question(flk!("help_faq_results_q7")),
            answer(flk!("help_faq_results_a8")),
            question(flk!("help_faq_results_q9")),
            answer(flk!("help_faq_results_a10")),
            question(flk!("help_faq_results_q11")),
            answer(flk!("help_faq_results_a12")),
            question(flk!("help_faq_results_q13")),
            answer(flk!("help_faq_results_a14")),
            question(flk!("help_faq_results_q15")),
            answer(flk!("help_faq_results_a16")),
            question(flk!("help_faq_results_q17")),
            answer(flk!("help_faq_results_a18")),
        ],
    )
}

fn faq_troubleshooting() -> HelpTab {
    tab(
        flk!("help_tab_troubleshooting"),
        vec![
            question(flk!("help_faq_troubleshooting_q1")),
            answer(flk!("help_faq_troubleshooting_a2")),
            question(flk!("help_faq_troubleshooting_q3")),
            answer(flk!("help_faq_troubleshooting_a4")),
            question(flk!("help_faq_troubleshooting_q5")),
            answer(flk!("help_faq_troubleshooting_a6")),
            question(flk!("help_faq_troubleshooting_q7")),
            answer(flk!("help_faq_troubleshooting_a8")),
            question(flk!("help_faq_troubleshooting_q9")),
            answer(flk!("help_faq_troubleshooting_a10")),
            question(flk!("help_faq_troubleshooting_q11")),
            answer(flk!("help_faq_troubleshooting_a12")),
            question(flk!("help_faq_troubleshooting_q13")),
            answer(flk!("help_faq_troubleshooting_a14")),
            question(flk!("help_faq_troubleshooting_q15")),
            answer(flk!("help_faq_troubleshooting_a16")),
            question(flk!("help_faq_troubleshooting_q17")),
            answer(flk!("help_faq_troubleshooting_a18")),
        ],
    )
}

pub(super) fn faq() -> HelpSection {
    general_section(flk!("help_section_faq"), vec![faq_general(), faq_scanning(), faq_results(), faq_troubleshooting()])
}
