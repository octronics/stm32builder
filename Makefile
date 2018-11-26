help:
	@echo "Available targets:"
	@echo "    update-expected-tests    - Store output of main commands as expected tests results"


# NOTE: Some command are expected to fail.
# Add a '-' prefix to those commands for make to ignore them.
update-expected-tests:
	cargo run -- decode stm32f051R8T6 > tests/expected/decode_stm32f051R8T6_stdout.txt
	cargo run -- show stm32f051R8T6 tests/stm32f051.yaml > tests/expected/show_stm32f051R8T6_stm32f051.yaml_stdout.txt
	-cargo run -- show stm32l042R8T6 tests/stm32f051.yaml > tests/expected/show_stm32l042R8T6_stm32f051.yaml_stdout.txt
	cargo run -- help > tests/expected/help_stdout.txt
