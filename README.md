# Radix Demo
Install Rust v1.77.2, then build: 
```
rustup default 1.77.2
cargo build
```

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
