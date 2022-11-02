// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

use core::time::Duration;

use anyhow::{bail, Result};
use clap::Parser;
use libbpf_rs::RingBufferBuilder;
use plain::Plain;
use std::net::Ipv4Addr;

mod connect {
    include!(concat!(env!("OUT_DIR"), "/connect.skel.rs"));
}

use connect::*;

/// Trace high run queue latency
#[derive(Debug, Parser)]
struct Command {
    /// Verbose debug output
    #[clap(short, long)]
    verbose: bool,
}

unsafe impl Plain for connect_bss_types::event {}

fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        bail!("Failed to increase rlimit");
    }

    Ok(())
}

fn callback1(data: &[u8]) -> i32 {
    let mut event = connect_bss_types::event::default();
    plain::copy_from_bytes(&mut event, data).expect("Data buffer was too short");


    eprintln!("received event, destination: {}, source: {}", Ipv4Addr::from(event.daddr), Ipv4Addr::from(event.saddr));

    // let task = std::str::from_utf8(&event.task).unwrap();
    0
}

fn main() -> Result<()> {
    let opts = Command::parse();

    let mut skel_builder = ConnectSkelBuilder::default();
    if opts.verbose {
        skel_builder.obj_builder.debug(true);
    }

    bump_memlock_rlimit()?;
    let mut open_skel = skel_builder.open()?;

    // // Write arguments into prog
    // open_skel.rodata().min_us = opts.latency;
    // open_skel.rodata().targ_pid = opts.pid;
    // open_skel.rodata().targ_tgid = opts.tid;

    // Begin tracing
    let mut skel = open_skel.load()?;
    skel.attach()?;
    // println!("Tracing run queue latency higher than {} us", opts.latency);
    // println!("{:8} {:16} {:7} {:14}", "TIME", "COMM", "TID", "LAT(us)");

    let mut builder = RingBufferBuilder::new();

    builder.add(skel.maps().events_ring(), callback1)?;

    let mgr = builder.build()?;


    loop {
        mgr.poll(Duration::from_millis(100))?;
    }

}
