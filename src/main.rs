#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

extern crate arx;

const HELP: &'static str = "
USAGE:  arx        <spec-file>+    > <target>
        arx     <  <spec-file>     > <target>
        arx -s     <shell-script>  > <target>
        arx -s  <  <shell-script>  > <target>
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
    println!("{:?}", mode(matches));
}


fn mode(matches: ArgMatches) -> Mode {
    let execute = matches.is_present("execute");
    let shell = matches.is_present("as-shell");
    let varargs = many(&matches, &"strings");
    match matches.subcommand() {
        (name, Some(m)) => match name {
            "code" => Code(req(m, &"cmd"), many(m, &"args")),
            "data" => Data(req(m, &"url"), opt(m, &"destination")),
            _ => panic!("No such subcommand: {}", name),
        },
        _ if execute && shell => match varargs.split_first() {
            Some((cmd, args)) => ExecuteShell(cmd.clone(), args.to_vec()),
            _ => panic!("No file specified."),
        },
        _ if execute => match varargs.split_first() {
            Some((cmd, args)) => Execute(cmd.clone(), args.to_vec()),
            _ => panic!("No file specified."),
        },
        _ if shell => match varargs.split_first() {
            Some((cmd, args)) if args.len() == 0 => Shell(cmd.clone()),
            _ => panic!("No file specified."),
        },
        _ => Filter(varargs),
    }
}

#[derive(Debug)]
enum Mode {
    Filter(Vec<String>),
    Shell(String),
    Execute(String, Vec<String>),
    ExecuteShell(String, Vec<String>),
    Code(String, Vec<String>),
    Data(String, Option<String>),
}

use Mode::*;

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
