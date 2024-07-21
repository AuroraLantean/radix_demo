# Radix Demo

Install Rust v1.79.0, then build:

```
rustup default 1.79.0
```

To update Radix CLI:
https://github.com/radixdlt/radixdlt-scrypto

```
rustup update stable
cargo install radix-clis
resim reset
rm Cargo.lock
cargo clean
scrypto build
```

When updating an existing project be sure to delete the existing cargo.lock file run `cargo clean` and `scrypto build` to recompile with all the latest updates.

https://github.com/radixdlt/radixdlt-scrypto

Format code: `cargo fmt`

Lint and auto fix: `cargo clippy --fix --allow-dirty`

1. Create a new account, and save the account component address

```
resim new-account
```

2. Publish the package, and save the package ID

```
resim publish .
```

3. Call the `instantiate` function to instantiate a component, and save the component address

```
resim call-function <PACKAGE_ADDRESS> DragonCoin instantiate
```

4. Call the `free_token` method of the component we just instantiated

```
resim call-method <COMPONENT_ADDRESS> free_token
```

5. Check out our balance

```
resim show <ACCOUNT_COMPONENT_ADDRESS>
```

or simply:

```
resim show
```

To show balance of default account.
