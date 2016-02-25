extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate arx;

const USAGE: &'static str = "arx [-f] [-s] <file>";

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



// pub fn main() {
//     let loaded = arx::parser::load("x: &y y\nz: *y");
//     println!("{:?}", loaded);
// }

pub fn main() {
    let spec = Arg::with_name("spec-file").multiple(true);
    let execute = Arg::with_name("execute").short("e");
    let as_shell = Arg::with_name("as-shell").short("s");
    let app = App::new("arx")
                  .version(env!("CARGO_PKG_VERSION"))
                  .version_short("v")
                  .usage(USAGE)
                  .help(HELP.trim())
                  .arg(spec)
                  .arg(execute)
                  .arg(as_shell);
    let matches = app.get_matches();
}
