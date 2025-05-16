download-models:
     wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx     
     wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json

generate-kotlin-bindings-debug:
  cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library target/debug/libaudify_rs.so --language kotlin --out-dir out
