install-system-dependencies:  
     brew install ktlint

download-models:
     wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx     
     wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json

generate-kotlin-bindings-debug:
     cargo run --features=uniffi/cli --bin uniffi-bindgen generate --library target/debug/libaudify_rs.dylib --language kotlin --out-dir out


install-android-target:
     rustup target add \
     aarch64-linux-android \
     armv7-linux-androideabi \
     x86_64-linux-android \
     i686-linux-android