mod rmv;
use argparse::{ArgumentParser, Store, StoreTrue};
use rmv::Args;
use std::process;

fn main() {
    let mut args = Args::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("rmv: regex mv, use regular expressions to rename / move files");
        ap.refer(&mut args.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "verbose mode");
        ap.refer(&mut args.pretend).add_option(
            &["-p", "--pretend"],
            StoreTrue,
            "pretend, redirect to generate a script",
        );
        ap.refer(&mut args.recurse).add_option(
            &["-R", "--recurse"],
            StoreTrue,
            "recurse subdirectories",
        );
        ap.refer(&mut args.copy).add_option(
            &["-c", "--copy"],
            StoreTrue,
            "copy instead of renaming / moving",
        );
        ap.refer(&mut args.src).add_option(
            &["-s", "--src"],
            Store,
            "source regex or characters (more on https://docs.rs/regex/#syntax)",
        );
        ap.refer(&mut args.dst).add_option(
            &["-d", "--dst"],
            Store,
            "replacement regex or character(s) (more on https://docs.rs/regex/#syntax)",
        );
        ap.refer(&mut args.replace).add_option(
            &["-r", "--replace"],
            StoreTrue,
            "replace mode: src contains the characters to replace, \
            dst their replacement, either one single character (replaces all chars from src) \
            or as many characters as src (each one replacing its counterpart in src)",
        );
        ap.refer(&mut args.nosquare).add_option(
            &["-a", "--nosquare"],
            StoreTrue,
            "remove square brackets and their contents (shortcut to --src \"(.*)\\[.*\\](.*)\" --dst \"\\$1\\$2\")",
        );
        ap.refer(&mut args.noparen).add_option(
            &["-e", "--noparen"],
            StoreTrue,
            "remove parenthesis and their contents (shortcut to --src \"(.*)\\(.*\\)(.*)\" --dst \"\\$1$2\"))",
        );
        ap.refer(&mut args.unixify).add_option(
            &["-u", "--unixify"],
            StoreTrue,
            "\"unixify\" the result (shortcut to --replace --src \"' +\" --dst \"_\")",
        );
        ap.refer(&mut args.rootdir).add_argument(
            "rootdir",
            Store,
            "root directory to handle (default ./)",
        );
        ap.refer(&mut args.targetdir).add_argument(
            "targetdir",
            Store,
            "target directory to move / copy the files to (default ./)",
        );
        ap.parse_args_or_exit();
    }

    if !rmv::validate_args(&mut args) {
        process::exit(1);
    }
    rmv::run(&args);
}
