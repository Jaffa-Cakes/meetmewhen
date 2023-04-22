# To-Do

meetmewhen.com ?

- Enable http 2 on Actix AND Tonic
- Add sitemap generator
- Add PWA Support
- Add Meta Description Support
- Add title description support

Maybe add meta desc and title desc support by using the route enum and modifying the html?

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