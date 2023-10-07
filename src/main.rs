use std::env;
use cecho::cecho::cecho;

fn main() {
    let mut arguments: Vec<String> = Vec::new();
    for a in env::args().skip(1) /* skip the process name */ {
        arguments.push(a)
    }
    let result = cecho(arguments);

    match result {
        Err(m) => {
            eprintln!("{}", m);
            std::process::exit(1)
        }
        Ok(s) => {
            print!("{}", s);
            std::process::exit(0)
        }
    }
}

/// CLI smoke tests
#[cfg(test)]
mod tests {
    use std::process::Command;
    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::cargo::CommandCargoExt;
    use predicates::prelude::predicate;

    #[test]
    fn require_at_least_2_arguments() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("cecho")?;

        cmd.arg("");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("The minimum number of arguments is 2. The first argument is the format. If no formatting is necessary, use an empty string."));

        Ok(())
    }

    #[test]
    fn print_any_string_by_omitting_the_format() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("cecho")?;

        cmd.arg("").arg("Whatever you want here!");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Whatever you want here!"));

        Ok(())
    }

    #[test]
    fn print_just_brackets_using_2_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("cecho")?;

        cmd.arg("{}").arg("{}");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("{}"));

        Ok(())
    }

    #[test]
    fn print_indexed_specs() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("cecho")?;

        cmd.arg("{%3} {%2} {%1} {%2} {%3}").arg("1").arg("2").arg("3");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("3 2 1 2 3"));

        Ok(())
    }

    #[test]
    fn print_literal_brackets() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("cecho")?;

        cmd.arg(r#"\{{}\}"#).arg("value");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("{value}"));

        Ok(())
    }
}
