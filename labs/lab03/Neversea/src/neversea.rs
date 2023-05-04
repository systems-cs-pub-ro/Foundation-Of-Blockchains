#![no_std]

use core::ops::Deref;

use multiversx_sc::codec::Empty;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Neversea {
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
        let payment_amount = payment_amount_ref.deref();
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
        self.registration_fee()
            .set(BigUint::from(new_registration_fee.clone()));
        self.registration_fee_vip()
            .set(BigUint::from(new_registration_fee_vip.clone()));

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
        self.participant_token().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount.deref().clone(),
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    #[only_owner]
    #[endpoint]
    #[payable("EGLD")]
    fn mint_nft(&self) {
        self.participant_token()
            .nft_create(BigUint::from(1u32), &Empty);
    }

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
}
