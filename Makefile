help:
	@echo "Available targets:"
	@echo "    update-expected-tests    - Store output of main commands as expected tests results"


# NOTE: Some command are expected to fail.
# Add a '-' prefix to those commands for make to ignore them.
#
# NOTE: Integration tests are executed on the tests folder this is why ther is no
# 'tests/' on expected file names.
update-expected-tests:
	cargo run -- decode stm32f051R8T6 > tests/expected/decode_stm32f051R8T6_stdout.txt
	cargo run -- parse tests/stm32f051.yaml > tests/expected/parse_stm32f051.yaml_stdout.txt
	cargo run -- show stm32f051R8T6 tests/stm32f051.yaml > tests/expected/show_stm32f051R8T6_stm32f051.yaml_stdout.txt
	-cargo run -- show stm32l042R8T6 tests/stm32f051.yaml > tests/expected/show_stm32l042R8T6_stm32f051.yaml_stdout.txt
	cargo run -- show stm32f051R8T6 tests/stm32f051.yaml device > tests/expected/show_stm32f051R8T6_stm32f051.yaml_device_stdout.txt
	cargo run -- show stm32f051R8T6 tests/stm32f051.yaml info > tests/expected/show_stm32f051R8T6_stm32f051.yaml_info_stdout.txt
	cargo run -- show stm32f051R8T6 tests/stm32f051.yaml gpio > tests/expected/show_stm32f051R8T6_stm32f051.yaml_gpio_stdout.txt
	cargo run -- print stm32f051R8T6 tests/stm32f051.yaml > tests/expected/print_stm32f051R8T6_stm32f051.yaml_stdout.txt
	cargo run -- print stm32f051R8T6 tests/stm32f051.yaml device > tests/expected/print_stm32f051R8T6_stm32f051.yaml_device_stdout.txt
	cargo run -- print stm32f051R8T6 tests/stm32f051.yaml info > tests/expected/print_stm32f051R8T6_stm32f051.yaml_info_stdout.txt
	cargo run -- print stm32f051R8T6 tests/stm32f051.yaml gpio > tests/expected/print_stm32f051R8T6_stm32f051.yaml_gpio_stdout.txt
	cargo run -- help > tests/expected/help_stdout.txt
