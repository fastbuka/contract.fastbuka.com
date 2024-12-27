use soroban_sdk::{
    Address,
    Symbol, contracterror, contracttype, String
};




// #[contracttype]
// #[derive(Clone, PartialEq, Eq)]
// pub enum Datakey {
//     OrderCount(u128),
// }


// Error definitions
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FastBukaError {
    InvalidAmount = 1,
    InvalidStatus = 2,
    InvalidCaller = 3,
    InvalidConfirmationNumber = 4,
    OrderNotFound = 5,
    OrderNotReady = 6,
    InvalidStatusTransition = 7,
    UnauthorizedAccess = 8,
    PaymentFailed = 9,
    DisputeAlreadyExists = 10,
    DisputeNotFound = 11,
    NotAdmin = 12,
    AlreadyResolved = 13,
    InsufficientBalance = 14,
}



// Status Enum
#[contracttype]
#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum OrderStatus {
    Waiting = 0,
    Accepted = 1,
    Preparing = 2,
    ReadyForPickup = 3,
    PickedUp = 4,
    Delivered = 5,
    Completed = 6,
    Cancelled = 7,
    Disputed = 8,
    Resolved = 9,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum DisputeResolution {
    UserFault,
    VendorFault,
    RiderFault,
}

// Event Structures
#[contracttype]
#[derive(Clone)]
pub struct OrderCreatedEvent {
    pub count: u128,
    pub user: Address,
    pub vendor: Address,
    pub amount: u128,
}

#[contracttype]
#[derive(Clone)]
pub struct OrderStatusUpdatedEvent {
    pub order_id: u128,
    pub old_status: OrderStatus,
    pub new_status: OrderStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct ConfirmationGeneratedEvent {
    pub order_id: u128,
    pub vendor: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct OrderPickedUpEvent {
    pub order_id: u128,
    pub rider: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct DisputeEvent {
    pub order_id: u128,
    pub initiator: Address,
    pub reason: String,
}

#[contracttype]
#[derive(Clone)]
pub struct DisputeResolvedEvent {
    pub order_id: u128,
    pub resolution: DisputeResolution,
    pub admin: Address,
}

// Main Order Structure
#[contracttype]
#[derive(Clone)]
pub struct Order {
    pub id: u128,
    pub user: Address,
    pub token: Address,
    pub vendor: Address,
    pub amount: i128,
    pub rider_fee: u128,
    pub status: OrderStatus,
    pub rider: Option<Address>,
    pub created_at: u64,
    pub confirmation_number: Option<u32>,
}