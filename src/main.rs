mod rust;
mod cplusplus;
mod java;
mod utils;
mod golang;
use notify;
use std::sync::mpsc::channel;
use notify::Watcher;
use notify::event:: {
    EventKind,
    ModifyKind
};
enum ParinPrakaar{
    Sahayta,
    Snippetscreated,
    Amanyanayifilename,
    CheckupSuccess,
    CheckupFail,
    PathNotExists,
    SangrahitSafal,
    SangrahitVifal,
    UnknownBhasha,
}
fn checkln(args: &[String], akaar: usize, msg: &str) {
    if args.len() < akaar {
        utils::error(&format!("{}", msg));
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
fn sambhaalo_input(args: &[String], extension: &str) ->ParinPrakaar {
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
                return ParinPrakaar::Sahayta;
            }
            "-newfile" => {
                let inmsg="file ka naam.extension do Jo bnani hai".to_string();
                checkln(&args, 3, &inmsg);
                if let Some((filename, extension)) = args[2].rsplit_once('.') {
                    let _ = utils::snippets(filename, extension);
                    return ParinPrakaar::Snippetscreated;
                } else {
                    utils::error(&inmsg);
                    return ParinPrakaar::Amanyanayifilename;
                }
            }
            "-checkup" => {
                utils::checkup();
                return ParinPrakaar::CheckupSuccess;
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
        return ParinPrakaar::PathNotExists;
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
            if safalparin{
                ParinPrakaar::SangrahitSafal
            }else{
                ParinPrakaar::SangrahitVifal
            }
        }
        utils::BhasaPrakaar::Cplusplus => {
            let safalparin = if pathhaidir {
                let prjd = std::path::Path::new(path.file_name().unwrap());
                cplusplus::compile_project(&prjd, &defalt_code_dir, &home, &extension).is_ok()
            }else {
                cplusplus::compile_single(&filename, &defalt_code_dir, &extension).is_ok()
            };
            if safalparin{
                ParinPrakaar::SangrahitSafal
            }else{
                ParinPrakaar::SangrahitVifal
            }
        }
        utils::BhasaPrakaar::Golang => {
            let safalparin = if pathhaidir {
                golang::compile_project(&defalt_code_dir, &home, extension, &filename).is_ok()
            }else {
                golang::compile_single(&filename, &defalt_code_dir, &extension).is_ok()
            };
            if safalparin{
                ParinPrakaar::SangrahitSafal
            }else{
                ParinPrakaar::SangrahitVifal
            }
        }
        utils::BhasaPrakaar::Java => {
            let safalparin = if pathhaidir {
                java::project(&path).is_ok()
            }else {
                java::single(&defalt_code_dir).is_ok()
            };
            if safalparin{
                ParinPrakaar::SangrahitSafal
            }else{
                ParinPrakaar::SangrahitVifal
            }
        }
        utils::BhasaPrakaar::ReactVite => {
            println!("defalt_code_dir: {}", defalt_code_dir.display());
            run_react_vite_watcher(&path, &home, &filename);
        }
        utils::BhasaPrakaar::Unknown => {
            utils::error(&format!("Cannot find language of this"));
            ParinPrakaar::UnknownBhasha
        }
    };
    safalparin
}
fn run_react_vite_watcher(srcproject_path: &std::path::Path, home: &std::path::Path, prjd: &str,) -> ! {
    let (tx, rx) = channel();
    let dest = home.join(prjd);
    if dest.exists() {
        for entry in std::fs::read_dir(&dest).expect("failed to read destination") {
            let entry = entry.expect("failed to read entry");
            let path = entry.path();
            if path.file_name().and_then(|n| n.to_str()) == Some("node_modules") {
                continue;
            }
            if path.is_dir() {
                std::fs::remove_dir_all(&path).expect("failed to remove directory");
            } else {
                std::fs::remove_file(&path).expect("failed to remove file");
            }
        }
    }
    utils::copy_dir(srcproject_path, &dest).expect("failed to copy project");
    let mut watcher = notify::RecommendedWatcher::new(tx, notify::Config::default()).expect("failed to create watcher");
    watcher.watch(srcproject_path, notify::RecursiveMode::Recursive).expect("failed to watch");
    println!("Watching for changes...");
    for res in rx {
        match res {
            Ok(event) => {
                println!("{:#?}", event);
                match event.kind {
                    // ---------- Rename ----------
                    EventKind::Modify(ModifyKind::Name(notify::event::RenameMode::Both)) => {
                        if event.paths.len() == 2 {
                            let old_rel = event.paths[0].strip_prefix(srcproject_path).unwrap();
                            let new_rel = event.paths[1].strip_prefix(srcproject_path).unwrap();
                            let old_dest = home.join(prjd).join(old_rel);
                            let new_dest = home.join(prjd).join(new_rel);

                            if let Some(parent) = new_dest.parent() {
                                std::fs::create_dir_all(parent).unwrap();
                            }

                            std::fs::rename(&old_dest, &new_dest).unwrap();

                            println!(
                                "Renamed {} -> {}",
                                old_dest.display(),
                                new_dest.display()
                            );
                        }
                    }
                    EventKind::Modify(ModifyKind::Name(_)) => {}
                    EventKind::Create(_)
                    | EventKind::Modify(ModifyKind::Data(_))
                    | EventKind::Modify(ModifyKind::Metadata(_)) => {
                        for changed in &event.paths {
                            if !changed.is_file() {
                                continue;
                            }
                            let relative = match changed.strip_prefix(srcproject_path) {
                                Ok(r) => r,
                                Err(_) => continue,
                            };
                            let dest = home.join(prjd).join(relative);
                            if let Some(parent) = dest.parent() {
                                std::fs::create_dir_all(parent)
                                .expect("failed to create directories");
                            }
                            std::fs::copy(changed, &dest)
                            .expect("failed to copy");
                            println!("Copied -> {}", dest.display());
                        }
                    }
                    // ---------- Remove ----------
                    EventKind::Remove(_) => {
                        for changed in &event.paths {
                            let relative = match changed.strip_prefix(srcproject_path) {
                                Ok(r) => r,
                                Err(_) => continue,
                            };
                            let dest = home.join(prjd).join(relative);
                            if dest.is_file() {
                                let _ = std::fs::remove_file(&dest);
                                println!("Removed file -> {}", dest.display());
                            } else if dest.is_dir() {
                                let _ = std::fs::remove_dir_all(&dest);
                                println!("Removed directory -> {}", dest.display());
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => eprintln!("Watch error: {e}"),
        }
    }
    unreachable!("watcher channel closed");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let extension: String;
    let haiandroid = utils::haiandroid();
    let hailinux = std::env::consts::OS == "linux";
    if !(hailinux || haiandroid) {
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
    let (unit, lga) = utils::generalise(aarambh.elapsed().as_micros().into());
    let exitcode: i32=match safalparin {
        ParinPrakaar::SangrahitVifal => {
            utils::error("sangrahit vifal");
            1
        }
        ParinPrakaar::SangrahitSafal => {
            utils::success("sangrahit safaltapoorvak");
            utils::info(&format!("samay lga({}): {:.2}", unit, lga));
            0
        }
        ParinPrakaar::Amanyanayifilename=>1,
        ParinPrakaar::CheckupFail=>1,
        ParinPrakaar::PathNotExists=>1,
        ParinPrakaar::UnknownBhasha=>1,
        _ =>0
    };
    std::process::exit(exitcode);
}