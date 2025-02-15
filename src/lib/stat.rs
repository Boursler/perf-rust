//! # Stat driver.
//! <p> Usage: <em> ruperf stat [COMMAND] [ARGS] </em>
//! Where COMMAND and ARGS are a shell command and it's arguments. </p>


use crate::event::open::*;
use crate::utils::ParseError;
use os_pipe::pipe;
use std::io::prelude::*;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::str::{self, FromStr};
use std::time::Instant;

/// Supported events
#[derive(Debug, Copy, Clone)]
pub enum StatEvent {
    Cycles,
    Instructions,
    TaskClock,
    ContextSwitches,
    L1DCacheRead,
    L1DCacheWrite,
    L1DCacheReadMiss,
    L1ICacheReadMiss,
}

/// Match on each supported event to parse from command line
impl FromStr for StatEvent {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cycles" => Ok(StatEvent::Cycles),
            "instructions" => Ok(StatEvent::Instructions),
            "task-clock" => Ok(StatEvent::TaskClock),
            "context-switches" => Ok(StatEvent::ContextSwitches),
            "L1D-cache-reads" => Ok(StatEvent::L1DCacheRead),
            "L1D-cache-writes" => Ok(StatEvent::L1DCacheWrite),
            "L1D-cache-read-misses" => Ok(StatEvent::L1DCacheReadMiss),
            "L1I-cache-read-misses" => Ok(StatEvent::L1ICacheReadMiss),
            _ => Err(ParseError::InvalidEvent),
        }
    }
}

/// Match on each supported event to parse from command line.
/// Note that the context-switches event runs in kernel mode
/// and requires a perf_event_paranoid setting < 1.
impl ToString for StatEvent {
    fn to_string(&self) -> String {
        match self {
            StatEvent::Cycles => "cycles".to_string(),
            StatEvent::Instructions => "instructions".to_string(),
            StatEvent::TaskClock => "task clock".to_string(),
            StatEvent::ContextSwitches => "context switches".to_string(),
            StatEvent::L1DCacheRead => "L1D-cache-reads".to_string(),
            StatEvent::L1DCacheWrite => "L1D-cache-writes".to_string(),
            StatEvent::L1DCacheReadMiss => "L1D-cache-read-misses".to_string(),
            StatEvent::L1ICacheReadMiss => "L1I-cache-read-misses".to_string(),
        }
    }
}

struct Counter {
    event: Event,
    start: isize,
    stop: isize,
}

impl Counter {
    /// Generate list of timers for a given `pid`.
    pub fn counters(options: &mut Vec<StatEvent>, pid: i32) -> Vec<Counter> {
        let mut counters: Vec<Counter> = Vec::new();

        if options.is_empty() {
            options.push(StatEvent::Cycles);
            options.push(StatEvent::Instructions);
            options.push(StatEvent::TaskClock);
            options.push(StatEvent::ContextSwitches);
            options.push(StatEvent::L1DCacheRead);
            options.push(StatEvent::L1DCacheWrite);
            options.push(StatEvent::L1DCacheReadMiss);
            options.push(StatEvent::L1ICacheReadMiss);
        }

        for event in options {
            counters.push(Counter {
                event: Event::new(*event, Some(pid)),
                start: 0,
                stop: 0,
            });
        }

        counters
    }
}

pub fn launch_stat_process(
    instant: Instant,
    command: Vec<String>,
    mut child_reader: os_pipe::PipeReader,
    mut child_writer: os_pipe::PipeWriter,
) -> i32 {
    match unsafe { libc::fork() as i32 } {
        0 => {
            //set up command to execute and initialize read buffer
            let mut comm = Command::new(&command[0]);
            comm.args(&command[1..]);
            let mut buf = [0];

            // Hear from parent that we may start.
            let nread = child_reader.read(&mut buf).unwrap();
            assert_eq!(nread, 1);

            // Write start start time in nanos as [u8; 16].
            child_writer
                .write_all(&instant.elapsed().as_nanos().to_ne_bytes())
                .expect("Could not write start time");
            child_writer.flush().unwrap();
            drop(child_writer);

            let e = comm.exec();
            panic!("child command failed: {}", e);
        }
        pid_child => pid_child,
    }
}

/// Run perf stat on the given command and event combinations.
/// Currently starts and stops a cycles timer in serial for each event specified.
pub fn run_stat(mut events: Vec<StatEvent>, command: Vec<String>) {
    // In future rather than starting and stopping counter
    // in series for each event, events will have the ability
    // to be added in groups that will coordinate their timing.

    let (reader, mut writer) = pipe().unwrap();
    let (mut parent_reader, parent_writer) = pipe().unwrap();
    let child_reader = reader.try_clone().unwrap();
    let child_writer = parent_writer.try_clone().unwrap();

    let instant = Instant::now();
    let pid_child = launch_stat_process(
        instant.clone(),
        command.clone(),
        child_reader,
        child_writer,
    );
    let mut counters = Counter::counters(&mut events, pid_child);

    let mut buffer: [u8; 16] = [0; 16];
    let mut status: libc::c_int = 0;
    // Start all the counters.
    for counter in counters.iter_mut() {
        counter.start = counter.event.start_counter().unwrap();
    }
    // Notify child we are ready.
    writer.write_all(&[1]).unwrap();
    writer.flush().unwrap();
    let nread = parent_reader.read(&mut buffer).unwrap();
    let result = unsafe { libc::waitpid(pid_child, (&mut status) as *mut libc::c_int, 0) };
    // Let's see how long they took.
    let stop_time: u128 = instant.elapsed().as_nanos();
    for counter in counters.iter_mut() {
        counter.stop = counter.event.stop_counter().unwrap();
    }
    let start_time = u128::from_ne_bytes(buffer);
    let t = stop_time - start_time;
    assert_eq!(nread, 16);
    assert_eq!(result, pid_child);
    // Don't forget to drop the writer!
    drop(writer);

    println!(
        "Performance counter stats for '{}:'\n",
        command.get(0).unwrap()
    );

    for counter in counters {
        if matches!(counter.event.event, StatEvent::TaskClock) {
            println!(
                " {:.2} msec task-clock\n CPU utilized: {:.3}",
                (counter.stop - counter.start) as f64 / 1_000_000.0,
                (counter.stop - counter.start) as f64 / t as f64
            );
        } else {
            println!(
                " Number of {}: {}",
                counter.event.event.to_string(),
                counter.stop - counter.start
            );
        }
    }
}
