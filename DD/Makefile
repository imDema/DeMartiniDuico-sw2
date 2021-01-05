-include

default:
	python3 ./api/generate.py ./api/openapi.yaml > ./api/api.tex
	latexmk -synctex=1 -interaction=nonstopmode -file-line-error -pdf -outdir=./ ./main

clean:
	rm ./api/api.tex
	rm ./main.pdf