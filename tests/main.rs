use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

macro_rules! assert_stdout {
    ( $name:ident: $( $arg:tt )* ) =>
    {
        #[test]
        fn $name() {
            let file: String = [ "tests/expected/", $( $arg, "_", )* "stdout.txt" ].iter().flat_map(|s| s.chars()).collect();
            let expected = BufReader::new(File::open(&file).expect(&format!("failed to open the '{}' output file", &file)));

            // Use cargo run to not having to store the binary path
            let mut child = Command::new("cargo")
                .arg("run")
                .arg("--")
                $( .arg($arg) )*
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .current_dir("tests")
                .spawn()
                .expect("failed to execute the main binary");
            child.wait().expect("command wasn't running");

            let output = BufReader::new(child.stdout.expect("failed to get the stdout"));

            for (expected, output) in expected.lines().zip(output.lines()) {
                assert_eq!(expected.unwrap(), output.unwrap());
            }
        }
    };
}

assert_stdout!(main_have_an_help_command: "help");
assert_stdout!(main_can_decode_a_device_id: "decode" "stm32f051R8T6" );
