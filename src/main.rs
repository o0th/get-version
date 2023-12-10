fn main() -> std::process::ExitCode {
    let regex = match regex::Regex::new(r#"^version[ ]*=[ ]*"(\d+.\d+.\d+)"$"#) {
        Ok(regex) => regex,
        Err(error) => {
            eprintln!("Error while parsing regex: {}", error);
            return std::process::ExitCode::FAILURE;
        }
    };

    if let Some(lines) = read_pipe::read_pipe() {
        return capture(lines.split("\n"), &regex);
    }

    if let Ok(content) = std::fs::read_to_string("./Cargo.toml") {
        return capture(content.lines(), &regex);
    }

    return std::process::ExitCode::FAILURE;
}

fn capture<'a, I>(lines: I, regex: &regex::Regex) -> std::process::ExitCode
where
    I: Iterator<Item = &'a str>,
{
    for line in lines {
        if let Some(captures) = regex.captures(line) {
            if let Some(version) = captures.get(1) {
                println!("{}", version.as_str());
                return std::process::ExitCode::SUCCESS;
            }
        }
    }

    return std::process::ExitCode::FAILURE;
}
