# Run code coverage
cov:
	cargo +nightly llvm-cov test --doctests --no-fail-fast --locked

# Generate coverage file for VS Code
cov_code:
	cargo +nightly llvm-cov test --doctests --no-fail-fast --locked --lcov --output-path lcov.info

cov_html:
	cargo +nightly llvm-cov test --doctests --no-fail-fast --locked --html
	open target/llvm-cov/html/index.html

test:
	cargo nextest run --no-fail-fast

llvm_lines:
	cargo llvm-lines --all-features --sort lines --color always

doc:
	RUSTDOCFLAGS="--html-in-header katex_header.html" cargo doc --no-deps
