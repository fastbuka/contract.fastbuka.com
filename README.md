# Fast buka Escrow Soroban Contract.
The Fast Buka Escrow Soroban Contract is a smart contract that will be deployed on the Stellar network to facilitate secure and transparent payment flows for the Fast Buka food delivery application. The contract ensures that payments between the Customer, Vendor (Restaurant), and Rider (Delivery Personnel) are securely handled, with automatic fund distribution based on specific confirmations.



## Overview

- This contract is will be designed to:

1. Hold customer payments in escrow until both the customer and rider confirm the delivery.

2. Automatically distribute funds to the vendor and rider based on delivery confirmations.

3. Handle dispute resolution efficiently by determining the fault party and adjusting the payments accordingly.


The contract will support payments in any Stellar asset token, providing flexibility to accommodate various tokens as required by the Fast Buka platform.



## Payment Distribution Process

### Total Payment and Rider Fee Storage:
    - When the customer makes a payment, the total amount is stored in the contract along with the rider’s fee.
    - Upon successful delivery confirmation by both the customer and rider, the rider’s fee is deducted from the total amount. The remaining balance is sent to the vendor
### Dispute Handling:
    - In case of a dispute, the resolve_dispute() function allows the admin to refund the necessary party (e.g., the user or the vendor) and apply penalties (e.g., deducting amounts from the rider’s wallet if the rider is at fault).


## Sequence Diagram for Soroban Escrow contract

### Customer Makes Payment: 
- Customer sends payment to the contract in any Stellar asset token.
- Payment is held in escrow until delivery is confirmed.


### Rider Confirms Delivery:
- Rider confirms successful delivery on the contract.
- Contract then triggers a request for customer confirmation.


### Customer Confirms Receipt:
- Customer confirms food receipt.
- Contract releases the funds.
    - The rider’s fee is deducted first.
    - Remaining balance is distributed between the vendor and rider.

- Dispute Resolution:
    - If any party contests the delivery, the admin can use the resolve_dispute() method to determine fault and adjust payments accordingly.

