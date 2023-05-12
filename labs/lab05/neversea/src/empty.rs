#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq)]
pub struct LockedLpAttributes {
    pub attr1: u64,
    pub attr2: usize,
    pub attr3: u64,
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait NeverseaContract:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self, initial_registration_fee: BigUint, initial_registration_fee_vip: BigUint) {
        self.registration_fee().set(initial_registration_fee);
        self.registration_fee_vip()
            .set(initial_registration_fee_vip);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    #[payable("EGLD")]
    fn register(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount_ref = self.call_value().egld_value();
        let payment_amount = payment_amount_ref.clone_value();
        let registration_fee_vip = self.registration_fee_vip().get();
        let registration_fee = self.registration_fee().get();

        require!(
            payment_amount.eq(&registration_fee_vip) || payment_amount.eq(&registration_fee),
            "Registration fee is incorrect; please check and try again"
        );
        if payment_amount.eq(&registration_fee) {
            self.participants().insert(caller);
            let owner = self.blockchain().get_owner_address();
            self.send().direct_egld(&owner, &payment_amount);
        } else if payment_amount.eq(&registration_fee_vip) {
            self.vip_participants().insert(caller);
        }
    }

    /// Add desired amount to the storage variable.
    #[only_owner]
    #[endpoint]
    fn update_registration_fees(
        &self,
        new_registration_fee: BigUint,
        new_registration_fee_vip: BigUint,
    ) {
        self.registration_fee().set(new_registration_fee.clone());
        self.registration_fee_vip()
            .set(new_registration_fee_vip.clone());

        self.registration_fee_event(new_registration_fee, new_registration_fee_vip)
    }

    #[only_owner]
    #[endpoint]
    fn withdraw_funds(&self, withdraw_amount: BigUint) {
        let owner = self.blockchain().get_owner_address();

        self.send().direct_egld(&owner, &withdraw_amount);
    }

    #[only_owner]
    #[endpoint]
    #[payable("EGLD")]
    fn issue_nft(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let payment_amount = self.call_value().egld_value();
        self.participant_token_name()
            .set(token_display_name.clone());
        self.participant_token().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount.clone_value(),
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    #[endpoint]
    #[payable("*")]
    fn mint_nft(&self) {
        let _caller = self.blockchain().get_caller();
        let _current_block = self.blockchain().get_block_epoch();

        let staked_lp_payment = self.mint_lp_token_direct(
            &self.participant_token().get_token_id(),
            10usize,
            &BigUint::from(1u32),
            self.blockchain().get_block_epoch(),
            10u64,
        );
        let attr:LockedLpAttributes = self.participant_token().get_token_attributes(staked_lp_payment.token_nonce);
        self.debug_minted_event(attr.attr1, attr.attr2, attr.attr3);


        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &staked_lp_payment.token_identifier,
            staked_lp_payment.token_nonce,
            &staked_lp_payment.amount,
        );
        self.debug_minted_event(attr.attr1, attr.attr2, attr.attr3);
    }

    fn mint_lp_token_direct(
        &self,
        token_id: &TokenIdentifier,
        token_type_id: usize,
        amount_to_mint: &BigUint,
        attr1: u64,
        attr3: u64,
    ) -> EsdtTokenPayment {
        let lp_token_attributes = LockedLpAttributes {
            attr1,
            attr2: token_type_id,
            attr3,
        };
        let token_display_name = self.participant_token_name().get();
        let minted_nonce = self.send().esdt_nft_create_compact_named(
            token_id,
            amount_to_mint,
            &token_display_name,
            &lp_token_attributes,
        );
        // self.increase_aggregated_exotic_lps_amount(&token_type_id, amount_to_mint);
        EsdtTokenPayment::new(token_id.clone(), minted_nonce, amount_to_mint.clone())
    }

    #[event("registration_fee_event")]
    fn block_epoch_event(&self, #[indexed] block_epoch: u64);

    #[event("debug_minted_event")]
    fn debug_minted_event(
        &self,
        #[indexed] stake_epoch: u64,
        #[indexed] locked_lp_type_id: usize,
        #[indexed] unbonding_start_epoch: u64,
    );

    #[event("registration_fee_event")]
    fn registration_fee_event(
        &self,
        #[indexed] registration_fee: BigUint,
        #[indexed] registration_fee_vip: BigUint,
    );

    #[view(getParticipants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getVipParticipants)]
    #[storage_mapper("vip_participants")]
    fn vip_participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getRegistrationFee)]
    #[storage_mapper("registration_fee")]
    fn registration_fee(&self) -> SingleValueMapper<BigUint>;

    #[view(getRegistrationFeeVip)]
    #[storage_mapper("registration_fee_vip")]
    fn registration_fee_vip(&self) -> SingleValueMapper<BigUint>;

    #[view(participantToken)]
    #[storage_mapper("participantToken")]
    fn participant_token(&self) -> NonFungibleTokenMapper;

    #[view(participantTokenName)]
    #[storage_mapper("participantTokenName")]
    fn participant_token_name(&self) -> SingleValueMapper<ManagedBuffer>;
}
