#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string, DirEntry};
    use std::process::Command;

    #[test]
    fn execute_tests() {
        let cases = read_dir("./src/tests/cases").unwrap();

        let mut msgs = vec![];
        let mut errors = vec![];
        for case in cases {
            let case = case.unwrap();
            let name = case.path().display().to_string();
            if name.contains(".swp") {
                continue;
            }
            match run_test(case) {
                Ok(_) => {
                    msgs.push(format!("Running {name:.<50}...ok"));
                }
                Err(msg) => {
                    errors.push(msg);
                    msgs.push(format!("Running {name:.<50}...failed"));
                }
            }
        }

        println!("Ran {} tests", msgs.len());
        msgs.sort();
        for msg in msgs {
            println!("{msg}");
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
            if line.starts_with("// --- Test") {
                continue;
            }
            if line.starts_with("// --- Expected") {
                idx = i;
                break;
            }
            test_code.push(line.clone());
        }

        let mut expected_output = vec![];

        for line in &lines[idx + 1..] {
            if line.len() > 0 {
                let output_line = line.to_string();
                expected_output.push((output_line[3..]).to_string());
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
                "{:#?}: output length does not match expected output: {} != {}\nFull output:\n{}",
                file.file_name(),
                lines.len(),
                expected_output.len(),
                lines.join("\n"),
            ));
        }

        for (i, expected) in expected_output.iter().enumerate() {
            if lines[i] != (*expected).trim() {
                return Err(format!(
                    "{:?}: {} != {}\nFull output:\n{}",
                    file.file_name(),
                    lines[i],
                    expected,
                    lines.join("\n"),
                ));
            }
        }

        return Ok(());
    }
}
