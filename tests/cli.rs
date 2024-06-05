use assert_cmd::Command;

mod common;

#[test]
pub fn convert_file() {
    in_temp_dir!({
        fixture!("input.png");
        fixture!("expected_output.svg");

        let mut cmd = Command::cargo_bin("pixel2svg").unwrap();
        cmd.arg("input.png").assert().success();

        assert_files_eq!("input.svg", "expected_output.svg");
    });
}

#[test]
pub fn convert_file_with_explicit_output() {
    in_temp_dir!({
        fixture!("input.png");
        fixture!("expected_output.svg");

        let mut cmd = Command::cargo_bin("pixel2svg").unwrap();
        cmd.arg("input.png")
            .arg("-O")
            .arg("output.svg")
            .assert()
            .success();

        assert_files_eq!("output.svg", "expected_output.svg");
    });
}
