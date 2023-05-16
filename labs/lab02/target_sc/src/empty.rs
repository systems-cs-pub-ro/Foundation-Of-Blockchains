#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait TargetScContract {
    #[init]
    fn init(&self) {
        self.first_value().get();
        self.second_value().get();

        #[storage_mapper("duplicateValue")]
        fn first_value(&self) -> SingleValueMapper<BigUint>;

        #[storage_mapper("duplicateValue")]
        fn second_value(&self) -> SingleValueMapper<BigUint>;
    }
}
