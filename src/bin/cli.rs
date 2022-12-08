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

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Stat(x) => {
            run_stat(x.event, x.command);
        },
        Opt::Test(x) => run_test(&x),
    }
}
