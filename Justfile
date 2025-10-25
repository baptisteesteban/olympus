import 'sandbox/documentation.just'

documentation: build-doc

build-doc: build-folder build-book cargo-doc

build-folder:
	[ -d "public" ] && rm -rf "public" || true
	mkdir -p public/generated;
	cp -r doc/imgs public/

install-mdbook:
	mdbook >/dev/null 2>&1 && echo "mdbook found" || { echo "Installing mdbook"; cargo install mdbook; }

build-book: install-mdbook generate-sandbox-data
	mdbook build doc
	mv doc/olympus-book/* public

cargo-doc:
	cargo doc -p olympus --no-deps --lib
	cargo doc -p tos-dt --no-deps --lib
	mv target/doc/ public/api