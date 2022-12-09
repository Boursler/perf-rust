//! # Main driver.
//! <p> Usage: <em> ruperf [COMMAND] [OPTION] </em>
//! where COMMAND is one of: </p>
//! <ul>
//! <li>test</li>
//! <li>stat</li>
//! <li>gui</li>
//! </ul>

extern crate structopt;
use ruperf_profile::stat::*;
use structopt::StructOpt;
use ruperf_profile::test::*;


/// Define command line options.
#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(
        setting = structopt::clap::AppSettings::TrailingVarArg,
        setting = structopt::clap::AppSettings::AllowLeadingHyphen,
        name = "stat",
        about = "Collects hardware/software event counters",
    )]
    Stat(StatOptions),
    #[structopt(
        setting = structopt::clap::AppSettings::TrailingVarArg,
        setting = structopt::clap::AppSettings::AllowLeadingHyphen,
        name = "test",
        about = "Runs sanity tests"
    )]
    Test(TestOptions),
}

/// Configuration settings for running stat. A program to profile is a required
/// argument. Default events will run on that program if no events are
/// specified. Specify events using the flag `-e or --event`. See `./ruperf stat
/// --help' for more information.
#[derive(Debug, StructOpt)]
pub struct StatOptions {
    #[structopt(short, long, help = "Event to collect", number_of_values = 1)]
    pub event: Vec<StatEvent>,

    // Allows multiple arguments to be passed, collects everything remaining on
    // the command line
    #[structopt(required = true, help = "Command to run")]
    pub command: Vec<String>,
}


/// Configuration settings for running test
#[derive(Debug, StructOpt)]
pub struct TestOptions {
    // Verbose flag, provides additional output
    #[structopt(short = "v", long = "verbose", help = "provide additional output")]
    pub verbose: bool,

    // Should list runnable tests instead of performing them
    #[structopt(
        short = "l",
        long = "list",
        help = "list runnable tests instead of running"
    )]
    pub should_list: bool,

    // Should format output as json
    #[structopt(short = "j", long = "json", help = "format output as json")]
    pub json: bool,

    // A comma-seperated list of tests to skip
    #[structopt(
        short = "s",
        long = "skip",
        help = "a comma-seperated list of tests to skip",
        default_value = ""
    )]
    pub to_skip: String,

    // A comma-seperated list of tests to run
    #[structopt(
        short = "o",
        long = "only",
        help = "a comma-seperated list of tests to run",
        default_value = ""
    )]
    pub to_run: String,
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Stat(x) => {
            run_stat(x.event, x.command);
        },
        Opt::Test(x) => run_test(x.verbose, x.should_list, x.json, x.to_skip, x.to_run),
    }
}
