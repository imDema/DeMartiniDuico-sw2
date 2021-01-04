all: latex

latex: convert
	pandoc api.md -f markdown -t latex -o api.tex

convert: markdown
	python3 convert.py

markdown:
	mkdir -p gen
	openapi-generator-cli generate -i openapi.yaml -o gen -g markdown

clean:
	rm ./gen/ -r
	rm ./api.md
	rm ./api.tex