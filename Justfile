run:
    @ cargo run --quiet --release -- --succeed

run-icons:
    @ cargo run --quiet --release -- --status-style icons

run-info:
    @ cargo run --quiet --release -- --status-style icons --header --footer --show-description

run-raw:
    @ cargo run --quiet --release -- --no-status
