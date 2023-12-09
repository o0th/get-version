use std::{fs, process::ExitCode};

fn main() -> ExitCode {
    let content = match fs::read_to_string("./Cargo.toml") {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error while reading Cargo.toml: {}", error);
            return ExitCode::FAILURE;
        }
    };

    let regex = match regex::Regex::new(r#"^version[ ]*=[ ]*"(\d+.\d+.\d+)"$"#) {
        Ok(regex) => regex,
        Err(error) => {
            eprintln!("Error while parsing regex: {}", error);
            return ExitCode::FAILURE;
        }
    };

    for line in content.lines() {
        if let Some(caps) = regex.captures(line) {
            if let Some(version) = caps.get(1) {
                println!("{}", version.as_str());
                return ExitCode::SUCCESS;
            }
        }
    }

    return ExitCode::FAILURE;
}
