#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq)]
pub struct LockedLpAttributes {
    pub attr1: u64,
    pub attr2: usize,
    pub attr3: u64,
}

#[multiversx_sc::contract]
pub trait NeverseaContract:
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
        let payment_amount = self.call_value().egld_value().clone_value();
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
    }

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
}
