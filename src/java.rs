
use crate::utils;

fn cleanup_class_files(dir: &std::path::PathBuf) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("class") {
            let _ = std::fs::remove_file(path);
        }
    }
    Ok(())
}

pub fn single(default_code_dir: &std::path::Path) -> std::io::Result<()> {
    utils::changdir(&default_code_dir);
    let file = std::fs::read_dir(".")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.extension().and_then(|e| e.to_str()) == Some("java"));

    let mut file = match file {
        Some(f) => f,
        None => {
            println!("{}", std::env::current_dir()?.display());
            println!("No .java file found.");
            std::process::exit(1);
        }
    };

    // replace spaces with underscores
    if let Some(name) = file.file_name().and_then(|n| n.to_str()) {
        let new_name = name.replace(' ', "_");
        if new_name != name {
            std::fs::rename(&file, &new_name)?;
            file = std::path::PathBuf::from(new_name);
        }
    }

    // javac
    let status = std::process::Command::new("javac")
        .arg(&file)
        .status()?;

    if !status.success() {
        println!("Compilation failed.");
        std::process::exit(1);
    }

    let class_name = file.file_stem().unwrap().to_string_lossy();

    println!("✓✓✓ Running {} ✓✓✓", class_name);

    std::process::Command::new("java")
        .arg(&*class_name)
        .status()?;

    cleanup_class_files(&std::path::PathBuf::from("."))?;
    println!("Program Finished.");
    Ok(())
}

pub fn project(default_code_dir: &std::path::Path) -> std::io::Result<()> {
    let src = default_code_dir.join("src/main/java");
    let lib = default_code_dir.join("lib");
    if !src.is_dir() {
        println!("Source directory not found.");
        std::process::exit(1);
    }
    // Build classpath
    let mut classpath = ".".to_string();
    if lib.is_dir() {
        for entry in std::fs::read_dir(&lib)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("jar") {
                classpath.push(':');
                classpath.push_str(path.to_str().unwrap());
            }
        }
    }
    // Compile
    let mut javac = std::process::Command::new("javac");
    javac.current_dir(&src);
    javac.arg("-cp").arg(&classpath);
    for entry in std::fs::read_dir(&src)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("java") {
            javac.arg(path.file_name().unwrap());
        }
    }
    let status = javac.status()?;
    if !status.success() {
        println!("Compilation failed!");
        return Err(std::io::Error::other("javac failed"));
    }
    // Run
    let status = std::process::Command::new("java")
        .current_dir(&src)
        .arg("-cp")
        .arg(&classpath)
        .arg("Main")
        .status()?;
    if !status.success() {
        println!("Execution failed!");
    }
    cleanup_class_files(&src)?;
    Ok(())
}

