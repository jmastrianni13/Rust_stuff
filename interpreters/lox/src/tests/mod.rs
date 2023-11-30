#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string, DirEntry};
    use std::process::Command;

    #[test]
    fn execute_tests() {
        // let cases = read_dir("./src/tests/cases").unwrap();
        let cases = read_dir("/Users/joemastrianni/Desktop/Sandbox/Rust/hello_rust/Rust_stuff/interpreters/lox/src/tests/cases").unwrap();

        let mut errors = vec![];
        for case in cases {
            let case = case.unwrap();
            let name = case.path().display().to_string();
            if name.contains(".swp") {
                continue;
            }
            match run_test(case) {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        if errors.len() > 0 {
            panic!("Errors:\n\n{}", errors.join("\n\n"));
        }
    }

    fn run_test(file: DirEntry) -> Result<(), String> {
        let contents = read_to_string(file.path()).unwrap();
        let lines = contents.split("\n").collect::<Vec<&str>>();

        let mut test_code = vec![];

        let mut idx = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("--- Test") {
                continue;
            }
            if line.starts_with("--- Expected") {
                idx = i;
                break;
            }
            test_code.push(line.clone());
        }

        let mut expected_output = vec![];

        for line in &lines[idx + 1..] {
            if line.len() > 0 {
                expected_output.push(*line);
            }
        }

        let input = test_code.join("\n");

        let output = Command::new("cargo")
            .arg("run")
            .arg("e")
            .arg(input)
            .output()
            .unwrap();

        let lines = std::str::from_utf8(output.stdout.as_slice())
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>();

        if !(lines.len() == expected_output.len() || lines.len() == expected_output.len() + 1) {
            return Err(format!(
                "{:#?}: output length does not match expected output: {} != {}",
                file.file_name(),
                lines.len(),
                expected_output.len(),
            ));
        }

        for (i, expected) in expected_output.iter().enumerate() {
            if lines[i] != (*expected).trim() {
                return Err(format!(
                    "{:?}: {} != {}\nFull output:\n{}",
                    file.file_name(),
                    lines[i],
                    expected,
                    expected_output.join("\n"),
                ));
            }
        }

        return Ok(());
    }
}
