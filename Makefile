# Generate coverage file for VS Code
cov_code:
	cargo llvm-cov nextest --verbose --no-fail-fast --locked --lcov --output-path lcov.info

# Run code coverage
cov:
	cargo llvm-cov nextest --no-fail-fast --locked
