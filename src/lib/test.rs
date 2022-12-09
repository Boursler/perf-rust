//! # Test driver.
//! <p> Usage: <em> ruperf test [OPTION] </em>
//! where OPTION is one of:
//! <ul>
//! <li>v, verbose</li>
//! <li>l, list</li>
//! <li>j, json</li>
//! <li>s, skip</li>
//! <li>o, only</li>
//! </ul>

mod basic;
mod counts;
mod events;
mod paranoid;
mod pfm;
mod testutils;

/// Test Struct
pub struct Test {
    pub name: String,
    pub description: String,
    pub call: fn(&RunSettings) -> TestResult,
    pub subtests: Vec<Test>,
    pub is_subtest: bool,
}

/// TestResult
#[derive(Clone)]
pub enum TestResult {
    Passed,
    Failed(String),
    Skipped,
}

pub struct RunSettings {
    pub verbose: bool,
    pub json: bool,
}

/// Handles the running of the "test" command.
pub fn run_test(verbose: bool, should_list: bool, json: bool, skip: String, to_run: String) {
    let mut to_skip: Vec<String> = Vec::new();
    let tests = testutils::make_tests();
    if !skip.is_empty() {
        for s in skip.split(',') {
            to_skip.push(s.to_string());
        }
    }
    if should_list {
        testutils::list_all_tests(&tests);
        return;
    }
    let settings = RunSettings {
        verbose: verbose || json,
        json: json,
    };
    if !to_run.is_empty() {
        to_skip = (0..tests.len()).map(|x| x.to_string()).collect();
        for s in to_run.split(',') {
            to_skip.retain(|x| *x != s);
        }
    }
    testutils::run_all_tests(&tests, &to_skip, &settings);
}
