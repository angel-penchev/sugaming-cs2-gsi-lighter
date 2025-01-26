all: build kill deploy run

build:
	cargo build -Zbuild-std --release --target aarch64-unknown-linux-gnu

kill:
	ssh tsb@192.168.4.224 "killall sugaming-cs2-gsi-lighter || true"

deploy:
	scp target/aarch64-unknown-linux-gnu/release/sugaming-cs2-gsi-lighter tsb@192.168.4.224:~

run:
	ssh tsb@192.168.4.224 "cd /home/tsb && ./sugaming-cs2-gsi-lighter"

clean:
	cargo clean

