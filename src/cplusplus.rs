use crate::utils;

fn getflags(default_code_dir: &std::path::Path)->Vec<String> {
    let flags_file = default_code_dir.join("flags.txt");
    if !flags_file.exists() {
        let _ = std::fs::write(&flags_file, "#ye file compileit ne bnayi. yahan flags likho\n");
    }
    let flags = std::fs::read_to_string(flags_file)
    .map(|content| {
        content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
    return flags;
}
pub fn compile_single(file: &str, default_code_dir: &std::path::Path, extension: &str,) -> Result<(), String> {
    utils::changdir(&default_code_dir);
    utils::checkfileastitv(file);
    let file_path = utils::replaceunderscore(&file);
    let flags = getflags(default_code_dir);
    let output_name = file_path.with_extension(extension);
    if let Ok(status) = std::process::Command::new("clang++").args(&flags).arg(&file_path).arg("-o").arg(&output_name).status() {
        if !status.success() {
            return Err("Compilation failed.".to_string());
        }
        Ok(())
    }else {
        return Err("nhin mila status".to_string());
    }
}

pub fn compile_project(prjd: &std::path::Path, default_code_dir: &std::path::Path, home: &std::path::Path, extension: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = std::path::Path::new(prjd);
    utils::changdir(&default_code_dir);
    let home_project = std::path::Path::new(home).join(&project_dir);
    let src_dir = &default_code_dir.join(&project_dir);
    let mainfilename = format!("{}.cpp", prjd.display());
    let mainfilepath = src_dir.join(&mainfilename);
    let extracomlib=std::path::PathBuf::from("/storage/emulated/0/Codes/cplusplus/lib");
    utils::checksrcexists(&src_dir,&default_code_dir,&prjd)?;
    if !mainfilepath.exists() {
        utils::error(&format!("{} nhin mili {}", mainfilename, src_dir.display()));
        return Err("mainfilename does not exist".to_string().into());
    }
    let output_name = std::path::PathBuf::from(format!("{}.{}", prjd.display(), extension));
    let outputbinpath=src_dir.join(&output_name);
    let file_path = utils::replaceunderscore(&prjd.display().to_string()).join(&mainfilename);
    let flags = getflags(src_dir);
    let mut copyganna = utils::copy_dir(&src_dir, &home_project)?;
    if extracomlib.exists(){
        copyganna += utils::copy_dir(&extracomlib,&home_project.join(extracomlib.file_name().unwrap()))?;
    }
    if copyganna==0&&outputbinpath.exists(){
        utils::info("nothing changed. not building binary");
        return Ok(());
    }
    utils::changdir(&home);
    let status = std::process::Command::new("clang++").args(&flags).arg(&file_path).arg("-o").arg(&output_name).status()?;
    if !status.success() {
        return Err("Compilation failed.".to_string().into());
    }
    println!("{}",outputbinpath.display());
    utils::moveto_dfcd(&outputbinpath, &output_name)
}