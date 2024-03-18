use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

pub struct Config {
    pub dup_size: String,
    pub src: String,
    pub dst: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 3 {
            return Err("not enough arguments");
        }
        let dup_size = args[1].clone();
        let src = args[2].clone();
        let dst = args[3].clone();

        Ok(Config { dup_size, src, dst })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir(&config.src)?;

    for i in 0..config.dup_size.parse::<u32>().unwrap() {
        let batch_path = format!("batch{}", i);
        fs::create_dir(Path::new(&config.dst).join(batch_path)).expect("Unable to create directory");
    }

    for file in files {
        match file {
            Ok(file) => replicate_file(
                file,
                &config.src.clone(),
                &config.dst.clone(),
                config.dup_size.parse::<u32>().unwrap(),
            ),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}

fn replicate_file(file: fs::DirEntry, src: &str, dst: &str, times: u32) {
    let file_name_base: String = file.file_name().into_string().unwrap();

    for i in 0..times {
        let dst_file_name = replicate_file_name(file_name_base.as_str(), format!(".replicate{}", i).as_str());

        let path = Path::new(dst)
            .join(format!("batch{}", &i).as_str())
            .join(&dst_file_name);

        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let contents = fs::read_to_string(Path::new(src).join(file_name_base.as_str()))
            .unwrap()
            .lines()
            .map(|line| {
                // SPDX
                if line.starts_with("  \"name\":") {
                    replace(line, "name", "replicate", i)
                } else if line.starts_with("  \"documentNamespace\":") {
                    replace(line, "documentNameSpacekey", "replicate", i)
                }
                // CycloneDX
                else if line.starts_with("    \"serialNumber\":") {
                    replace(line, "serialNumber", "replicate", i)
                } else if line.starts_with("    \"version\":") {
                    replace(line, "version", "replicate", i)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        match file.write_all(contents.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

/// Insert provided string after the file base name and before the extensions
/// Example with string being "blah": "ubi9-minimal-9.3-830.json" becomes "ubi9-minimal-9.blah.3-830.json"
fn replicate_file_name(file_name: &str, str: &str) -> String {
    let mut file_name_with_extensions = file_name.split(".");
    let base_file_name: String = file_name_with_extensions.next().unwrap().to_string();

    let mut dst_file_name = base_file_name.clone();
    dst_file_name.push_str(str);

    for extension in file_name_with_extensions {
        dst_file_name.push_str(".");
        dst_file_name.push_str(extension);
    }

    dst_file_name
}

fn replace(line: &str, key: &str, value: &str, index: u32) -> String {
    println!("Amending {}: {}", key, line);
    let mut document_key = line.split(": ");
    document_key.next();
    let remainder = document_key.next().unwrap();

    let new_remainder = remainder.replace("\",", format!("-{}{}\",", value, index).as_str());
    let new_remainder_str = new_remainder.as_str();

    line.replace(remainder, new_remainder_str).to_string()
}
