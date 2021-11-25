enum extern_inline:

%: %.rs
	rustc --crate-type lib $<

version:
	rustc --version --verbose
