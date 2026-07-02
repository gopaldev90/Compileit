use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
    process::{Command, exit},
};
use crate::utils;

fn cleanup_class_files(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("class") {
            let _ = fs::remove_file(path);
        }
    }
    Ok(())
}

pub fn single(default_code_dir: &std::path::Path) -> io::Result<()> {
    utils::changdir(&default_code_dir);
    let file = fs::read_dir(".")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().and_then(|e| e.to_str()) == Some("java"));

    let mut file = match file {
        Some(f) => f,
        None => {
            println!("{}", env::current_dir()?.display());
            println!("No .java file found.");
            exit(1);
        }
    };

    // replace spaces with underscores
    if let Some(name) = file.file_name().and_then(|n| n.to_str()) {
        let new_name = name.replace(' ', "_");
        if new_name != name {
            fs::rename(&file, &new_name)?;
            file = PathBuf::from(new_name);
        }
    }

    // javac
    let status = Command::new("javac")
        .arg(&file)
        .status()?;

    if !status.success() {
        println!("Compilation failed.");
        exit(1);
    }

    let class_name = file.file_stem().unwrap().to_string_lossy();

    println!("✓✓✓ Running {} ✓✓✓", class_name);

    Command::new("java")
        .arg(&*class_name)
        .status()?;

    cleanup_class_files(".")?;
    println!("Program Finished.");
    Ok(())
}

pub fn project(default_code_dir: &std::path::Path) -> io::Result<()> {
    utils::changdir(&default_code_dir);
    let src = Path::new(default_code_dir).join("src/main/java");
    if !src.is_dir() {
        println!("Source directory not found.");
        exit(1);
    }
    utils::changdir(&src);
    let mut classpath = ".".to_string();
    for entry in fs::read_dir(".")? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("jar") {
            classpath.push(':');
            classpath.push_str(path.to_str().unwrap());
        }
    }

    let status = Command::new("javac")
        .arg("-cp")
        .arg(&classpath)
        .arg("*.java")
        .status()?;

    if status.success() {
        Command::new("java")
            .arg("-cp")
            .arg(&classpath)
            .arg("Main")
            .status()?;

        cleanup_class_files(".")?;
    } else {
        println!("Compilation failed!");
    }

    Ok(())
}
