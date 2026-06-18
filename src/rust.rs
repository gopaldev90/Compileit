use std:: {
    path:: {
        Path
    },
    process:: {
        Command,
    },
};
use crate::utils;

pub fn compile_project(dfcd: &str, home: &str, extension: &str, prjd: &str, mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = std::path::Path::new(prjd);
    let default_code_dir = std::path::Path::new(dfcd);
    let home_project = std::path::Path::new(home).join(&project_dir);
    let home_src = home_project.join("src");
    let src_dir = default_code_dir.join(&project_dir).join("src");
    if !src_dir.exists(){
        println!("{prjd} project hai hee nhin");
        println!("ismain {dfcd}");
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "src_dir does not exist",)));
    }
    utils::copy_src_files(&src_dir,&home_src,"rs")?;
    for file in ["Cargo.toml","Cargo.lock"] {
        let src = default_code_dir.join(&project_dir).join(file);
        let dst = home_project.join(file);
        if src.exists() {
            utils::copy_file(&src, &dst)?;
        }else {
            println!("{file} nhin mili");
        }
    }
    utils::changdir(&home_project.display().to_string());
    if mode {
        let status = Command::new("cargo")
        .arg("run")
        .status()?;
        std::process::exit(status.code().unwrap_or(1));
    }
    let status = Command::new("cargo").args(["build", "--release"]).status()?;
    if !status.success() {
        return Err("Build failed".into());
    }
    let exe_name = format!("{}.{}", prjd.to_string(), extension);
    let release_dir = home_project.join("target/release");
    let original_bin = release_dir.join(&project_dir);
    let renamed_bin = release_dir.join(&exe_name);
    std::fs::rename(&original_bin, &renamed_bin)?;
    let dest_dir = default_code_dir.join(&project_dir);
    utils::moveto_dfcd(&dest_dir,&renamed_bin,&exe_name)
}

pub fn compile_single(file: &str, default_code_dir: &str, extension: &str)-> Result<(), Box<dyn std::error::Error>> {
    utils::changdir(default_code_dir);
    utils::checkfileastitv(file);
    let mut file_name = file.to_string();
    let new_name = file_name.replace(' ', "_");
    if new_name != file_name {
        std::fs::rename(&file_name, &new_name).expect("rename failed");
        file_name = new_name;
    }
    let status = Command::new("rustc").arg(&file_name).status().expect("failed to run rustc");
    if !status.success() {
        return Err("Build failed".into());
    }
    let bin = file_name.strip_suffix(".rs").unwrap_or(&file_name);
    if !Path::new(bin).is_file() {
        return Err("Executable not found after compilation".into());
    }
    let exe_name = format!("{bin}{extension}");
    std::fs::rename(bin, &exe_name).expect("failed to rename binary");
    Ok(())
}