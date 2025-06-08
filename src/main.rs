use std::{fs, path::{absolute, Path, PathBuf}};

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Args {
    from_dir: PathBuf,
    to_dir: PathBuf,
    #[arg(long, default_value_t = Regex::new(r"^..").expect("The default regex value is somehow invalid"))]
    id_regex: Regex,
    // TODO: implement prefix chunk size. however, this will impact the default value for id_regex
    // prefix_chunk_size: usize,
    // TODO: implement prefix count. this will also impact the default value for id_regex
    // id_regex should be (prefix_chunk_size * prefix_count) dots
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FileAction {
    Mvp,
    Cpp,
    Lnp,
}

impl FileAction {
    pub fn action<P: AsRef<Path>, Q: AsRef<Path>>(self, source: P, dest: Q) -> std::io::Result<()> {
        match self {
            Self::Mvp => fs::rename(source, dest),
            Self::Cpp => fs::copy(source, dest).map(|_| ()),
            Self::Lnp => fs::soft_link(source, dest),
        }
    }

    pub const fn verb(self) -> &'static str {
        match self {
            Self::Mvp => "move",
            Self::Cpp => "copy",
            Self::Lnp => "symlink",
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let has_capture_group = args.id_regex.captures_len() > 1;

    let file_action= match std::env::args().next() {
        Some(argv0) => {
            match argv0.as_str() {
                "mvp" => FileAction::Mvp,
                "cpp" => FileAction::Cpp,
                "lnp" => FileAction::Lnp,
                _ => FileAction::Mvp,
            }
        },
        None => FileAction::Mvp,
    };

    let from_dir = absolute(args.from_dir)?;
    let to_dir = absolute(args.to_dir)?;

    fs::read_dir(&from_dir)?
        .for_each(|entry| {
            let item = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Cannot read entry: {}", e);
                    return;
                }
            };

            let name = match item.file_name().into_string() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Entry {:?} is not valid Unicode; skipping", e);
                    return;
                }
            };

            let id = if has_capture_group {
                match args.id_regex.captures(&name) {
                    Some(c) => {
                        c.get(1).unwrap().as_str().to_owned()
                    },
                    None => {
                        eprintln!("Entry {} does not match provided regex", name);
                        return;
                    }
                }
            } else {
                match args.id_regex.find(&name) {
                    Some(m) => {
                        m.as_str().to_owned()
                    },
                    None => {
                        eprintln!("Entry {} does not match provided regex", name);
                        return;
                    }
                }
            };

            let first_prefix = &id[0..1];
            let second_prefix = &id[1..2];

            let target_dir = to_dir.join(first_prefix).join(second_prefix);

            match fs::create_dir_all(&target_dir) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Cannot create prefix directory {:?}", target_dir);
                    eprintln!("{}", e);
                    return;
                }
            };

            let source = from_dir.join(&name);
            let dest = target_dir.join(&name);
            match file_action.action(source, dest) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Cannot {} entry {}", file_action.verb(), name);
                    eprintln!("{}", e);
                    return;
                }
            };
        });

    Ok(())
}
