.PHONY: stark101
stark101: 
		@cargo run --release --bin stark101

.PHONY: babysnark
babysnark: 
		@cargo test -p babysnark

.PHONY: rsa
rsa: 
		@cargo test -p rsa

.PHONY: shamir
shamir: 
		@cargo test -p shamir-secret-share

.PHONY: vault-of-loki
vault-of-loki: 
		@cargo run --release --bin vault-of-loki
