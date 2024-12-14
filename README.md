Result i3 12100 @ win11@x86-64

```
tk;dr

Chrome V8💪🏽
Chrome Wasm🤮
FF SpiderMonkey🤮
FF Wasm💪🏽

Browsers native json > Serde
MsgPack > JSON

@msgpack/msgpack🏎️
```

(first run/second run)

---Chrome 131---
JS JSON: 181/184<br>
JS MsgPack: 193/187<br>
Wasm/Rust JSON: 2083/895<br>
Wasm/Rust MsgPack: 1789/264<br>
Wasm/Rust MsgPack: 2268/251<br>

---FF 133---
JS JSON: 196/184<br>
JS MsgPack: 448/187<br>
Wasm/Rust JSON: 272/242<br>
Wasm/Rust MsgPack: 164/115<br>
Wasm/Rust MsgPack: 147/100<br>

---Win---
Rust JSON: 244/227<br>
Rust MsgPack: 131/120<br>
Rust MsgPack: 104/92<br>

---wsl Ubuntu---
Rust JSON: 238/159<br>
Rust MsgPack: 128/105<br>
Rust MsgPack: 125/107<br>


```
$wasm-pack build --release --target web // Already builded
docker-compose up
http://localhost:8080/index.html
Results in console

cd native
cargo build --release
./target/release/native.exe
```
