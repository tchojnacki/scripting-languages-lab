use clap::StructOpt;
use head_tail::{
    args::{CommonArgs, TailArgs},
    ringbuffer::RingBuffer,
    shared::handle_results,
};
use std::io::{self, BufRead};

fn main() {
    let args = TailArgs::parse();

    let lines = RingBuffer::from_iter(io::stdin().lock().lines().flatten(), args.expected_lines())
        .into_vec();

    handle_results(lines, &args);
}
