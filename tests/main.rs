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
assert_stdout!(main_can_parse_a_device: "parse" "stm32f051.yaml" );
assert_stdout!(main_can_show_a_device: "show" "stm32f051R8T6" "stm32f051.yaml" );
assert_stdout!(main_fail_to_show_unmatch_device: "show" "stm32l042R8T6" "stm32f051.yaml" ); // not real id
assert_stdout!(main_can_show_the_whole_device: "show" "stm32f051R8T6" "stm32f051.yaml" "device" );
assert_stdout!(main_can_show_the_whole_info: "show" "stm32f051R8T6" "stm32f051.yaml" "info" );
assert_stdout!(main_can_show_the_whole_gpio: "show" "stm32f051R8T6" "stm32f051.yaml" "gpio" );
assert_stdout!(main_can_show_the_whole_rcc: "show" "stm32f051R8T6" "stm32f051.yaml" "rcc" );
assert_stdout!(main_can_print_a_device: "print" "stm32f051R8T6" "stm32f051.yaml" );
assert_stdout!(main_can_print_the_whole_device: "print" "stm32f051R8T6" "stm32f051.yaml" "device" );
assert_stdout!(main_can_print_the_info: "print" "stm32f051R8T6" "stm32f051.yaml" "info" );
assert_stdout!(main_can_print_the_gpio: "print" "stm32f051R8T6" "stm32f051.yaml" "gpio" );
assert_stdout!(main_can_print_the_rcc: "print" "stm32f051R8T6" "stm32f051.yaml" "rcc" );
assert_stdout!(main_can_print_the_gpio_for_other_part: "print" "stm32f051K8T6" "stm32f051.yaml" "gpio" );
assert_stdout!(main_can_render_a_template: "render" "stm32f051R8T6" "stm32f051.yaml" "template.tmp" "output.tmp" );
