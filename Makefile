run:
	cargo watch -i "*.csv" -i "*.zip" -i "*.xlsx" -x "run"

release:
	cargo run -r

docker:
	docker compose build
	docker compose down
	docker compose up

format:
	cargo fmt
	cargo clippy

commit: format
	git add .
	git commit
