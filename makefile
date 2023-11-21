build:
	@cargo build --release && mv target/release/i18n_compiler ./bin/i18n_compiler_x86 ;
	@cargo build --release --target aarch64-apple-darwin && mv target/aarch64-apple-darwin/release/i18n_compiler ./bin/i18n_compiler_m1;
	@cargo build --target x86_64-pc-windows-gnu --release && mv target/x86_64-pc-windows-gnu/release/i18n_compiler.exe ./bin/i18n_compiler.exe;

root := $(shell pwd)

test-i18n:
	@cd bin && ./i18n_compiler android $(root)/test/i18n.csv $(root)/test/android && ./i18n_compiler ios $(root)/test/i18n.csv $(root)/test/ios && ./i18n_compiler web $(root)/test/i18n.csv $(root)/test/web && cd -;