use scrypto::prelude::*;
#[blueprint]
mod token_sale {
  enable_method_auth! {
    roles {
      admin => updatable_by: [super_admin, OWNER];
      super_admin => updatable_by: [OWNER];
  },//TODO: continue here at the Scrypto101 course
    methods {
          buy_token => PUBLIC;
          burn => PUBLIC;
          withdraw => restrict_to: [OWNER];
          withdraw_xrd => restrict_to: [OWNER];
          withdraw_all => restrict_to: [OWNER];
          make_new_token => restrict_to: [OWNER];
      }//SELF - Denotes the component itself
  }
  struct TokenSale {
    new_token_vault: FungibleVault,
    xrd_vault: FungibleVault,
    token_price: Decimal,
  }
  impl TokenSale {
    pub fn instantiate_token_sale(token_price: Decimal) -> Global<TokenSale> {
      let admin_badge = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(DIVISIBILITY_NONE)
        .metadata(metadata!(
            init {
                "name" => "Admin Badge", locked;
            }
        ))
        .mint_initial_supply(1);

      let my_bucket = ResourceBuilder::new_fungible(OwnerRole::None)
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
        new_token_vault: FungibleVault::with_bucket(my_bucket),
        xrd_vault: FungibleVault::new(XRD),
        token_price,
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
    pub fn buy_token(
      &mut self,
      amount: Decimal,
      mut payment: FungibleBucket,
    ) -> (FungibleBucket, FungibleBucket) {
      // take our price in XRD out of the payment if the caller has sent too few, or sent something other than XRD, they'll get a runtime error
      let xrd_payment_amount = self.token_price.checked_mul(amount).unwrap();
      let xrd_payment = payment.take(xrd_payment_amount);
      self.xrd_vault.put(xrd_payment);

      // return a tuple containing a new token, plus whatever change is left on the input payment (if any) if we're out of new tokens to give, we'll see a runtime error when we try to grab one
      (self.new_token_vault.take(amount), payment)
    }
    pub fn withdraw(&mut self, amount: Decimal) -> FungibleBucket {
      info!("new_token_vault balance: {}", self.new_token_vault.amount());
      assert!(self.new_token_vault.amount() >= amount);
      self.new_token_vault.take(amount)
    }
    pub fn withdraw_all(&mut self) -> FungibleBucket {
      self.new_token_vault.take_all()
    }
    pub fn burn(&self, bucket: Bucket) {
      assert!(bucket.resource_address() == self.new_token_vault.resource_address());
      bucket.burn();
    }
    pub fn withdraw_xrd(&mut self, amount: Decimal) -> FungibleBucket {
      assert!(self.xrd_vault.amount() >= amount);
      self.xrd_vault.take(amount)
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
