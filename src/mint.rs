#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod random;

use random::Random;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait Mint {
    #[storage_mapper("tokenIdentifier")]
    fn token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("nftPrice")]
    fn nft_price(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("nftsReceived")]
    fn nfts_received(&self) -> VecMapper<u64>;

    #[init]
    fn init(&self, initial_nft_identifier: TokenIdentifier, nft_price: BigUint) {
        self.token_identifier().set(&initial_nft_identifier);
        self.nft_price().set(&nft_price);
    }

    /// Add desired amount to the storage variable.
    #[payable("*")]
    #[endpoint(receiveNft)]
    fn call_receive_nft(
        &self,
        #[payment_token]  nft_collection_identifier: TokenIdentifier,
        #[payment_nonce]  nft_nonce: u64,
    )
      -> SCResult<()> {
        require!(nft_collection_identifier == self.token_identifier().get(), "You can't send nfts from that collection");
        require!(self.blockchain().get_owner_address() == self.blockchain().get_caller(), "You're not allowed to do that :)");

        self.nfts_received().push(&nft_nonce);
        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint(payForNft)]
    fn call_pay_for_nft(
        &self,
        #[payment_amount] egld_amount: BigUint,
    )
      -> SCResult<()> {
        require!(self.nfts_received().len() > 0 as usize, "No more nfts");
        require!(egld_amount == self.nft_price().get(), "Incorrect Price");

        //Cleaner to use next week
        //let mut randSource = RandomnessSource::<Self::Api>::new();
        //let randomIndex = randSource.next_usize_in_range(1 as usize, self.nfts_received().len() + (1 as usize));

        let seed = self.blockchain().get_block_random_seed_legacy();
        let mut rand_source = Random::new(*seed);
        let random_index = rand_source.next_usize_in_range(1 as usize, self.nfts_received().len() + 1);

        // Get the nft and remove it
        let nft_nonce = self.nfts_received().get(random_index);
        self.nfts_received().swap_remove(random_index);

        let caller = self.blockchain().get_caller();
        self.send().direct(
            &caller,
            &self.token_identifier().get(),
            nft_nonce,
            &BigUint::from(1 as u64),
            &[]
        );

        let owner = self.blockchain().get_owner_address();
        self.send().direct(
            &owner,
            &TokenIdentifier::egld(),
            0,
            &egld_amount,
            &[]
        );
        Ok(())
    }
}
