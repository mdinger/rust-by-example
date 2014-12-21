GITBOOK = node_modules/.bin/gitbook
RUSTC = rustc

.PHONY: all book clean test serve

serve: node_modules/gitbook
	$(GITBOOK) serve .

node_modules/gitbook:
	npm install gitbook
