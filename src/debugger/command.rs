use std::borrow::Cow;
use std::str::{self, FromStr};

use nom::{IResult, eof, space, digit};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Step(usize),
    Run,
    Memdump(Option<usize>, usize),
    CpuInfo,
    Exit,
    Repeat,
}

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command(s.as_bytes()) {
            IResult::Done(_, c) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into())
        }
    }
}

named!(
    command<Command>,
    chain!(
        c: alt_complete!(
            step |
            run |
            memdump |
            cpuinfo |
            exit |
            repeat) ~
            eof,
    || c));

named!(
    step<Command>,
    chain!(
        alt_complete!(tag!("step") | tag!("s")) ~
            count: opt!(preceded!(space, usize_parser)),
        || Command::Step(count.unwrap_or(1))));

named!(
    run<Command>,
    map!(
        alt_complete!(tag!("run") | tag!("r")),
        |_| Command::Run));

named!(
    memdump<Command>,
    chain!(
        alt_complete!(tag!("memdump") | tag!("m")) ~
            address: opt!(preceded!(space, usize_parser)) ~
            size: opt!(preceded!(space, usize_parser)),
        || Command::Memdump(address, size.unwrap_or(256))));

named!(
    exit<Command>,
    map!(
        alt_complete!(tag!("exit") | tag!("quit") | tag!("e") | tag!("q")),
        |_| Command::Exit));

named!(
    cpuinfo<Command>,
    map!(
        alt_complete!(tag!("cpuinfo") | tag!("i")),
        |_| Command::CpuInfo));

named!(
    repeat<Command>,
    value!(Command::Repeat));

named!(
    usize_parser<usize>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8),
        FromStr::from_str));
