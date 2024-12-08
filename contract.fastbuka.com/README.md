# Fast buka Escrow Soroban Contract.

- A Soroban smart contract for secure food delivery payments on the Stellar network, powering the Fast Buka food delivery application.

## Overview

- The Fast Buka Escrow smart contract manages payment flows between three parties:

1. Users (Customers)
2. Vendors (Restaurants)
3. Riders (Delivery Personnel)

The contract implements a secure escrow system where:
- Customer payments are held in escrow until delivery is confirmed.
- Funds are automatically distributed based on confirmations.
- An admin can resolve disputes.
- For now, Payments are handled in NGNC tokens (Nigerian Naira stablecoin).


## Features
### Payment Distribution

- 98% of payment goes to the vendor.
- 2% goes to the rider.
- Automatic distribution upon successful delivery confirmation.

### Multi-Party Confirmation

- User must confirm receive of food.
- Rider must confirm delivery.
- Both confirmations trigger automatic payment distribution.

### Dispute Resolution

The contract includes comprehensive dispute resolution:
1. User Fault
    - Normal payment processing proceeds
    - Vendor receives 98%.
    - Rider receives 2%.

2. Rider Fault.
    - Vendor receives their share.
    - User gets refunded.
    - Amount deducted from rider's wallet.

3. Vendor Fault.
    - User receives refund of vendor's share.
    - Rider still receives their commission.
    - Vendor receives nothing.
