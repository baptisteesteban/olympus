generate-genericity-data: generate-image2d generate-node-graph

generate-image2d:
	cargo run --bin watershed_image2d doc/imgs/lena_gray.png 500 public/generated/lena_watershed.png

generate-node-graph:
	cargo run --bin watershed_node_graph
	for f in *.dot; do \
	neato -Tsvg "$f" -o "${f%.dot}.svg"; \
	done; \
	rm *.dot; \
	mv *.svg public/generated