use soroban_sdk::{
    Address,
    Symbol, contracterror, contracttype, String
};


pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Escrow(String),
    Balance(Address),
    Allowance(AllowanceDataKey),
    Admin,

    // User storage
    Customer(Address),
    CustomerRegId(Address),
    OrderCounter,

    // vendor
    Vendor(Address),

    // rider
    Rider(Address),
}

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
    VendorPaymentFailed = 9,
    DisputeAlreadyExists = 10,
    DisputeNotFound = 11,
    NotAdmin = 12,
    AlreadyResolved = 13,
    InsufficientBalance = 14,
    OrderNotDelivered = 15,
    CalculationError = 16,
    RiderPaymentFailed = 17,
    DepositPaymentFailed = 18,
}



// Status Enum
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum OrderStatus {
    Waiting = 0,
    ReadyForPickup = 1,
    PickedUp = 2,
    Delivered = 3,
    Completed = 4,
    Cancelled = 5,
    Disputed = 6,
    Resolved = 7,
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
#[derive(Clone, Debug)]
pub struct Order {
    pub id: u128,
    pub user: Address,
    pub token: Address,
    pub vendor: Address,
    pub amount: i128,
    pub rider_fee: i128,
    pub status: OrderStatus,
    pub rider: Option<Address>,
    pub created_at: u64,
    pub confirmation_number: Option<u32>,
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}