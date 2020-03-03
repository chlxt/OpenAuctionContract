# A Simulated Open Auction Smart Contract Implementation Using Substrate

This project implements a simple smart contract for open auction, currently only simulated(virtual) coin (i.e., bidder's input value) is used as collateral.


## TODOs
1. Currently bid coin is not returned to bidder immediately when does new bid, which causes a large amount of bidder's coin has to be locked in contract.
2. Implement real coin transfer method using substrate's Balance runtime module.
