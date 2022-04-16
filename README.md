# Sample Yew etch Application

### Local setup

Install Rust 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install trunk
```

Build bundle
```
cargo install wasm-pack
wasm-pack build --target web --out-name wasm --out-dir ./static
```

In static directory create an `index.html` file with an entrypoint for the bundle
```
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew Sample App</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
```

Run using a webserver
```
cargo install miniserve
miniserve ./static --index index.html
```

