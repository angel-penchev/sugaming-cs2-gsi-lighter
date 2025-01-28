all: build kill deploy run

build:
	cargo build -Zbuild-std --release --target aarch64-unknown-linux-gnu

kill:
	ssh ${RPI_USERNAME}@${RPI_IP_ADDRESS} "killall sugaming-cs2-gsi-lighter || true"

deploy:
	scp target/aarch64-unknown-linux-gnu/release/sugaming-cs2-gsi-lighter ${RPI_USERNAME}@${RPI_IP_ADDRESS}:~

run:
	ssh ${RPI_USERNAME}@${RPI_IP_ADDRESS} "cd ~ && ./sugaming-cs2-gsi-lighter"

clean:
	cargo clean

