# Every check CI runs, as one local command: `make check`.
# Each step is also its own target, and CI (.github/workflows/ci.yml) calls these same targets.
# Needs Rust (rustup), Lean via elan (lean/lean-toolchain pins the version) and cargo-deny.

export PATH := $(HOME)/.elan/bin:$(PATH)
CARGO := cargo
VECTORS := json.json requests.json decode_reject.json

.PHONY: check fmt clippy test proofs vectors deny

check: fmt clippy test proofs vectors deny
	@echo "make check: every check passed"

fmt:
	$(CARGO) fmt --all --check

# Default features are what ships; all features add the test approvers and test registration.
clippy:
	$(CARGO) clippy --workspace --all-targets --locked -- -D warnings
	$(CARGO) clippy --workspace --all-targets --all-features --locked -- -D warnings

# Build every binary first: some tests start bogo-relay from the target directory.
test:
	$(CARGO) build --workspace --all-targets --all-features --locked
	$(CARGO) test --workspace --all-features --locked

# The Lean core and its proofs. `#guard_msgs in #print axioms` (BogoProofs/Axioms.lean)
# fails the build if any theorem's axioms change.
proofs:
	cd lean && lake build

# The checked-in vectors must be exactly what the Lean encoder writes today.
vectors: proofs
	@out="$$(mktemp -d)" && \
	(cd lean && lake exe BogoVectors "$$out" > /dev/null) && \
	for f in $(VECTORS); do diff -u "vectors/$$f" "$$out/$$f" || exit 1; done && \
	rm -rf "$$out" && echo "vectors match the Lean encoder"

# Known advisories, licences and sources (deny.toml).
deny:
	$(CARGO) deny --locked check
