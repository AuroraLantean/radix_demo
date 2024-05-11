use scrypto::prelude::*;

#[blueprint]
mod dragon_coin {
  enable_method_auth! {
      methods {
          get_token => PUBLIC;
          withdraw => restrict_to: [OWNER];
          withdraw_all => restrict_to: [OWNER];
          burn => PUBLIC;
          make_new_token => restrict_to: [OWNER];
      }
  }
  struct DragonCoin {
    vault: Vault,
  }
  impl DragonCoin {
    pub fn instantiate() -> Global<DragonCoin> {
      let admin_badge = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(DIVISIBILITY_NONE)
        .metadata(metadata!(
            init {
                "name" => "Admin Badge", locked;
            }
        ))
        .mint_initial_supply(1);

      let my_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(DIVISIBILITY_MAXIMUM)
        .metadata(metadata! {
            init {
                "name" => "DragonCoin", locked;
                "symbol" => "DRGC", locked;
                "description" => "DragonCoin has magical powers", locked;
            }
        })
        .mint_roles(mint_roles!(
            minter => rule!(require(admin_badge.resource_address()));
            minter_updater => rule!(deny_all);
        ))
        .burn_roles(burn_roles!(
            burner => rule!(require(admin_badge.resource_address()));
            burner_updater => rule!(deny_all);
        ))
        .withdraw_roles(withdraw_roles!(
            withdrawer => rule!(require(admin_badge.resource_address()));
            withdrawer_updater => rule!(deny_all);
        ))
        .deposit_roles(deposit_roles!(
            depositor => rule!(require(admin_badge.resource_address()));
            depositor_updater => rule!(deny_all);
        ))
        .recall_roles(recall_roles!(
            recaller => rule!(require(admin_badge.resource_address()));
            recaller_updater => rule!(deny_all);
        ))
        .freeze_roles(freeze_roles!(
            freezer => rule!(require(admin_badge.resource_address()));
            freezer_updater => rule!(deny_all);
        ))
        .mint_initial_supply(1000)
        .into();
      //.create_with_no_initial_supply();
      /*.non_fungible_data_update_roles(non_fungible_data_update_roles!(
          non_fungible_data_updater => rule!(require(nft_data_updater_badge));
          non_fungible_data_updater_updater => rule!(deny_all);
      )) */
      // Instantiate component, populating its vault with our supply of 1000 tokens
      Self {
        vault: Vault::with_bucket(my_bucket),
      }
      .instantiate()
      .prepare_to_globalize(OwnerRole::Updatable(rule!(require(
        admin_badge.resource_address()
      ))))
      /*.enable_component_royalties(component_royalties!(
      init {
          withdraw => Xrd(dec!(1)), locked;
          withdraw_all => Xrd(dec!(0)), locked;
          burn => Xrd(dec!(0)), locked;
          make_new_token => Xrd(dec!(1)), locked;
        }))*/
      .globalize()
      // .roles(roles!(admin => rule!(require(admin_badge.resource_address()));
      // ))
    }
    //a method refers to itself
    pub fn get_token(&mut self, amount: Decimal) -> Bucket {
      info!("Vault balance: {}... now minting", self.vault.amount());
      assert!(self.vault.amount() >= amount);
      self.vault.take(amount)
    }
    pub fn withdraw(&mut self, amount: Decimal) -> Bucket {
      info!("original vault balance: {}...", self.vault.amount());
      assert!(self.vault.amount() >= amount);
      self.vault.take(amount)
    }
    pub fn withdraw_all(&mut self) -> Bucket {
      self.vault.take_all()
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

      tokens
    }
  }
}
