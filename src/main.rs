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
fn sambhaalo_input(args: &[String], defalt_code_dir: &mut std::path::PathBuf, extension: &str) ->i32 {
    checkln(&args, 2, "koi arg do");
    let homestr = std::env::var("HOME").expect("home not set");
    let home = std::path::Path::new(&homestr);
    utils::changdir(&defalt_code_dir);
    {
        let mode = args[1].as_str();
        if mode == "-help" || mode == "-newfile" {
            let safalparin: i32;
            if mode == "-help" {
                sahayta();
                safalparin = 5;
            }else {
                checkln(&args, 3, "file ka naam.extension do Jo bnani hai");
                if let Some((filename, prakaar)) = args[2].rsplit_once(".") {
                    let _ = utils::snippets(&filename, &prakaar);
                    safalparin = 4;
                }else {
                    safalparin = 3;
                }
            }
            return safalparin;
        }
    }
    let filename=&args[1];
    let path = std::path::Path::new(&defalt_code_dir).join(&filename);
    if !path.exists() {
        println!("{} hai hee nhin", filename);
        println!("ismain {}", defalt_code_dir.display());
        return 0;
    }
    let mode = utils::decide_project_type(&path).unwrap();
    let safalparin = match mode {
        utils::BhasaPrakaar::Rust => {
            let safalparin = if path.is_dir() {
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
            let safalparin = if path.is_dir() {
                let prjd = std::path::Path::new(path.file_name().unwrap());
                cplusplus::compile_project(&prjd, &defalt_code_dir, &home, &extension).is_ok()
            }else {
                cplusplus::compile_single(&filename.to_string(), &defalt_code_dir, &extension).is_ok()
            };
            i32::from(safalparin)
        }
        utils::BhasaPrakaar::Golang => {
            let safalparin = if path.is_dir() {
                golang::compile_project(&defalt_code_dir, &home, extension, &filename).is_ok()
            }else {
                golang::compile_single(&filename, &defalt_code_dir, &extension).is_ok()
            };
            i32::from(safalparin)
        }
        utils::BhasaPrakaar::Java => {
            let safalparin = if !path.is_dir() {
                java::single(&defalt_code_dir).is_ok()
            }else {
                java::project(&path).is_ok()
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

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.push("₹".to_string());
    let mut defalt_code_dir = std::env::current_dir().unwrap();
    let extension: String;
    utils::changdir(&defalt_code_dir);
    args.pop();
    let haiandroid = utils::haiandroid();
    let hailinux = std::env::consts::OS == "linux";
    let hailinuxkernal = hailinux || haiandroid;
    if !hailinuxkernal {
        utils::error("anjaan os");
        std::process::exit(1);
    }
    let exitcode: i32;
    if haiandroid {
        extension = "axe".to_string();
    }else {
        extension = "mixe".to_string();
    }
    let aarambh = std::time::Instant::now();
    let safalparin = sambhaalo_input(&args, &mut defalt_code_dir, &extension);
    let lga = aarambh.elapsed().as_secs_f64();
    match safalparin {
        0 => {
            utils::error("sangrahit vifal");
        }
        1 => {
            utils::success("sangrahit safaltapoorvak");
            utils::info(&format!("samay lga(seconds): {:.2}", lga));
        }
        _ => {}
    }
    exitcode = 1-i32::from([1, 4, 5, 6].contains(&safalparin));
    std::process::exit(exitcode);
}