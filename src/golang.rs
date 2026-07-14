use std:: {
    fs,
    process:: {
        Command
    },
};
use crate::utils;

pub fn compile_project(dfcd: &std::path::Path, home: &std::path::Path, extension: &str, prjd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = std::path::Path::new(prjd);
    let default_code_dir = std::path::Path::new(dfcd);
    let home_dir = std::path::Path::new(home);
    let home_project = std::path::Path::new(home).join(&project_dir);
    let home_src = home_project.clone();
    let src_dir = default_code_dir.join(&project_dir);
    if src_dir.exists() {
        fs::create_dir_all(&home_src)?;
        for entry in fs::read_dir(&src_dir)? {
            let entry = entry?;
            if entry.path().extension() == Some("go".as_ref()) {
                fs::copy(entry.path(), home_src.join(entry.file_name()))?;
            }
        }
    }else {
        println!("{prjd} project hai hee nhin");
        println!("ismain {}",dfcd.display());
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "src_dir does not exist",)));
    }
    utils::changdir(&home_project);
    {
        let file ="go.mod";
        let dst = home_project.join(file);
        if !dst.exists() {
            let status = Command::new("go").args(["mod", "init", prjd]).status()?;
            if !status.success() {
                return Err("Init failed".into());
            }
            let src = default_code_dir.join(&project_dir).join(file);
            if src.exists() {
                utils::copy_file(&src,&dst)?;
            }else {
                println!("{file} nhin mili");
            }
        }
    }
    let status = Command::new("go").args(["build"]).status()?;
    if !status.success() {
        return Err("Build failed".into());
    }
    let exe_name = format!("{}.{}", prjd.to_string(), extension);
    let original_bin = std::path::Path::new(prjd);
    if !original_bin.exists() {
        eprintln!("sangrahit karne ke baad binary nhin mili");
        return Err("Build failed".into());
    }
    let renamed_bin = std::path::Path::new(&exe_name);
    fs::rename(&original_bin, &renamed_bin)?;
    let dest = default_code_dir.join(&project_dir);
    fs::create_dir_all(&dest)?;
    utils::changdir(&home);
    {
        let src = home_dir.join(&project_dir).join(&exe_name);
        let dst = default_code_dir.join(&project_dir).join(&exe_name);
        utils::move_file(&src, &dst)?;
    }
    Ok(())
}
pub fn compile_single(file: &str, default_code_dir: &std::path::Path, extension: &str)->Result<(), Box<dyn std::error::Error>> {
    utils::changdir(default_code_dir);
    utils::checkfileastitv(file);
    let file_name = utils::replaceunderscore(&file);
    let status = Command::new("go").arg("build").arg(&file_name).status().expect("failed to run go build");
    if !status.success() {
        eprintln!("Compilation failed");
        return Err("Build failed".into());
    }
    let mut bin = file_name.clone();
    bin.set_extension(std::env::consts::EXE_EXTENSION);
    if !bin.is_file() {
        eprintln!("Executable not found after compilation");
        return Err("Build failed".into());
    }
    // add extension
    let exe_name = file_name.with_extension(extension);
    std::fs::rename(&bin, &exe_name).expect("failed to rename binary");
    Ok(())
}