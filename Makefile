help:
	@echo "Available targets:"
	@echo "    update-expected-tests    - Store output of main commands as expected tests results"

stm32builder = cargo run --bin stm32builder --

# NOTE: Some command are expected to fail.
# Add a '-' prefix to those commands for make to ignore them.
#
# NOTE: Integration tests must be executed on the tests folder.
update-expected-tests:
	cd tests && $(stm32builder) decode stm32f051R8T6 > expected/decode_stm32f051R8T6_stdout.txt
	cd tests && $(stm32builder) parse stm32f051.yaml > expected/parse_stm32f051.yaml_stdout.txt
	cd tests && $(stm32builder) show stm32f051R8T6 stm32f051.yaml > expected/show_stm32f051R8T6_stm32f051.yaml_stdout.txt
	-cd tests && $(stm32builder) show stm32l042R8T6 stm32f051.yaml > expected/show_stm32l042R8T6_stm32f051.yaml_stdout.txt
	cd tests && $(stm32builder) show stm32f051R8T6 stm32f051.yaml device > expected/show_stm32f051R8T6_stm32f051.yaml_device_stdout.txt
	cd tests && $(stm32builder) show stm32f051R8T6 stm32f051.yaml info > expected/show_stm32f051R8T6_stm32f051.yaml_info_stdout.txt
	cd tests && $(stm32builder) show stm32f051R8T6 stm32f051.yaml gpio > expected/show_stm32f051R8T6_stm32f051.yaml_gpio_stdout.txt
	cd tests && $(stm32builder) show stm32f051R8T6 stm32f051.yaml rcc > expected/show_stm32f051R8T6_stm32f051.yaml_rcc_stdout.txt
	cd tests && $(stm32builder) print stm32f051R8T6 stm32f051.yaml > expected/print_stm32f051R8T6_stm32f051.yaml_stdout.txt
	cd tests && $(stm32builder) print stm32f051R8T6 stm32f051.yaml device > expected/print_stm32f051R8T6_stm32f051.yaml_device_stdout.txt
	cd tests && $(stm32builder) print stm32f051R8T6 stm32f051.yaml info > expected/print_stm32f051R8T6_stm32f051.yaml_info_stdout.txt
	cd tests && $(stm32builder) print stm32f051R8T6 stm32f051.yaml gpio > expected/print_stm32f051R8T6_stm32f051.yaml_gpio_stdout.txt
	cd tests && $(stm32builder) print stm32f051R8T6 stm32f051.yaml rcc > expected/print_stm32f051R8T6_stm32f051.yaml_rcc_stdout.txt
	cd tests && $(stm32builder) print stm32f051K8T6 stm32f051.yaml gpio > expected/print_stm32f051K8T6_stm32f051.yaml_gpio_stdout.txt
	cd tests && $(stm32builder) render stm32f051R8T6 stm32f051.yaml template.tmp output.tmp > expected/render_stm32f051R8T6_stm32f051.yaml_template.tmp_output.tmp_stdout.txt
	cd tests && $(stm32builder) update-cargo Cargo-toml stm32f051.yaml > expected/update-cargo_Cargo-toml_stm32f051.yaml_stdout.txt
	cd tests && $(stm32builder) help > expected/help_stdout.txt
