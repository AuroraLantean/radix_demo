# Radix Demo

1. Create a new account, and save the account component address

```
resim new-account
```

2. Publish the package, and save the package ID

```
resim publish .
```

3. Call the `instantiate_hello` function to instantiate a component, and save the component address

```
resim call-function <PACKAGE_ADDRESS> Hello instantiate_hello
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
