#!/bin/bash

DOCUMENT_NAME=Data_Lexicon
VERSION=$(date +"%Y-%m-%d_%H-%M-%S")

pdflatex -interaction=nonstopmode $DOCUMENT_NAME.tex

#mv $DOCUMENT_NAME.pdf "$DOCUMENT_NAME""_v""$VERSION"".pdf"

rm -f $DOCUMENT_NAME.aux $DOCUMENT_NAME.log

echo "PDF version $VERSION built successfully."
