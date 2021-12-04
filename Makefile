.PHONY: test-pmatch
test-pmatch: tests/data/gen/pmatch-streets-fr.hfst
	cargo run --release --bin hfst-pmatch -- $< --input tests/data/streets-fr-input.txt --output tests/data/gen/streets-fr-output.txt

tests/data/gen/pmatch-%.hfst: tests/data/pmatch-%.txt
	cargo run --release --bin hfst-pmatch2fst -- < $< > $@
