# wasm-minisign-verify

[![GitHub CI][github-actions-badge]][github-actions]
[![License][license-badge]][license]

This is the
[rust-minisign-verify](https://github.com/jedisct1/rust-minisign-verify)
library compiled to WebAssembly.


## Loading the Library

Note: A dependency graph that contains any WASM must all be imported
asynchronously.  This can be done using a bundler with an appropriate WASM
loader, or through
[dynamic imports](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#Dynamic_Imports).

### Bootstrapping JS

The simplest way is to use a bootstrapping js as the entry point to your entire application:

```js
// bootstrap.js
import('./index.js')
  .catch(e => console.error('Error importing `index.js`:', e));
```

```js
// index.js
import * as minisignVerify from '@threema/wasm-minisign-verify';
```

### Dynamic Import (Promise)

Alternatively, import the library asynchronously:

```js
import('@threema/wasm-minisign-verify')
    .then((minisignVerify) => {
        // Use the library
    });
```

If you're in an asynchronous context, you can also use the `await` keyword.

```js
const minisignVerify = await import('@threema/wasm-minisign-verify');
```


## Usage

### Initialization

In order to set up logging of log messages and panics:

```js
minisignVerify.setupLogging("info");
```

Valid log levels: `trace`, `debug`, `info`, `warn` or `error`.

### Signatures and Public Keys

Create a Minisign public key from a string, as in the `minisign.pub` file:

```js
const publicKey = minisignVerify.PublicKey.decode(
    "untrusted comment: minisign public key 60DF2F3B621B4533\n" +
    "RWQzRRtiOy/fYNCli5tW96CO6R+FnO92LceeIoWlCLj+BTVe+6q8T69M"
);
```

Create a Minisign signature from a string:

```js
const signature = minisignVerify.Signature.decode(
    "untrusted comment: signature from minisign secret key\n" +
    "RWQzRRtiOy/fYEU/vGHUEfBg+lSmrdpViX3l9fX1Ps6FMBrBcsMw9uxsLPFr9pAMdKy1NVEX3MsHsuCKlSVNYc4C5/pCnU/Kugk=\n" +
    "trusted comment: timestamp:1634045550	file:test.txt\n" +
    "zEHzYWS0L/lFlN3hfMdAJA0MsVfazBXbwSw9XihxQ0msFQPlC30F6Ajvxi67KEFNd1GUhdi3DcslssTW8MUECQ=="
);
```

### Signature Verification

To verify a signature, use the `verify` method on the `PublicKey`. If
verification fails, an exception will be thrown. Otherwise, the method will
return `true`.

```js
const signedData = new TextEncoder().encode("test\n");
try {
    publicKey.verify(signedData, signature);
    console.info("Signature verification succeeded");
} catch(e) {
    console.error(e);
}
```


## Dev Setup

    cargo install wasm-pack


## Building

    # Debug build
    wasm-pack build

    # Release build
    wasm-pack build --release -- --no-default-features


## Running the testproject

    # Setup npm
    cd www
    npm install

    # Run server
    npm run start


## Testing

    # Unit tests
    cargo test

    # NodeJS tests
    wasm-pack test --node


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.


<!-- Badges -->
[github-actions]: https://github.com/threema-ch/wasm-minisign-verify/actions?query=branch%3Amain
[github-actions-badge]: https://github.com/threema-ch/wasm-minisign-verify/workflows/CI/badge.svg
[license]: https://github.com/threema-ch/wasm-minisign-verify#license
[license-badge]: https://img.shields.io/badge/License-Apache%202.0%20%2f%20MIT-blue.svg
