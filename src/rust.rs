use crate::utils;

pub fn compile_project(default_code_dir: &std::path::Path, home: &std::path::Path, extension: &str, prjd: &str, mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = std::path::Path::new(prjd);
    let project_root = default_code_dir.join(project_dir);
    let home_project = home.join(project_dir);
    let src_dir = project_root.join("src");
    let home_src = home_project.join("src");
    let dest_dir = default_code_dir.join(&project_dir);
    let exe_name = format!("{prjd}.{extension}");
    let release_dir = home_project.join("target").join("release");
    let original_bin = release_dir.join(project_dir);
    let renamed_bin = release_dir.join(&exe_name);
    utils::checksrcexists(&src_dir, &default_code_dir, &std::path::PathBuf::from(prjd))?;
    let mut copyganna: u32 = utils::copy_dir(&src_dir, &home_src)?;
    let outputbinpath = dest_dir.join(&exe_name);
    let cargolofkname = "Cargo.lock";
    for file in ["Cargo.toml",
        cargolofkname] {
        let src = default_code_dir.join(&project_dir).join(file);
        let dst = home_project.join(file);
        if src.exists() {
            let status = utils::copy_file(&src, &dst)?;
            match status {
                utils::CopyState::NoChange => {
                    copyganna += 0;
                }
                _ => {
                    copyganna += 1;
                }
            }
        }else {
            println!("{file} nhin mili");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "file not mili",)));
        }
    }
    if copyganna == 0 && outputbinpath.exists() {
        utils::info("nothing changed. not building binary");
        return Ok(());
    }
    utils::changdir(&home_project);
    if mode {
        let status = std::process::Command::new("cargo")
        .arg("run")
        .status()?;
        std::process::exit(status.code().unwrap_or(1));
    }
    let status = std::process::Command::new("cargo").args(["build", "--release"]).status()?;
    if !status.success() {
        return Err("Build failed".into());
    }
    std::fs::rename(&original_bin, &renamed_bin)?;
    utils::copy_file(&home_project.join(cargolofkname), &dest_dir.join(cargolofkname))?;
    utils::moveto_dfcd(&outputbinpath, &renamed_bin)
}

pub fn compile_single(file: &str, default_code_dir: &std::path::Path, extension: &str)-> Result<(), Box<dyn std::error::Error>> {
    utils::changdir(default_code_dir);
    utils::checkfileastitv(file);
    let mut file_name = file.to_string();
    let new_name = file_name.replace(' ', "_");
    if new_name != file_name {
        std::fs::rename(&file_name, &new_name).expect("rename failed");
        file_name = new_name;
    }
    let status = std::process::Command::new("rustc").arg(&file_name).status().expect("failed to run rustc");
    if !status.success() {
        return Err("Build failed".into());
    }
    let bin = file_name.strip_suffix(".rs").unwrap_or(&file_name);
    if !std::path::Path::new(bin).is_file() {
        return Err("Executable not found after compilation".into());
    }
    let exe_name = format!("{bin}{extension}");
    std::fs::rename(bin, &exe_name).expect("failed to rename binary");
    Ok(())
}