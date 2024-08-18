use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
struct ClaimData {
  #[mutable]
  claimed: bool,
}
#[blueprint]
mod token_sale {
  const INIT_BADGE: ResourceManager =
    resource_manager!("resource_sim1n27mf55xqp3hadmwlt9tsyf6frsl4hw4ufhadtq5u5arhgy5vjftyh");
  enable_function_auth! {
      instantiate_token_sale => rule!(require(INIT_BADGE.address()));
  }
  //TODO: RadixDLT Academy course: 4.7 Application-level Authentication
  //define custom roles and method visibilities
  enable_method_auth! {
    roles {
      admin => updatable_by: [super_admin, OWNER];
      super_admin => updatable_by: [OWNER];
  },
    methods {
      get_price => PUBLIC;
      claim_tokens => PUBLIC;
      mint_claim_badge => PUBLIC;
      send_airdrop => PUBLIC;
      buy_token => PUBLIC;
          burn => PUBLIC;
          update_role => PUBLIC;
          claim_royalty => PUBLIC;//but certain role requirement will still be enforced
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
    minter_badge_vault: FungibleVault,
    claim_badge_manager: ResourceManager,
    airdrop_badge_vault: FungibleVault,
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

      let super_admin_badge = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(DIVISIBILITY_NONE)
        .metadata(metadata!(
            init {
                "name" => "Super Admin Badge", locked;
            }
        ))
        .mint_initial_supply(1);

      let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None)
        .divisibility(DIVISIBILITY_NONE)
        .metadata(metadata!(
            init {
                "name" => "Owner Badge", locked;
            }
        ))
        .mint_initial_supply(1);

      let minter_badge = ResourceBuilder::new_fungible(OwnerRole::None).mint_initial_supply(1);
      let claim_badge = ResourceBuilder::new_ruid_non_fungible::<ClaimData>(OwnerRole::None)
        .mint_roles(mint_roles!(
            minter => rule!(require(minter_badge.resource_address()));
            minter_updater => rule!(deny_all);
        ))
        .non_fungible_data_update_roles(non_fungible_data_update_roles!(
            non_fungible_data_updater => rule!(require(minter_badge.resource_address()));
            non_fungible_data_updater_updater => rule!(deny_all);
        ))
        .create_with_no_initial_supply();

      let airdrop_badge = ResourceBuilder::new_fungible(OwnerRole::None).mint_initial_supply(1);

      let abadge_addr = admin_badge.resource_address();

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
            minter => rule!(require(abadge_addr));
            minter_updater => rule!(deny_all);
        ))
        .burn_roles(burn_roles!(
            burner => rule!(require(abadge_addr));
            burner_updater => rule!(deny_all);
        ))
        .withdraw_roles(withdraw_roles!(
            withdrawer => rule!(require(abadge_addr));
            withdrawer_updater => rule!(deny_all);
        ))
        .deposit_roles(deposit_roles!(
            depositor => rule!(require(abadge_addr));
            depositor_updater => rule!(deny_all);
        ))
        .recall_roles(recall_roles!(
            recaller => rule!(require(abadge_addr));
            recaller_updater => rule!(deny_all);
        ))
        .freeze_roles(freeze_roles!(
            freezer => rule!(require(abadge_addr));
            freezer_updater => rule!(deny_all);
        ))
        .mint_initial_supply(1000);
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
        minter_badge_vault: FungibleVault::with_bucket(minter_badge),
        claim_badge_manager: claim_badge,
        airdrop_badge_vault: FungibleVault::with_bucket(airdrop_badge),
      }
      .instantiate()
      .prepare_to_globalize(
        OwnerRole::Fixed(rule!(require(owner_badge.resource_address()))), //OwnerRole::Updatable(rule!(require(owner_badge_addr)))
      )
      .roles(roles!(
          super_admin => rule!(require(super_admin_badge.resource_address()));
          admin => rule!(require(admin_badge.resource_address()));
      ))
      /*.roles(roles!(
      super_admin => OWNER;
      admin => OWNER;))  */
      .metadata(metadata!(
          roles {
              metadata_setter => rule!(require(admin_badge.resource_address()));
              metadata_setter_updater => OWNER;
              metadata_locker => rule!(require(super_admin_badge.resource_address()));
              metadata_locker_updater => OWNER;
          },
          init {
              "name" => "Token Sale Component", locked;
              "description" => "A component that sells useful tokens", locked;
          }
      ))
      .enable_component_royalties(component_royalties!(
          roles {
              royalty_setter => rule!(require(admin_badge.resource_address()));
              royalty_setter_updater => OWNER;
              royalty_locker => rule!(require(super_admin_badge.resource_address()));
              royalty_locker_updater => OWNER;
              royalty_claimer => OWNER;
              royalty_claimer_updater => OWNER;
          },
          init {
              get_price => Usd(1.into()), updatable;
              buy_token => Xrd(1.into()), updatable;
              claim_tokens => Free, locked;
              mint_claim_badge => Free, locked;
              send_airdrop => Free, locked;
              claim_royalty => Free, locked;
              withdraw => Free, locked;
              withdraw_all => Free, locked;
              withdraw_xrd => Free, locked;
              burn => Free, locked;
              make_new_token => Free, locked;
              update_role => Free, locked;
          }
      ))
      .globalize()
    }

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

    #[allow(clippy::let_and_return)]
    pub fn mint_claim_badge(&mut self) -> NonFungibleBucket {
      let claim_badge = self
        .minter_badge_vault
        .authorize_with_amount(1, || {
          self
            .claim_badge_manager
            .mint_ruid_non_fungible(ClaimData { claimed: false })
        })
        .as_non_fungible();
      claim_badge
    }

    pub fn claim_tokens(&mut self, claim_badge_proof: NonFungibleProof) -> FungibleBucket {
      let checked_proof = claim_badge_proof
        .check_with_message(self.claim_badge_manager.address(), "Incorrect proof!");
      //.check().expect("Incorrect proof!");
      let item = checked_proof.non_fungible::<ClaimData>();
      //let nft_data = item.data();

      // Asserting that the claimed field does not have a value of true.//assert_ne
      assert!(
        !item.data().claimed,
        "{} {}",
        true,
        "You have already claimed your tokens"
      );
      //update item info like mappings in Solidity
      self.minter_badge_vault.authorize_with_amount(1, || {
        self
          .claim_badge_manager
          .update_non_fungible_data(item.local_id(), "claimed", true)
      });
      self.new_token_vault.take(10)
    }

    //Send token; Components Generating Proofs: components that wish to regularly send tokens to an account will require the component to become an “authorized depositor”,
    pub fn send_airdrop(&mut self, mut account: Global<Account>) {
      self.airdrop_badge_vault.authorize_with_amount(dec!(1), || {
        let bucket_of_tokens = self.new_token_vault.take(1);

        account.try_deposit_or_abort(
          bucket_of_tokens.into(),
          Some(ResourceOrNonFungible::Resource(
            self.airdrop_badge_vault.resource_address(),
          )),
        );
      });
    }

    pub fn get_price(&mut self) -> Decimal {
      info!("get_price");
      self.token_price
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

    pub fn update_role(&mut self, role_name: String, new_badge: ResourceAddress) {
      let access_rule = rule!(require(new_badge));

      Runtime::global_component().set_role(&role_name, access_rule);
    }
    pub fn claim_royalty(&mut self) -> Bucket {
      let royalty_bucket: Bucket = Runtime::global_component().claim_component_royalties();
      royalty_bucket
    }
  }
}
