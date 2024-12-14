Result i3 12100 @ win11dev@x86-64

```
tl;dr

Chrome V8       💪🏽
Chrome Wasm     🤮
FF SpiderMonkey 🤔
FF Wasm         💪🏽

Browsers native json > Serde
MsgPack > JSON

@msgpack/msgpack 🏎️
```

```
---Chrome 131---
JS JSON:            181 / 184
JS MsgPack:         193 / 187
Wasm/Rust JSON:    2083 / 895
Wasm/Rust MsgPack: 1789 / 264
Wasm/Rust MsgPack: 2268 / 251

---FF 133---
JS JSON:           196 / 184
JS MsgPack:        448 / 187
Wasm/Rust JSON:    272 / 242
Wasm/Rust MsgPack: 164 / 115
Wasm/Rust MsgPack: 147 / 100

---Win---
Rust JSON:    244 / 227
Rust MsgPack: 131 / 120
Rust MsgPack: 104 / 92

---wsl Ubuntu---
Rust JSON:    238 / 159
Rust MsgPack: 128 / 105
Rust MsgPack: 125 / 107
```

```
$wasm-pack build --release --target web // Already builded
docker-compose up
http://localhost:8080/index.html
Results in console

cd native
cargo build --release
./target/release/native.exe
```
