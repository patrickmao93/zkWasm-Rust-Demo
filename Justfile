wasm := "pkg/mimc_bg.wasm"
args := "-k 22 --function zkmain --output ./output --wasm " + wasm
proof := "output/zkwasm.0.transcript.data"

cli := env_var_or_default('ZKWASM_CLI', 'delphinus-cli')

run:
  just build
  just prove

build:
  wasm-pack build --release
  wasm-opt -O3 pkg/mimc_bg.wasm -o mimc_bg.wasm --signext-lowering
  mv mimc_bg.wasm pkg/mimc_bg.wasm

setup:
  rm -rf output
  mkdir -p output
  {{cli}} {{args}} setup

prove:
  {{cli}} {{args}} single-prove

verify:
  {{cli}} {{args}} single-verify --proof {{proof}}

test:
  just build
  just setup
  just prove
  just verify
