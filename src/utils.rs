use std::io;
use std::path::Path;
use sha2:: {
    Digest
};
use std::io::Read;
use colored::*;
pub enum CopyState {
    Owerwrite,
    Copy,
    NoChange
}
#[derive(PartialEq, Eq, Debug)]
pub enum BhasaPrakaar {
    Rust,
    Cplusplus,
    Golang,
    Java,
    Unknown
}

pub fn success(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn error(msg: &str) {
    eprintln!("{}", msg.red().bold());
}
pub fn info(msg: &str) {
    println!("{}", msg.cyan().bold());
}
fn lang_from_extension(ext: &str) -> Option<BhasaPrakaar> {
    match ext {
        "rs" => Some(BhasaPrakaar::Rust),
        "cpp" | "cc" | "cxx" => Some(BhasaPrakaar::Cplusplus),
        "go" => Some(BhasaPrakaar::Golang),
        "java" => Some(BhasaPrakaar::Java),
        _ => None,
    }
}

pub fn decide_project_type(searchpath: &Path) -> std::io::Result<BhasaPrakaar> {
    if searchpath.is_file() {
        return Ok(
            searchpath
                .extension()
                .and_then(|e| e.to_str())
                .and_then(lang_from_extension)
                .unwrap_or(BhasaPrakaar::Unknown),
        );
    }
    if searchpath.join("Cargo.toml").exists() {
        return Ok(BhasaPrakaar::Rust);
    }
    if searchpath.join("go.mod").exists() {
        return Ok(BhasaPrakaar::Golang);
    }
    if searchpath.join("pom.xml").exists()
        || searchpath.join("build.gradle").exists()
        || searchpath.join("build.gradle.kts").exists()
    {
        return Ok(BhasaPrakaar::Java);
    }
    if searchpath.join("CMakeLists.txt").exists()
        || searchpath.join("Makefile").exists()
    {
        return Ok(BhasaPrakaar::Cplusplus);
    }
    for entry in std::fs::read_dir(searchpath)? {
        let path = entry?.path();
        if path.is_file() {
            if let Some(lang) = path
                .extension()
                .and_then(|e| e.to_str())
                .and_then(lang_from_extension)
            {
                return Ok(lang);
            }
        }
    }
    recursive_scan(searchpath)
}

fn recursive_scan(dir: &Path) -> std::io::Result<BhasaPrakaar> {
    const SKIP: &[&str] = &[
        "target",
        ".git",
        "node_modules",
        ".idea",
        ".vscode",
        "build",
        "out",
        "bin",
    ];

    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if SKIP.contains(&name) {
                    continue;
                }
            }

            let lang = recursive_scan(&path)?;
            if lang != BhasaPrakaar::Unknown {
                return Ok(lang);
            }
        } else if let Some(lang) = path
            .extension()
            .and_then(|e| e.to_str())
            .and_then(lang_from_extension)
        {
            return Ok(lang);
        }
    }

    Ok(BhasaPrakaar::Unknown)
}

pub fn checksrcexists(src_dir: &std::path::Path, default_code_dir: &std::path::Path, prjd: &std::path::Path)->Result<(), Box<dyn std::error::Error>> {
    if !src_dir.exists() {
        println!("{} project hai hee nhin", prjd.display());
        println!("ismain {}", default_code_dir.display());
        return Err("src_dir does not exist".to_string().into());
    }
    Ok(())
}
pub fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> Result<u32, String> {
    std::fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
    let mut copiganna: u32 = 0;
    for entry in std::fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        if ty.is_dir() {
            copy_dir(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            let src = entry.path();
            let dest = dst.join(entry.file_name());
            if let Ok(status) = copy_file(&src, &dest) {
                match status {
                    CopyState::NoChange => {
                        copiganna += 0;
                    }
                    _ => {
                        copiganna += 1;
                    }
                }
            }
        }
    }
    Ok(copiganna)
}
pub fn moveto_dfcd(dest_dir: &std::path::Path, src: &std::path::Path)-> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = dest_dir.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if move_file(&src, &dest_dir).is_ok() {
        Ok(())
    }else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to move file,")))
    }
}
pub fn replaceunderscore(file: &str)->std::path::PathBuf {
    let mut file_name = file.to_string();
    let new_name = file_name.replace(' ', "_");
    if new_name != file_name {
        std::fs::rename(&file_name, &new_name).expect("rename failed");
        file_name = new_name;
    }
    std::path::Path::new(&file_name).to_path_buf()
}
pub fn sha256_file(path: &std::path::Path) -> io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}
pub fn copy_file(
    src: &std::path::Path,
    dst: &std::path::Path,
) -> Result<CopyState, Box<dyn std::error::Error>> {
    let filename = src.file_name().ok_or("missing filename")?
    .to_string_lossy()
    .to_string();

    if dst.exists() {
        let srchash = sha256_file(src)?;
        let dsthash = sha256_file(dst)?;

        if srchash != dsthash {
            std::fs::copy(src, dst)?;
            info(&format!("overwritten {filename}"));
            Ok(CopyState::Owerwrite)
        } else {
            Ok(CopyState::NoChange)
        }
    } else {
        std::fs::copy(src, dst)?;
        info(&format!("copied {filename}"));
        Ok(CopyState::Copy)
    }
}
pub fn snippets(name: &str, prakaar: &str) -> std::io::Result<()> {
    fn html_boiler(name: &str) -> io::Result<()> {
        let filename = format!("{}.html", name);
        // Embedded template
        let content = include_str!("templates/html.html");
        std::fs::write(&filename, content)?;
        println!("{} created Safaltapoorvak", filename);
        Ok(())
    }
    fn cpp_boiler(name: &str) -> io::Result<()> {
        let filename = format!("{}.cpp", name);
        let content = include_str!("templates/cplusplus.cpp");
        std::fs::write(&filename, content)?;
        println!("{} created Safaltapoorvak", filename);
        Ok(())
    }
    fn java_hoiler(name: &str) -> io::Result<()> {
        let filename = format!("{}.java", name);
        let content = include_str!("templates/java.java");
        std::fs::write(&filename, content)?;
        println!("{} created Safaltapoorvak", filename);
        Ok(())
    }
    match prakaar {
        "html" => html_boiler(name),
        "cpp" => cpp_boiler(name),
        "java" => java_hoiler(name),
        _ => {
            eprintln!("Unknown snippet type: {}", prakaar);
            Err(std::io::Error::other("mainfilename does not exist"))
        }
    }
}
pub fn haiandroid()->bool {
    let os = std::env::consts::OS;
    os == "android"
}
pub fn move_file(src: &Path, dst: &Path) -> io::Result<()> {
    match std::fs::rename(src, dst) {
        Ok(_) => Ok(()),
        Err(_e) => {
            if dst.exists() {
                std::fs::remove_file(dst)?;
            }
            std::fs::copy(src, dst)?;
            std::fs::remove_file(src)
        }
    }
}
pub fn checkfileastitv(file: &str) {
    if !std::path::Path::new(file).is_file() {
        let pwd = std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|_| "<unknown>".to_string());
        error(&format!("Nhin mili: {}", file));
        error(&format!("isme: {}", pwd));
        std::process::exit(1);
    }
}
pub fn changdir(dest: &std::path::Path) {
    if let Err(_) = std::env::set_current_dir(&dest) {
        error(&format!("Failed to enter {}", dest.display()));
        std::process::exit(1);
    }
}