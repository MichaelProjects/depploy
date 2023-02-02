install:
	cargo build --release
	#rm /usr/local/bin/depploy
	cp target/release/depploy /usr/local/bin/depploy
remove:
	rm /usr/local/bin/depploy
