mod fs;

use regex::Regex;

static DEFAULTDIR: &str = "./";
static DEFAULTSRC: &str = "(.*)";
static DEFAULTDST: &str = "$1";
static NOPAREN: &str = "(.*)\\(.*\\)(.*)";
static NOSQUARE: &str = "(.*)\\[.*\\](.*)";
static NOPSDST: &str = "$1$2";
static UNIXIFY: &str = "' +";
static REPLACE: &str = "_";

pub struct Args {
    pub verbose: bool,
    pub pretend: bool,
    pub recurse: bool,
    pub copy: bool,
    pub src: String,
    pub dst: String,
    pub replace: bool,
    pub nosquare: bool,
    pub noparen: bool,
    pub unixify: bool,
    pub rootdir: String,
    pub targetdir: String,
}

impl Args {
    pub fn new() -> Args {
        Args {
            verbose: false,
            pretend: false,
            recurse: false,
            copy: false,
            src: String::from(DEFAULTSRC),
            dst: String::from(DEFAULTDST),
            replace: false,
            nosquare: false,
            noparen: false,
            unixify: false,
            rootdir: String::from(DEFAULTDIR),
            targetdir: String::from(DEFAULTDIR),
        }
    }
}

fn has_one_shortcut(args: &Args) -> bool {
    return count_shortcuts(args) == 1;
}

fn has_several_shortcuts(args: &Args) -> bool {
    return count_shortcuts(args) > 1;
}

fn count_shortcuts(args: &Args) -> u32 {
    let mut count = 0;
    if args.noparen {
        count += 1;
    }
    if args.nosquare {
        count += 1;
    }
    if args.unixify {
        count += 1;
    }
    return count;
}

pub fn validate_args(args: &mut Args) -> bool {
    if !fs::exists_dir(&args.rootdir) {
        return false;
    }
    if !fs::exists_dir(&args.targetdir) {
        return false;
    }
    if args.pretend && args.verbose {
        eprintln!(
            "Warning: --pretend and --verbose activated at the same time. Script may be huge!"
        )
    }
    // cannot use two shortcuts together
    if has_several_shortcuts(args) {
        eprintln!("Error: cannot specify two or more shortcuts together (--nosquare, --noparen, --unixify)");
        return false;
    }
    if has_one_shortcut(args) {
        // cannot use src or dst with shortcuts
        if args.src != DEFAULTSRC || args.dst != DEFAULTDST {
            eprintln!(
                "Error: cannot specify both a shortcut and a regex (--nosquare, --noparen, --unixify and --src, --dst)"
            );
            return false;
        }
        if args.noparen || args.nosquare {
            if args.replace {
                eprintln!("Warning: cannot specify --replace with --nosquare or --noparen (removing --replace)");
                args.replace = false;
            }
            if args.nosquare {
                args.src = String::from(NOSQUARE);
                args.dst = String::from(NOPSDST);
            } else if args.noparen {
                args.src = String::from(NOPAREN);
                args.dst = String::from(NOPSDST);
            }
        } else if args.unixify {
            args.replace = true;
            args.src = String::from(UNIXIFY);
            args.dst = String::from(REPLACE);
        }
    }
    if args.replace {
        if args.dst.len() != args.src.len() && args.dst.len() != 1 {
            eprintln!(
                "Error: destination should have either 1 character or be the same length as source"
            );
            return false;
        }
    } else {
        match Regex::new(&args.src) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Error: cannot compile regex: {:?}", error);
                return false;
            }
        };
    }
    return true;
}

pub fn run(args: &Args) {
    fs::process_dir(args);
}
