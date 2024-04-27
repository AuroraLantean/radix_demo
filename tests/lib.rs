use dragon_coin::test_bindings::*;
use radix_engine_interface::prelude::*;
use scrypto::this_package;
use scrypto_test::prelude::*;

use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn test() {
  // Setup the environment
  let mut test_runner = TestRunnerBuilder::new().build();

  // Create an account
  let (public_key, _private_key, account) = test_runner.new_allocated_account();

  // Publish package
  let package_address = test_runner.compile_and_publish(this_package!());

  // Test the `instantiate` function.
  let manifest = ManifestBuilder::new()
    .call_function(
      package_address,
      "dragog_coin",
      "instantiate",
      manifest_args!(),
    )
    .build();
  let receipt = test_runner.execute_manifest_ignoring_fee(
    manifest,
    vec![NonFungibleGlobalId::from_public_key(&public_key)],
  );
  println!("{:?}\n", receipt);
  let component = receipt.expect_commit(true).new_component_addresses()[0];

  // Test the `free_token` method.
  let manifest = ManifestBuilder::new()
    .call_method(component, "free_token", manifest_args!())
    .call_method(
      account,
      "deposit_batch",
      manifest_args!(ManifestExpression::EntireWorktop),
    )
    .build();
  let receipt = test_runner.execute_manifest_ignoring_fee(
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
  let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

  let mut dragog_coin = DragonCoin::instantiate(package_address, &mut env)?;

  // Act
  let amount = dec!(3);
  let bucket = dragog_coin.get_token(amount, &mut env)?;

  // Assert
  let amount = bucket.amount(&mut env)?;
  assert_eq!(amount, dec!("1"));

  Ok(())
}
