mod rust;
mod cplusplus;
mod java;
mod utils;
mod golang;
use serde_json;

fn checkln(args: &Vec<String>, akaar: usize, msg: &str) {
    if args.len() < akaar {
        utils::error(&format!("{}", msg));
        utils::error("run help for details");
        std::process::exit(1);
    }
}

fn sahayta() {
    println!("=== Code Runner Help ===");
    println!();
    println!("Usage:");
    println!("  tool <workdir> <mode> [options]");
    println!();

    println!("Modes:");
    println!("  rus <project/file> [d] [c]");
    println!("    Compile Rust project/file");
    println!("    d = debug mode (optional)");
    println!("    Example: rus myproj d");
    println!();

    println!("  gol <project.go>");
    println!("    Compile & run file/directory");
    println!("    Example: myproj ");
    println!();

    println!("  cpp <file.cpp>");
    println!("    Compile & run C++ file/directory");
    println!("    Example: cpp test.cpp");
    println!();

    println!("  jav e");
    println!("    Compile & run single Main.java");
    println!("    Example: jav e");
    println!();

    println!("  jav p <project>");
    println!("    Compile & run Java project");
    println!("    Example: jav p myjavaproj");
    println!();

    println!("  newfile <name.ext>");
    println!("    Create new file with snippet");
    println!("    Example: newfile main.rs");
    println!();

    println!("  help");
    println!("    Show this help again");
    println!();
}

fn sambhaalo_input(args: Vec<String>, home: &std::path::Path, defalt_code_dir: &mut std::path::PathBuf, extension: &str) ->i32{
    let mode=&args[1];
    let safalparin = match mode.as_str() {
        "rus" => {
            checkln(&args, 3, "project ya filename do");
            utils::changdir(&defalt_code_dir);
            let filename=&args[2];
            let path = std::path::Path::new(&defalt_code_dir).join(&filename);
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
        "cpp" => {
            checkln(&args, 3, "filename do Jo compile karni hai");
            let filename = &args[2];
            utils::changdir(&defalt_code_dir);
            let path = std::path::Path::new(&defalt_code_dir).join(&filename);
            let safalparin =if path.is_dir() {
                let prjd=std::path::Path::new(path.file_name().unwrap());
                cplusplus::compile_project(&prjd,&defalt_code_dir,&home,&extension).is_ok()
            }else{
                cplusplus::compile_single(&filename.to_string(), &defalt_code_dir, &extension).is_ok()
            };
            i32::from(safalparin)
        }
        "gol" => {
            checkln(&args, 3, "project ya filename do");
            utils::changdir(&defalt_code_dir);
            let filename=&args[2];
            let path = std::path::Path::new(&defalt_code_dir).join(&filename);
            let safalparin=if path.is_dir() {
                golang::compile_project(&defalt_code_dir, &home, extension, &filename).is_ok()
            }else {
                golang::compile_single(&filename, &defalt_code_dir, &extension).is_ok()
            };
            i32::from(safalparin)
        }
        "jav" => {
            checkln(&args, 3, "project dir naam do");
            utils::changdir(&defalt_code_dir);
            let ptype=&args[2];
            let safalparin=if ptype == "e" {
                java::single(&defalt_code_dir).is_ok()
            }else if ptype == "p" {
                checkln(&args, 4, "project dir naam do");
                let prjd = args[3].clone();
                *defalt_code_dir = defalt_code_dir.join(&prjd);
                java::project(&defalt_code_dir).is_ok()
            }else {
                utils::error(&format!("project type sirf e(ekfile) ya p(project) do."));
                utils::error(&format!("diya: {ptype}"));
                return 2;
            };
            i32::from(safalparin)
        }
        "newfile" => {
            checkln(&args, 3, "file ka naam.extension do Jo bnani hai");
            if let Some((filename, prakaar)) = args[2].rsplit_once(".") {
                let _ = utils::snippets(&filename, &prakaar);
                4
            }else {
                3
            }
        }
        "help" => {
            sahayta();
            5
        }
        _ => {
            utils::error(&format!("agyaat mode: {mode}"));
            6
        }
    };
    return safalparin;
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.push("₹".to_string());
    let mut defalt_code_dir=std::env::current_dir().unwrap();
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
    let exitcode:i32;
    let homestr=std::env::var("HOME").expect("home not set");
    let home= std::path::Path::new(&homestr);
    let configpath = std::path::Path::new(&home).join("compileit_config.json");
    if !configpath.exists() {
        utils::error("config file need at ~/compileit_config.json");
        exitcode=1;
    }else if let Ok(file) = std::fs::File::open(&configpath) {
        let reader = std::io::BufReader::new(file);
        let data: serde_json::Value = serde_json::from_reader(reader).unwrap();
        if haiandroid {
            extension = data["extension"]["android"].as_str().unwrap().to_string();
        }else {
            extension = data["extension"]["linux"].as_str().unwrap().to_string();
        }
        checkln(&args, 2, "koi mode do");
        let aarambh = std::time::Instant::now();
        let safalparin=sambhaalo_input(args, home, &mut defalt_code_dir, &extension);
        let lga = aarambh.elapsed().as_secs_f64();
        match safalparin{
            0=>{
                utils::error("sangrahit vifal");
            }
            1=>{
                utils::success("sangrahit safaltapoorvak");
                utils::info(&format!("samay lga(seconds): {:.2}", lga));
            }
            _=>{
                
            }
        }
        exitcode=1-i32::from([1,4,5,6].contains(&safalparin));
    }else {
        utils::error("cant open config file");
        exitcode=1;
    }
    std::process::exit(exitcode);
}