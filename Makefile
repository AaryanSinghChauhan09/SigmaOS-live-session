.PHONY: build test install clean
build:
	python3 -m compileall -q src
test:
	python3 -m unittest discover -s tests -v
install:
	install -Dm755 bin/sigma-live-session $(DESTDIR)/usr/bin/sigma-live-session
clean:
	rm -rf src/__pycache__ tests/__pycache__
