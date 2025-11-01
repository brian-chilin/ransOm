## ransOm

#### dev
- `wasm-pack build --target web`
- the devcontainer already has nginx serving `/workspace` via symlink so `index.html` in this project (and the other files it refers to) are served - nginx will update allowing live development as long as you hard refresh

#### todo
- config dockerfile and www files as to prevent also serving ./ ie ./reame.md, ./Cargo.toml, ./src/lib.rs, ./.devcontainer/dockerfile, ./dockerfile etc...
