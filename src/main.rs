mod rust;
mod cplusplus;
mod java;
mod utils;
mod golang;
fn checkln(args: &[String], akaar: usize, msg: &str) {
    if args.len() < akaar {
        utils::error(&format!("{}", msg));
        utils::error("run help for details");
        utils::info("run compileit -help");
        std::process::exit(1);
    }
}

fn sahayta() {
    println!("=== CompileIt ===");
    println!();
    println!("Universal compiler for Rust, C++, Go and Java.");
    println!();
    println!("Usage:");
    println!("  compileit <file|project>");
    println!("  compileit -newfile <name.ext>");
    println!("  compileit -help");
    println!();

    println!("Examples:");
    println!("  compileit main.rs");
    println!("  compileit hello.cpp");
    println!("  compileit server.go");
    println!("  compileit Main.java");
    println!("  compileit my_project");
    println!();

    println!("Features:");
    println!("  • Automatically detects the language.");
    println!("  • Detects whether the input is a file or project.");
    println!("  • Compiles and runs the program.");
    println!("  • Displays compilation time.");
    println!();

    println!("Options:");
    println!("  -help                 Show this help.");
    println!("  -newfile <name.ext>   Create a new source file from a template.");
    println!();

    println!("Supported languages:");
    println!("  Rust (.rs)");
    println!("  C++  (.cpp, .cc, .cxx)");
    println!("  Go   (.go)");
    println!("  Java (.java)");
}
fn sambhaalo_input(args: &[String], extension: &str) ->i32 {
    let homestr = std::env::var("HOME").expect("HOME not set");
    let home = std::path::Path::new(&homestr);
    let mut defalt_code_dir = std::env::current_dir().unwrap();
    utils::changdir(&defalt_code_dir);
    let mut filename = if args.len() <= 1 {
        ".".to_string()
    } else {
        match args[1].as_str() {
            "-help" => {
                sahayta();
                return 5;
            }
            "-newfile" => {
                checkln(&args, 2, "file ka naam.extension do Jo bnani hai");
                if let Some((filename, extension)) = args[1].rsplit_once('.') {
                    let _ = utils::snippets(filename, extension);
                    return 4;
                } else {
                    return 3;
                }
            }
            other => other.to_string(),
        }
    };
    if filename.ends_with("/") {
        filename.pop();
    }
    if filename == "." {
        filename = defalt_code_dir.file_name().unwrap().display().to_string();
        defalt_code_dir = defalt_code_dir.parent().unwrap().to_path_buf();
    }
    let path = defalt_code_dir.join(&filename);
    if !path.exists() {
        println!("{} hai hee nhin", filename);
        println!("ismain {}", defalt_code_dir.display());
        return 0;
    }
    let mode = utils::decide_project_type(&path).unwrap_or(utils::BhasaPrakaar::Unknown);
    let pathhaidir: bool = path.is_dir();
    let safalparin = match mode {
        utils::BhasaPrakaar::Rust => {
            let safalparin = if pathhaidir {
                let debugmod = if let Some(m) = args.get(4) {
                    m == "d" || m == "debug"
                } else {
                    false
                };
                rust::compile_project(&defalt_code_dir, &home, &extension, &filename, debugmod).is_ok()
            } else {
                rust::compile_single(&filename, &defalt_code_dir, &extension).is_ok()
            };
            i32::from(safalparin)
        }
        utils::BhasaPrakaar::Cplusplus => {
            let safalparin = if pathhaidir {
                let prjd = std::path::Path::new(path.file_name().unwrap());
                cplusplus::compile_project(&prjd, &defalt_code_dir, &home, &extension).is_ok()
            }else {
                cplusplus::compile_single(&filename, &defalt_code_dir, &extension).is_ok()
            };
            i32::from(safalparin)
        }
        utils::BhasaPrakaar::Golang => {
            let safalparin = if pathhaidir {
                golang::compile_project(&defalt_code_dir, &home, extension, &filename).is_ok()
            }else {
                golang::compile_single(&filename, &defalt_code_dir, &extension).is_ok()
            };
            i32::from(safalparin)
        }
        utils::BhasaPrakaar::Java => {
            let safalparin = if pathhaidir {
                java::project(&path).is_ok()
            }else {
                java::single(&defalt_code_dir).is_ok()
            };
            i32::from(safalparin)
        }
        utils::BhasaPrakaar::Unknown => {
            utils::error(&format!("Cannot find language of this"));
            6
        }
    };
    safalparin
}

fn generalise(mut v: u128) -> (String, u128) {
    let units = [
        ("Microseconds", 1000),
        ("Milliseconds", 1000),
        ("Seconds", 60),
        ("Minutes", 60),
        ("Hours", 1),
    ];

    let mut idx = 0;
    while idx < units.len() - 1 && v >= units[idx].1 {
        v /= units[idx].1;
        idx += 1;
    }
    (units[idx].0.to_string(), v)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let extension: String;
    let haiandroid = utils::haiandroid();
    let hailinux = std::env::consts::OS == "linux";
    let hailinuxkernal = hailinux || haiandroid;
    if !hailinuxkernal {
        utils::error("anjaan os");
        std::process::exit(1);
    }
    if haiandroid {
        extension = "axe".to_string();
    }else {
        extension = "mixe".to_string();
    }
    println!("compileit version: {}", std::env!("CARGO_PKG_VERSION"));
    let aarambh = std::time::Instant::now();
    let safalparin = sambhaalo_input(&args, &extension);
    let (unit, lga) = generalise(aarambh.elapsed().as_micros().into());
    match safalparin {
        0 => {
            utils::error("sangrahit vifal");
        }
        1 => {
            utils::success("sangrahit safaltapoorvak");
            utils::info(&format!("samay lga({}): {:.2}", unit, lga));
        }
        _ => {}
    }
    let exitcode: i32 = 1-i32::from([1, 4, 5, 6].contains(&safalparin));
    std::process::exit(exitcode);
}