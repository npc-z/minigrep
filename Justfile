# run with query. Usage: just run name
run query:
  cargo run -- {{query}} src/poem.txt

# build release
build:
  cargo build --release
