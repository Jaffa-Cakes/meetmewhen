# meetmewhen

This is a basic Rust clone of when2meet.com.

# To-Do

There are various things I have yet to complete in this project, please look here to view everything that needs to be done: https://github.com/users/Jaffa-Cakes/projects/3

Various things are a little broken and not fixed as this is just a small proof of concept, you will likely have trouble running this application as it is not yet a fully fleshed out prototype.

## Useful Commands

If issues are occuring during build, run the following commands and then try again:
```powershell
cargo clean
rmdir ./csr/dist/ -Recurse -Force -Confirm:$false
```

### During Development

Running API:
```powershell
$env:PQ_LIB_DIR = "C:/Program Files/PostgreSQL/14/lib"
$env:DATABASE_URL = "postgres://admin:password@localhost:5432/tidsync"
cargo run --package api
```

Running CSR:
```powershell
cd csr
npm install
cd ..
tailwindcss -c ./csr/tailwind.config.js -o ./csr/tailwind.css -w
trunk serve ./csr/index.html -w ./
```

Running SSR:
```powershell
cargo clean
tailwindcss -c ./csr/tailwind.config.js -o ./csr/tailwind.css
trunk build ./csr/index.html
cargo run --package ssr
```

### During Production

Running API:
```powershell
$env:PQ_LIB_DIR = "C:/Program Files/PostgreSQL/14/lib"
$env:DATABASE_URL = "postgres://admin:password@localhost:5432/tidsync"
cargo run --package api --release
```

Running CSR:
```powershell
tailwindcss -c ./csr/tailwind.config.js -o ./csr/tailwind.css -w --minify
trunk serve ./csr/index.html -w ./ --release
```

Running SSR:
```powershell
cargo clean
tailwindcss -c ./csr/tailwind.config.js -o ./csr/tailwind.css --minify
trunk build ./csr/index.html --release
cargo run --package ssr --release
```
