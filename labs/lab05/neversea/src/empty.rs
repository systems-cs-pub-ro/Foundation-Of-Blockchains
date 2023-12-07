#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait NeverseaContract:
{
    #[init]
    fn init(&self, initial_registration_fee: BigUint) {
        self.registration_fee().set(initial_registration_fee);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    #[payable("EGLD")]
    fn register(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value().clone_value();
        let registration_fee = self.registration_fee().get();

        require!(
            payment_amount.eq(&registration_fee),
            "Registration fee is incorrect; please check and try again"
        );
        if payment_amount.eq(&registration_fee) {
            self.participants().insert(caller);
            let owner = self.blockchain().get_owner_address();
            self.send().direct_egld(&owner, &payment_amount);
        }
    }

    /// Add desired amount to the storage variable.
    #[only_owner]
    #[endpoint]
    fn update_registration_fees(
        &self,
        new_registration_fee: BigUint,
    ) {
        self.registration_fee().set(new_registration_fee.clone());
    }

    #[view(getParticipants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getRegistrationFee)]
    #[storage_mapper("registration_fee")]
    fn registration_fee(&self) -> SingleValueMapper<BigUint>;

}
