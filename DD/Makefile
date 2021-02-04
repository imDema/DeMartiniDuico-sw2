default: generate
	latexmk -synctex=1 -interaction=nonstopmode -file-line-error -pdf -outdir=./ ./main

generate:
	python3 ./api/generate.py ./api/openapi.yaml > ./api/api.tex

clean:
	rm ./api/api.tex
	rm ./main.pdf
