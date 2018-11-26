help:
	@echo "Available targets:"
	@echo "    update-expected-tests    - Store output of main commands as expected tests results"

update-expected-tests:
	cargo run -- decode stm32f051R8T6 > tests/expected/decode_stm32f051R8T6_stdout.txt
	cargo run -- help > tests/expected/help_stdout.txt
