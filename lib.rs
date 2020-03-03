#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod open_auction {
    use ink_core::storage;

    #[ink(event)]
    struct Bid {
        #[ink(topic)]
        bidder: Option<AccountId>,
        #[ink(topic)]
        bid: Balance,
    }

    #[ink(storage)]
    struct OpenAuction {
        // Auction params
        // Beneficiary receives money from the highest bidder
        beneficiary: storage::Value<AccountId>,
        auction_start: storage::Value<Timestamp>,
        auction_end: storage::Value<Timestamp>,

        // Current state of auction
        //highest_bidder: (storage::Value<AccountId>, storage::Value<Balance>),
        highest_bidder: storage::Value<(AccountId, Balance)>,

        // Set to true at the end, disallows any change
        ended: storage::Value<bool>,

        // Keep track of refunded bids so we can follow the withdraw pattern
        pending_returns: storage::HashMap<AccountId, Balance>,
    }


    impl OpenAuction {
        #[ink(constructor)]
        fn new(&mut self, beneficiary: AccountId, bidding_time: Timestamp) {
            let caller = self.env().caller();
            self.beneficiary.set(beneficiary);
            self.auction_start.set(self.env().block_timestamp());
            self.auction_end.set(self.env().block_timestamp() + bidding_time);
        }

        #[ink(message)]
        //fn bid(&mut self) -> bool {
        fn bid(&mut self, bid: u128) -> bool {
            if self.env().block_timestamp() >= *self.auction_end { // Check if bidding period is over
                return false;
            }

            let bidder = self.env().caller();
            //let bid = self.env().transferred_balance(); 
            let bid = bid as Balance;

            if bid <= self.highest_bidder.1 { // Check if bid is high enough
                return false;
            }

            self.pending_returns.insert(
                self.highest_bidder.0,
                self.pending_returns.get(&self.highest_bidder.0).unwrap_or(&0) + self.highest_bidder.1); // Track the refund for the previous high bidder

            self.highest_bidder.set((bidder, bid)); // Track new high bid

            true
        }

        // Withdraw a previously refunded bid. The withdraw pattern is
        // used here to avoid a security issue. If refunds were directly
        // sent as part of bid(), a malicious bidding contract could block
        // those refunds and thus block new higher bids from coming in.
        #[ink(message)]
        fn withdraw(&mut self) {
            let bidder = self.env().caller();
            //let &pending_amount = self.pending_returns.get(&bidder).unwrap_or(&0);
            //let pending_amount = self.pending_returns.get(&bidder).unwrap_or(&0);
            if let Some(&mut pending_amount) = self.pending_returns.get_mut(&bidder) {
                //*pending_amount = 0;
                //*self.pending_returns.get_mut(&bidder) = 0;
                let pending_amount = self.pending_returns.remove(&bidder).unwrap();
                self.send_safely(bidder, pending_amount);
            }
        }

        #[ink(message)]
        fn end_auction(&mut self) -> bool {
            if self.env().block_timestamp() < *self.auction_end { // Check if auction endtime
                return false;
            }
            if *self.ended { // Check if this function has already been called
                return false;
            }

            self.ended.set(true);

            self.send_safely(*self.beneficiary, self.highest_bidder.1);

            true
        }


        fn send_safely(&mut self, _addr: AccountId, _amount: Balance) {
            //TODO: do transaction
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_auction() {
        }
    }
}
