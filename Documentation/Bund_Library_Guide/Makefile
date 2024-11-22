SOURCES=$(wildcard *.typ)
NAME=BUND_standard_lisbrary_reference

all: $(NAME).pdf

rebuild:
	make clean
	make all

export:
	make rebuild
	mv $(NAME).pdf ../$(NAME)_`date +"%m%d%Y%H%M"`.pdf


$(NAME).pdf: $(SOURCES)  Makefile
	typst compile $(NAME).typ

clean:
	rm -f $(NAME)*.pdf
