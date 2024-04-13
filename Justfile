export coverageignore_file := ".coverageignore"
export default_rust_test_flags := "-- --nocapture --test-threads=1"
coverage:
    #!/usr/bin/env bash
    set -euxo pipefail
    cmd="cargo tarpaulin --engine=llvm --out Html"
    while IFS= read -r line || [ -n "$line" ];\
    do \
        cmd+=" --exclude-files \"$line\"";\
    done < $coverageignore_file; \
    cmd+=" ${default_rust_test_flags}"
    (set -x; eval "${cmd}");

test:
    cargo test ${default_rust_test_flags}


build-release:
    cargo build --release


run:
    cargo run

run-release:
    cargo run --release

run-local-detail:
    cargo run --features develop-detail


var-test:
    #!/usr/bin/env bash
    set -euxo pipefail
    var="hello"
    echo "$var"

build-docker:
    #!/usr/bin/env bash
    docker build --tag card_router --file Dockerfile --ssh default=$HOME/.ssh/id_ed25519 .

