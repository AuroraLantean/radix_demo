use scrypto::resource::ScryptoBucket;
use token_sale::token_sale_test::*;
//use dragon_coin::dragon_coin_test::*;
use scrypto_test::prelude::*;

#[test]
fn test() {
  // Setup the environment
  let mut ledger = LedgerSimulatorBuilder::new().build();

  // Create an account
  let (public_key, _private_key, account) = ledger.new_allocated_account();

  // Publish package
  let package_address = ledger.compile_and_publish(this_package!());

  // Test the `instantiate` function.
  let manifest = ManifestBuilder::new()
    .lock_fee_from_faucet()
    .call_function(
      package_address,
      "dragon_coin",
      "instantiate",
      manifest_args!(),
    )
    .build();
  let receipt = ledger.execute_manifest(
    manifest,
    vec![NonFungibleGlobalId::from_public_key(&public_key)],
  );
  println!("{:?}\n", receipt);
  let component = receipt.expect_commit(true).new_component_addresses()[0];

  // Test the `get_token` method.
  let manifest = ManifestBuilder::new()
    .lock_fee_from_faucet()
    .call_method(component, "get_token", manifest_args!())
    .call_method(
      account,
      "deposit_batch",
      manifest_args!(ManifestExpression::EntireWorktop),
    )
    .build();
  let receipt = ledger.execute_manifest(
    manifest,
    vec![NonFungibleGlobalId::from_public_key(&public_key)],
  );
  println!("{:?}\n", receipt);
  receipt.expect_commit_success();
}

#[test]
fn test_with_test_environment() -> Result<(), RuntimeError> {
  // Arrange
  let mut env = TestEnvironment::new();
  let package_address =
    PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

  let token_price = dec!(1);
  let mut token_sale = TokenSale::instantiate_token_sale(token_price, package_address, &mut env)?;

  // Act
  let amount = dec!(3);
  let bucket = token_sale.withdraw(amount, &mut env)?;

  // Assert
  let amount = bucket.amount();
  assert_eq!(amount, dec!("1"));

  Ok(())
}
