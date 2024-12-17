use soroban_sdk::{
    Address,
    Symbol, contracterror,
};



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
}

// Status Enum
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

#[derive(Clone, PartialEq)]
pub enum DisputeResolution {
    UserFault,
    VendorFault,
    RiderFault,
}

// Event Structures
#[derive(Clone)]
pub struct OrderCreatedEvent {
    order_id: Symbol,
    user: Address,
    vendor: Address,
    amount: i128,
}

#[derive(Clone)]
pub struct OrderStatusUpdatedEvent {
    order_id: Symbol,
    old_status: OrderStatus,
    new_status: OrderStatus,
}

#[derive(Clone)]
pub struct ConfirmationGeneratedEvent {
    order_id: Symbol,
    vendor: Address,
}

#[derive(Clone)]
pub struct OrderPickedUpEvent {
    order_id: Symbol,
    rider: Address,
}

#[derive(Clone)]
pub struct DisputeEvent {
    order_id: Symbol,
    initiator: Address,
    reason: Symbol,
}

#[derive(Clone)]
pub struct DisputeResolvedEvent {
    order_id: Symbol,
    resolution: DisputeResolution,
    admin: Address,
}

// Main Order Structure
#[derive(Clone)]
pub struct Order {
    id: Symbol,
    user: Address,
    vendor: Address,
    amount: i128,
    status: OrderStatus,
    rider: Option<Address>,
    created_at: u64,
    confirmation_number: Option<u32>,
}