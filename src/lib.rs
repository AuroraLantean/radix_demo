use scrypto::prelude::*;

#[blueprint]
mod hello {
  enable_method_auth! {
      methods {
          free_token => PUBLIC;
          burn => PUBLIC;
          make_new_token => restrict_to: [OWNER];
          withdraw => restrict_to: [OWNER];
      }
  }
  struct Hello {
    vault: Vault,
  }
  impl Hello {
    pub fn instantiate_hello() -> Global<Hello> {
      let admin_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None) // #2
        .divisibility(DIVISIBILITY_NONE)
        .metadata(metadata!(
            init {
                "name" => "Admin Badge", locked;
            }
        ))
        .mint_initial_supply(1)
        .into();

      // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
      let my_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(DIVISIBILITY_MAXIMUM)
        .metadata(metadata! {
            init {
                "name" => "HelloToken", locked;
                "symbol" => "HT", locked;
                "description" => "Hello Token is the best!", locked;
            }
        })
        .mint_roles(mint_roles!(
            minter => rule!(require(admin_badge));
            minter_updater => rule!(deny_all);
        ))
        .burn_roles(burn_roles!(
            burner => rule!(require(admin_badge));
            burner_updater => rule!(deny_all);
        ))
        .withdraw_roles(withdraw_roles!(
            withdrawer => rule!(require(admin_badge));
            withdrawer_updater => rule!(deny_all);
        ))
        .deposit_roles(deposit_roles!(
            depositor => rule!(require(admin_badge));
            depositor_updater => rule!(deny_all);
        ))
        .recall_roles(recall_roles!(
            recaller => rule!(require(admin_badge));
            recaller_updater => rule!(deny_all);
        ))
        .freeze_roles(freeze_roles!(
            freezer => rule!(require(admin_badge));
            freezer_updater => rule!(deny_all);
        ))
        .mint_initial_supply(1000)
        .into();
      //.create_with_no_initial_supply();
      /*.non_fungible_data_update_roles(non_fungible_data_update_roles!(
          non_fungible_data_updater => rule!(require(nft_data_updater_badge));
          non_fungible_data_updater_updater => rule!(deny_all);
      )) */
      // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
      Self {
        vault: Vault::with_bucket(my_bucket),
      }
      .instantiate()
      .prepare_to_globalize(OwnerRole::Updatable(rule!(require(admin_badge))))
      .enable_component_royalties(component_royalties!(
              init {
                  free_token => Xrd(dec!(1)), locked;
                  make_new_token => Xrd(dec!(1)), locked;
                  withdraw => Xrd(dec!(0)), locked;
                  burn => Xrd(dec!(0)), locked;
              }
      ))
      .roles(roles!(
              admin => rule!(require(admin_badge));
      ))
      .globalize()
    }

    pub fn burn(&self, bucket: Bucket) {
      assert!(bucket.resource_address() == self.vault.resource_address());
      bucket.burn();
    }

    pub fn make_new_token(
      &mut self,
      token_name: String,
      token_symbol: String,
      supply: Decimal,
    ) -> FungibleBucket {
      let tokens: FungibleBucket = ResourceBuilder::new_fungible(OwnerRole::None)
        .metadata(metadata! (
            init {
                "name" => token_name, locked;
                "symbol" => token_symbol, locked;
            }
        ))
        .mint_initial_supply(supply);

      return tokens;
    }
    pub fn withdraw(&mut self) -> Bucket {
      // This method can only be called if the caller presents an admin badge
      self.vault.take_all()
    }
    // This is a method, because it needs a reference to self.  Methods can only be called on components
    pub fn free_token(&mut self) -> Bucket {
      info!(
        "My balance is: {} HelloToken. Now giving away a token!",
        self.vault.amount()
      );
      // If the semi-colon is omitted on the last line, the last value seen is automatically returned
      // In this case, a bucket containing 1 HelloToken is returned
      self.vault.take(1)
    }
  }
}
