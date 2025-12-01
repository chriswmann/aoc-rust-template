# Run a specific day
run-day day:
  cargo r --bin day{{day}}


run-tests:
  cargo t --workspace

add-day day:
  mkdir -vp cached_data
  cargo new day{{day}}
  echo "santas_little_helpers = { workspace = true }" >> day{{day}}/Cargo.toml
  cp main_template.rs day{{day}}/src/main.rs
  sed -i 's/members = \[/members = ["day{{day}}", /' Cargo.toml

