install:
	cargo build --release
	rm /usr/local/bin/depploy
	cp target/release/depploy /usr/local/bin/
remove:
	rm /usr/local/bin/depploy