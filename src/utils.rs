use std::fs;
use std::io;
use std::path::Path;
use sha2:: {
    Digest
};
use std::io::Read;
use colored::*;

pub fn success(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn error(msg: &str) {
    eprintln!("{}", msg.red().bold());
}

pub fn info(msg: &str) {
    println!("{}", msg.cyan().bold());
}

pub fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), String> {
    fs::create_dir_all(&dst)
        .map_err(|e| e.to_string())?;

    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;

        if ty.is_dir() {
            copy_dir(
                entry.path(),
                dst.as_ref().join(entry.file_name()),
            )?;
        } else {
            fs::copy(
                entry.path(),
                dst.as_ref().join(entry.file_name()),
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
pub fn copy_src_files(src_dir: &std::path::Path, home_src: &std::path::Path, fileendswith: &str)->  Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&home_src)?;
    for entry in std::fs::read_dir(&src_dir)? {
        let entry = entry?;
        if entry.path().extension() == Some(fileendswith.as_ref()) {
            let src = entry.path();
            let dst = home_src.join(entry.file_name());
            copy_file(&src, &dst)?;
        }
    }
    Ok(())
}
pub fn moveto_dfcd(dest_dir: &std::path::Path, renamed_bin: &std::path::Path, exe_name: &str)-> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dest_dir)?;
    let dst = dest_dir.join(&exe_name);
    move_file(&renamed_bin, &dst)?;
    Ok(())
}
pub fn replaceunderscore(file: &str)->std::path::PathBuf {
    let mut file_name = file.to_string();
    let new_name = file_name.replace(' ', "_");
    if new_name != file_name {
        fs::rename(&file_name, &new_name).expect("rename failed");
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
pub fn copy_file(src: &std::path::Path, dst: &std::path::Path)-> Result<(), Box<dyn std::error::Error>> {
    let filename = src.file_name().ok_or("missing filename")?.to_string_lossy().to_string();
    if dst.exists() {
        let srchash = sha256_file(&src)?;
        let dsthash = sha256_file(&dst)?;
        if srchash != dsthash {
            std::fs::copy(src, dst)?;
            info(&format!("overwriten {filename}"));
        }
    }else {
        std::fs::copy(src, dst)?;
        info(&format!("copied {filename}"));
    }
    Ok(())
}
pub fn snippets(name: &str, prakaar: &str) -> std::io::Result<()> {
    fn html_boiler(name: &str) -> io::Result<()> {
        let filename = format!("{}.html", name);

        let content = r#"<!DOCTYPE html>
        <html lang="en">
        <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Minimal</title>

        <style>

        </style>
        </head>
        <body>

        <main class="container">
        <h1>Minimal Page</h1>

        </main>

        </body>
        </html>
        "#;

        fs::write(&filename, content)?;
        println!("{} created Safaltapoorvak", filename);
        Ok(())
    }
    fn cpp_boiler(name: &str) -> io::Result<()> {
        let filename = format!("{}.cpp", name);

        let content = r#"#include "lib/shortcut.hpp"
        #include <stdio.h>
        #include <string.h>
        using namespace std;

        int main() {

        return 0;
        }
        "#;

        fs::write(&filename, content)?;
        println!("{} created Safaltapoorvak", filename);
        Ok(())
    }
    fn java_hoiler(name: &str) -> io::Result<()> {
        let filename = format!("{}.java", name);
        let content = r#"public class Main {

        public static void main(String[] args) {

        }
        }
        "#;

        fs::write(&filename, content)?;
        println!("{} created Safaltapoorvak", filename);
        Ok(())
    }
    match prakaar {
        "html" => html_boiler(name),
        "cpp" => cpp_boiler(name),
        "java" => java_hoiler(name),
        _ => {
            eprintln!("Unknown snippet type: {}", prakaar);
            Ok(())
        }
    }
}
pub fn haiandroid()->bool {
    let os = std::env::consts::OS;
    os == "android"
}
pub fn move_file(src: &Path, dst: &Path) -> io::Result<()> {
    match fs::rename(src, dst) {
        Ok(_) => Ok(()),
        Err(_e) => {
            fs::copy(src, dst)?;
            fs::remove_file(src)
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
pub fn changdir(dest: &str) {
    if let Err(_) = std::env::set_current_dir(&dest) {
        error(&format!("Failed to enter {dest}"));
        std::process::exit(1);
    }
}