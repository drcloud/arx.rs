#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

extern crate arx;

const HELP: &'static str = "
USAGE:  arx        <spec-file>+    >  <target>
        arx     <  <spec-file>     >  <target>
        arx -s     <shell-script>  >  <target>
        arx -s  <  <shell-script>  >  <target>
        arx -f [-s] <file> <args>*
        arx data <url> <destination>?
        arx code <cmd> <args>*

Consumes a manifest file, which describes a sequence of commands and data files
to make available while they are running, and produces a Bash script which
executes the manifest. (With `-e`, executes instead of generating a script.)

Arx accepts manifests with `code` and `data` sections, which are both arrays.

The elements of the `code` section are either single strings or arrays of
strings. The first (or only) string is the command word; it may be a URL, in
which case, Arx's URL handling is invoked to fetch the command. The commands
are executed in the order given. The first failure terminates that task.

The elements of the `data` section are either single strings or one element
key/value maps. Single strings indicate sources to unpack in the present
directory (usually a temporary directory); a mapping specifies a source (on
the left) and a target location (on the right). Note that the `data` section
is an array of such mappings, not itself a mapping; so a single source may be
specified more than once. The sources are unpacked in the order given. The
first failure terminates that task.
";


pub fn main() {
    let spec = Arg::with_name("strings").multiple(true);
    let execute = Arg::with_name("execute").short("f");
    let as_shell = Arg::with_name("as-shell").short("s");
    let cmd = Arg::with_name("cmd").required(true);
    let args = Arg::with_name("args").multiple(true);
    let url = Arg::with_name("url").required(true);
    let destination = Arg::with_name("destination");
    let code = SubCommand::with_name("code").arg(cmd).arg(args)
                          .usage("arx code <cmd> <args>*");
    let data = SubCommand::with_name("data").arg(url).arg(destination)
                          .usage("arx data <url> <destination>?");
    let app = App::new("arx").version(crate_version!())
                             .version_short("v")
                             .usage("arx [-f] [-s] <file>+")
                             .help(HELP.trim())
                             .arg(spec)
                             .arg(execute)
                             .arg(as_shell)
                             .subcommand(code)
                             .subcommand(data);
    let matches = app.get_matches();
    let mode = Mode::from(matches);
    println!("{:?}", mode);
}

#[derive(Debug)]
pub enum Input {
    File(String),
    StdIO
}

use Input::*;

impl Into<Input> for String {
    fn into(self) -> Input {
        if "-" == self { StdIO } else { File(self) }
    }
}

impl Into<Input> for Option<String> {
    fn into(self) -> Input {
        self.map(|s| s.into()).unwrap_or(StdIO)
    }
}

#[derive(Debug)]
enum Mode {
    Filter(Vec<Input>),
    Shell(Input),
    Execute(Input, Vec<String>),
    ExecuteShell(Input, Vec<String>),
    Code(String, Vec<String>),
    Data(String, Option<String>),
}

use Mode::*;

impl<'a> From<ArgMatches<'a>> for Mode {
    fn from(matches: ArgMatches) -> Self {
        let execute = matches.is_present("execute");
        let shell = matches.is_present("as-shell");
        let varargs = many(&matches, &"strings");
        let (cmd, argv) = if varargs.len() > 0 {
                              (Some(varargs[0].clone()), tail(&varargs))
                          } else {
                              (None, vec!())
                          };
        match matches.subcommand() {
            (name, Some(m)) => match name {
                "code" => Code(req(m, &"cmd"), many(m, &"args")),
                "data" => Data(req(m, &"url"), opt(m, &"destination")),
                _ => panic!("No such subcommand: {}", name),
            },
            _ if execute && shell => ExecuteShell(cmd.into(), argv),
            _ if execute => Execute(cmd.into(), argv),
            _ if shell => if argv.len() == 0 {
                Shell(cmd.into())
            } else {
                panic!("Please pass only one argument to `-s` or add `-f`.")
            },
            _ => Filter(varargs.iter().map(|s| s.clone().into()).collect()),
        }
    }
}


fn many<S: AsRef<str>>(m: &ArgMatches, key: &S) -> Vec<String> {
    if m.is_present(key) {
        m.values_of(key).unwrap().map(|s| s.into()).collect()
    } else {
        vec!()
    }
}

fn req<S: AsRef<str>>(m: &ArgMatches, key: &S) -> String {
    m.value_of(key).unwrap().into()
}

fn opt<S: AsRef<str>>(m: &ArgMatches, key: &S) -> Option<String> {
    m.value_of(key).map(|s| s.into())
}

fn tail(vec: &Vec<String>) -> Vec<String> {
    vec.iter().skip(1).map(|s| s.clone()).collect()
}
