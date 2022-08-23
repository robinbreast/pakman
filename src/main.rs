use anyhow::{anyhow, Result};
use clap::Parser;
use serde::Deserialize;
//use std::fmt::format;
use chrono::Utc;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::iter::Iterator;
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// config json filepath
    #[clap(short, long)]
    config_filepath: String,
    /// input root directory
    #[clap(short, long)]
    input_dirname: String,
    /// select package
    #[clap(short, long)]
    package_name: String,
    /// output zip filepath
    #[clap(short, long)]
    output_filepath: String,
}

fn default_output_filepath() -> String {
    let cwd = std::env::current_dir().unwrap();
    let now = Utc::now();
    String::from(
        format!(
            "{}-{}.zip",
            cwd.as_path().file_name().unwrap().to_str().unwrap()
        ,
        now.format("%Y%m%d_%H%M"))
    )
}

#[derive(Debug, Deserialize)]
struct Shortcut {
    name: String,
    target: String,
    #[serde(default = "default_cwd")]
    cwd: String,
}

fn default_cwd() -> String {
    String::from("")
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    filepaths: Vec<String>,
    #[serde(default = "default_shortcuts")]
    shortcuts: Vec<Shortcut>,
}

fn default_shortcuts() -> Vec<Shortcut> {
    Vec::new()
}

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
    packages: Vec<Package>,
}

fn main() -> Result<()> {
    // parse command line arguments
    let args = Args::parse();
    let mut output_filepath = args.output_filepath;
    if output_filepath == "default" {
        output_filepath = default_output_filepath();
    }
    // check input root directory
    if !Path::new(&args.input_dirname).exists() {
        return Err(anyhow!("'{}' is not found", &args.input_dirname));
    }
    // open config json file
    let config_file = File::open(args.config_filepath).expect("failed to open config json file");
    // parse config json file
    let config: Config =
        serde_json::from_reader(config_file).expect("failed to parse config json file");
    let mut found = false;
    for package in &config.packages {
        if config.version != "0.0.1" {
            return Err(anyhow!("unsupported version {}", config.version));
        }
        if package.name == args.package_name {
            let filepaths: Vec<String> = build_list(&package, &args.input_dirname);
            // packing files
            pack_files(&args.input_dirname, &filepaths, &output_filepath).unwrap();
            found = true;
        }
    }
    if !found {
        return Err(anyhow!("'{}' is not found", &args.package_name));
    }
    println!("successfully done!");
    Ok(())
}

fn build_list(package: &Package, input_dir: &str) -> Vec<String> {
    let mut filepaths = Vec::new();
    for filepath in &package.filepaths {
        filepaths.push(String::from(Path::new(&input_dir).join(filepath).to_str().unwrap()));
    }
    for shortcut in &package.shortcuts {
        println!("n {} => {} ...", shortcut.name, shortcut.target);
        filepaths.push(create_shortcut(shortcut, input_dir));
    }
    println!("{:#?}", filepaths);
    filepaths
}

fn create_shortcut(shortcut: &Shortcut, input_dir: &str) -> String {
    let fname = String::from(Path::new(&input_dir).join(&shortcut.name).to_str().unwrap());
    let mut file = File::create(&fname).unwrap();
    if shortcut.cwd == "" {
        writeln!(&mut file, "@start \"\" \"{}\"", shortcut.target).unwrap();
    } else {
        writeln!(
            &mut file,
            "@echo off\n\ncd /D %~dp0{}\ncall .\\{}",
            shortcut.cwd,
            Path::new(&shortcut.target)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        )
        .unwrap();
    }
    fname
}

fn pack_files(
    input_dirname: &str,
    filepaths: &Vec<String>,
    output_filename: &str,
) -> zip::result::ZipResult<()> {
    let path = Path::new(output_filename);
    let file = File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    for filepath in filepaths {
        let walkdir = WalkDir::new(filepath);
        let mut it = walkdir.into_iter();
        loop {
            let entry = match it.next() {
                None => break,
                Some(Err(err)) => {
                    println!("warning: {}", err);
                    continue;
                }
                Some(Ok(entry)) => entry,
            };
            let path = entry.path();
            let dir = path.strip_prefix(Path::new(input_dirname)).unwrap();
            if path.is_file() {
                println!("+ {:?} ...", path);
                #[allow(deprecated)]
                zip.start_file_from_path(dir, options)?;
                let mut f = File::open(path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&*buffer)?;
            } else if !dir.as_os_str().is_empty() {
                println!("adding dir {:?}", dir);
                #[allow(deprecated)]
                zip.add_directory_from_path(dir, options)?;
            }
        }
    }
    zip.finish()?;
    Result::Ok(())
}
