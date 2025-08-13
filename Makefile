build-broken:
	docker build -f broken.Dockerfile -t rust-https-check:broken .

run-broken:
	docker run --rm rust-https-check:broken https://example.com

build:
	docker build -t rust-https-check:stable .

run:
	docker run --rm rust-https-check:stable https://example.com