rm -rf dist\*
bun build-only 
cargo build --release --manifest-path=./backend/Cargo.toml
cp backend\target\release\backend.exe .\dist\backend.exe