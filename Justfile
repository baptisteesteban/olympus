documentation: install-mdbook cargo-doc
	mkdir public
	mdbook build doc
	mv doc/olympus-book/* public
	mv target/doc/ public/api

install-mdbook:
	mdbook >/dev/null 2>&1 && echo "mdbook found" || { echo "Installing mdbook"; cargo install mdbook; }

cargo-doc:
	cargo doc -p olympus --no-deps --lib
	cargo doc -p tos-dt --no-deps --lib