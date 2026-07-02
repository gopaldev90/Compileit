
use std::{
    process::{Command},
};
use crate::utils;

pub fn compile_single(file: &str,default_code_dir: &std::path::Path,extension: &str,) -> Result<(), String> {
    println!("Using path: {}", default_code_dir.display());
    utils::changdir(&default_code_dir);
    utils::checkfileastitv(file);
    let file_path =utils::replaceunderscore(&file);
    let flags_file = std::path::Path::new(default_code_dir).join("flags.txt");
    if !flags_file.exists(){
        let _=std::fs::write(&flags_file,"#ye file compileit ne bnayi. yahan flags likho\n");
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
    let output_name = file_path.with_extension(extension);
    // clang++ command
    if let Ok(status) = Command::new("clang++").args(&flags).arg(&file_path).arg("-o").arg(&output_name).status(){
        if !status.success() {
            return Err("Compilation failed.".to_string());
        }
        Ok(())
    }else{
        return Err("nhin mila status".to_string());
    }
}

pub fn compile_project(prjd: &std::path::Path,default_code_dir: &std::path::Path,home: &std::path::Path,extension: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = std::path::Path::new(prjd);
    println!("Using path: {}", default_code_dir.display());
    utils::changdir(&default_code_dir);
    //utils::checkfileastitv(file);
    let home_project = std::path::Path::new(home).join(&project_dir);
    let src_dir = std::path::Path::new(&default_code_dir).join(&project_dir);
    let mainfilename=format!("{}.cpp",prjd.display());
    let mainfilepath=src_dir.join(&mainfilename);
    if !src_dir.exists(){
        println!("{} project hai hee nhin",prjd.display());
        println!("ismain {}",default_code_dir.display());
        return Err("src_dir does not exist".to_string().into());
    }
    if !mainfilepath.exists(){
        utils::error(&format!("{} nhin mili {}",mainfilename,src_dir.display()));
        return Err("mainfilename does not exist".to_string().into());
    }
    let file_path =utils::replaceunderscore(&prjd.display().to_string()).join(&mainfilename);
    let flags_file = src_dir.join("flags.txt");
    if !flags_file.exists(){
        let _=std::fs::write(&flags_file,"#ye file compileit ne bnayi. yahan flags likho\n");
    }
    let flags = std::fs::read_to_string(flags_file).map(|content| {content.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).map(String::from).collect::<Vec<_>>()}).unwrap_or_default();
    let output_name = format!("{}.{}",prjd.display(),extension);
    utils::copy_dir(&src_dir,&home_project)?;
    utils::changdir(&home);
    // clang++ command
    if let Ok(status) = Command::new("clang++").args(&flags).arg(&file_path).arg("-o").arg(&output_name).status(){
        let dest_dir=src_dir;
        if !status.success() {
            return Err("Compilation failed.".to_string().into());
        }
        let rnm=std::path::Path::new(&output_name);
        if let Err(e)=utils::moveto_dfcd(&dest_dir,&rnm,&output_name){
            return Err(e);
        }
        std::fs::remove_dir_all(home_project)?;
        Ok(())
    }else{
        return Err("nhin mila status".to_string().into());
    }
}

