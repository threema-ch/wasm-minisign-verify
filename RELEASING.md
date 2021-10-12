# Releasing

Set variables:

    export VERSION=X.Y.Z
    export GPG_KEY=E7ADD9914E260E8B35DFB50665FDE935573ACDA6

Update version numbers:

    vim -p Cargo.toml
    cargo update -p wasm-minisign-verify

Update changelog:

    vim CHANGELOG.md

Commit & tag:

    git commit -S${GPG_KEY} -m "Release v${VERSION}"
    git tag -s -u ${GPG_KEY} v${VERSION} -m "Version ${VERSION}"

Publish:

    rm -rf pkg && wasm-pack build --scope threema --release -t nodejs
    # Now: Make sure that `pkg/package.json` includes `wasm_minisign_verify_bg.js`...
    # See https://github.com/rustwasm/wasm-pack/issues/990
    cd pkg && npm publish --access=public && cd ..
    git push && git push --tags
