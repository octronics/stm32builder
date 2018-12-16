help:
	@echo "Available targets:"
	@echo "    update-expected-tests    - Store output of main commands as expected tests results"

stm32builder = cargo run --bin stm32builder --

# NOTE: Some command are expected to fail.
# Add a '-' prefix to those commands for make to ignore them.
#
# NOTE: Integration tests are executed on the tests folder this is why ther is no
# 'tests/' on expected file names.
update-expected-tests:
	$(stm32builder) decode stm32f051R8T6 > tests/expected/decode_stm32f051R8T6_stdout.txt
	$(stm32builder) parse tests/stm32f051.yaml > tests/expected/parse_stm32f051.yaml_stdout.txt
	$(stm32builder) show stm32f051R8T6 tests/stm32f051.yaml > tests/expected/show_stm32f051R8T6_stm32f051.yaml_stdout.txt
	-$(stm32builder) show stm32l042R8T6 tests/stm32f051.yaml > tests/expected/show_stm32l042R8T6_stm32f051.yaml_stdout.txt
	$(stm32builder) show stm32f051R8T6 tests/stm32f051.yaml device > tests/expected/show_stm32f051R8T6_stm32f051.yaml_device_stdout.txt
	$(stm32builder) show stm32f051R8T6 tests/stm32f051.yaml info > tests/expected/show_stm32f051R8T6_stm32f051.yaml_info_stdout.txt
	$(stm32builder) show stm32f051R8T6 tests/stm32f051.yaml gpio > tests/expected/show_stm32f051R8T6_stm32f051.yaml_gpio_stdout.txt
	$(stm32builder) show stm32f051R8T6 tests/stm32f051.yaml rcc > tests/expected/show_stm32f051R8T6_stm32f051.yaml_rcc_stdout.txt
	$(stm32builder) print stm32f051R8T6 tests/stm32f051.yaml > tests/expected/print_stm32f051R8T6_stm32f051.yaml_stdout.txt
	$(stm32builder) print stm32f051R8T6 tests/stm32f051.yaml device > tests/expected/print_stm32f051R8T6_stm32f051.yaml_device_stdout.txt
	$(stm32builder) print stm32f051R8T6 tests/stm32f051.yaml info > tests/expected/print_stm32f051R8T6_stm32f051.yaml_info_stdout.txt
	$(stm32builder) print stm32f051R8T6 tests/stm32f051.yaml gpio > tests/expected/print_stm32f051R8T6_stm32f051.yaml_gpio_stdout.txt
	$(stm32builder) print stm32f051R8T6 tests/stm32f051.yaml rcc > tests/expected/print_stm32f051R8T6_stm32f051.yaml_rcc_stdout.txt
	$(stm32builder) print stm32f051K8T6 tests/stm32f051.yaml gpio > tests/expected/print_stm32f051K8T6_stm32f051.yaml_gpio_stdout.txt
	$(stm32builder) render stm32f051R8T6 tests/stm32f051.yaml tests/template.tmp tests/output.tmp > tests/expected/render_stm32f051R8T6_stm32f051.yaml_template.tmp_output.tmp_stdout.txt
	cd tests && $(stm32builder) update-cargo Cargo-toml stm32f051.yaml > expected/update-cargo_Cargo-toml_stm32f051.yaml_stdout.txt
	$(stm32builder) help > tests/expected/help_stdout.txt
