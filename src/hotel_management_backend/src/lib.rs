// Imorting necessary libraries
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define struct and types
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;


// ... [dependencies and imports similar to your original code] ...

// Define the Room Structure
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Room {
    id: u64,
    room_number: String,
    room_type: String,
    availability: bool,
    created_at: u64,
    updated_at: Option<u64>,
}

// Implement Storable and BoundedStorable for Room...

impl Storable for Room {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Room {
    const MAX_SIZE: u32 = 1024; // Example size, adjust as needed
    const IS_FIXED_SIZE: bool = false;
}


// Define the Reservation Structure
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Reservation {
    id: u64,
    guest_id: u64,
    room_id: u64,
    start_date: u64,
    end_date: u64,
    created_at: u64,
}

// Implement Storable and BoundedStorable for Reservation...

impl Storable for Reservation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Reservation {
    const MAX_SIZE: u32 = 1024; // Example size, adjust as needed
    const IS_FIXED_SIZE: bool = false;
}

// Define the Guest Structure
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Guest {
    id: u64,
    name: String,
    email: String,
    created_at: u64,
}

// Implement Storable and BoundedStorable for Guest...
impl Storable for Guest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Guest {
    const MAX_SIZE: u32 = 1024; // Example size, adjust as needed
    const IS_FIXED_SIZE: bool = false;
}




// Define `RoomPayload`, `ReservationPayload`, `GuestPayload` for CRUD operations
// Initialize memory storage for Room, Reservation, and Guest similar to the original code
// Define CRUD operations for each entity (Room, Reservation, Guest)
// Following the pattern of your original CRUD operations, adjust them to handle the new entities



// ... [rest of the code with adapted CRUD operations and relevant query/update methods] ...

// Define Memory manager and the ID counter 
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ROOM_STORAGE: RefCell<StableBTreeMap<u64, Room, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static RESERVATION_STORAGE: RefCell<StableBTreeMap<u64, Reservation, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

    static GUEST_STORAGE: RefCell<StableBTreeMap<u64, Guest, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );
}

// Define the RoomPayload structure for creating and Updating rooms 
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct RoomPayload {
    room_number: String,
    room_type: String,
    availability: bool,
}


// Function to create a room
#[ic_cdk::update]
fn create_room(payload: RoomPayload) -> Option<Room> {
    let id = generate_new_id();
    let room = Room {
        id,
        room_number: payload.room_number,
        room_type: payload.room_type,
        availability: payload.availability,
        created_at: time(),
        updated_at: None,
    };
    ROOM_STORAGE.with(|s| s.borrow_mut().insert(id, room.clone()));
    Some(room)
}

// Function that check the number of available rooms 
fn available_rooms_count(room_type: &str) -> u64 {
    ROOM_STORAGE.with(|rooms| {
        rooms.borrow().iter()
             .filter(|(_, room)| room.room_type == room_type && room.availability)
             .count() as u64
    })
}


// Function to get a single room by ID
#[ic_cdk::query]
fn get_room(id: u64) -> Result<Room, Error> {
    ROOM_STORAGE.with(|s| s.borrow().get(&id))
        .ok_or(Error::NotFound {
            msg: format!("Room with id={} not found.", id),
        })
}

// Function to update room
#[ic_cdk::update]
fn update_room(id: u64, payload: RoomPayload) -> Result<Room, Error> {
    ROOM_STORAGE.with(|s| {
        if let Some(mut room) = s.borrow_mut().get(&id) {
            room.room_number = payload.room_number;
            room.room_type = payload.room_type;
            room.availability = payload.availability;
            room.updated_at = Some(time());
            s.borrow_mut().insert(id, room.clone());
            Ok(room)
        } else {
            Err(Error::NotFound {
                msg: format!("Room with id={} not found.", id),
            })
        }
    })
}

// Function to delete room by Id
#[ic_cdk::update]
fn delete_room(id: u64) -> Result<Room, Error> {
    ROOM_STORAGE.with(|s| s.borrow_mut().remove(&id))
        .ok_or(Error::NotFound {
            msg: format!("Room with id={} not found.", id),
        })
}

// Define the ReservationPayload structure for creating and updating Reservation
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ReservationPayload {
    guest_id: u64,
    room_id: u64,
    start_date: u64,
    end_date: u64,
}

// Function to create a new Reservation 
#[ic_cdk::update]
fn create_reservation(payload: ReservationPayload) -> Result<Reservation, Error> {
    // Validate Date Range
    if payload.start_date >= payload.end_date {
        return Err(Error::InvalidDateRange {
            msg: "Start date must be before end date.".to_string(),
        });
    }

    // Check if the start and end dates are in the future
    let current_time = time();
    if payload.start_date < current_time || payload.end_date < current_time {
        return Err(Error::InvalidDateRange {
            msg: "Reservation dates must be in the future.".to_string(),
        });
    }
    // Check for overbooking
    let room_type = ROOM_STORAGE.with(|rooms| {
        rooms.borrow().get(&payload.room_id)
             .map(|room| room.room_type.clone())
             .unwrap_or_default()
    });
    let available_count = available_rooms_count(&room_type);
    if available_count == 0 {
        return Err(Error::Overbooking {
            msg: "No available rooms of the requested type.".to_string(),
        });
    }

    // Check Room Availability
    let is_room_available = RESERVATION_STORAGE.with(|reservations| {
        !reservations.borrow().iter()
            .any(|(_, reservation)| {
                reservation.room_id == payload.room_id &&
                !(payload.end_date <= reservation.start_date || payload.start_date >= reservation.end_date)
            })
    });

    if !is_room_available {
        return Err(Error::RoomUnavailable {
            msg: "Room is not available for the selected dates.".to_string(),
        });
    }

    // Create the Reservation
    let id = generate_new_id();
    let reservation = Reservation {
        id,
        guest_id: payload.guest_id,
        room_id: payload.room_id,
        start_date: payload.start_date,
        end_date: payload.end_date,
        created_at: current_time,
    };

    RESERVATION_STORAGE.with(|s| s.borrow_mut().insert(id, reservation.clone()));

    Ok(reservation)
}


// Function to get a reservation by Id
#[ic_cdk::query]
fn get_reservation(id: u64) -> Result<Reservation, Error> {
    RESERVATION_STORAGE.with(|s| s.borrow().get(&id))
        .ok_or(Error::NotFound {
            msg: format!("Reservation with id={} not found.", id),
        })
}


// Function to update the room_availability
#[ic_cdk::update]
fn update_room_availability(room_id: u64, is_available: bool) -> Result<(), Error> {
    ROOM_STORAGE.with(|rooms| {
        if let Some(mut room) = rooms.borrow_mut().get(&room_id) {
            room.availability = is_available;
            rooms.borrow_mut().insert(room_id, room);
            Ok(())
        } else {
            Err(Error::NotFound {
                msg: format!("Room with id={} not found.", room_id),
            })
        }
    })
}

// Function to delete reservation by id
#[ic_cdk::update]
fn delete_reservation(id: u64) -> Result<Reservation, Error> {
    // Retrieve the reservation to delete
    let maybe_reservation = RESERVATION_STORAGE.with(|reservations| {
        reservations.borrow_mut().remove(&id)
    });

    if let Some(reservation) = maybe_reservation {
        // Update the room's availability to true, as the reservation is being deleted
        update_room_availability(reservation.room_id, true)?;

        Ok(reservation)
    } else {
        Err(Error::NotFound {
            msg: format!("Reservation with id={} not found.", id),
        })
    }
}

// Define a GuestPayload structure for creating/updating Guests
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct GuestPayload {
    name: String,
    email: String,
}

// Function to generate a new id 
fn generate_new_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_value + 1);
        current_value
    })
}

// Function to create a guest 
#[ic_cdk::update]
fn create_guest(payload: GuestPayload) -> Option<Guest> {
    let id = generate_new_id();
    let guest = Guest {
        id,
        name: payload.name,
        email: payload.email,
        created_at: time(),
        // Initialize other fields...
    };
    GUEST_STORAGE.with(|s| s.borrow_mut().insert(id, guest.clone()));
    Some(guest)
}

// Function to get guest by id
#[ic_cdk::query]
fn get_guest(id: u64) -> Result<Guest, Error> {
    GUEST_STORAGE.with(|s| s.borrow().get(&id))
        .ok_or(Error::NotFound {
            msg: format!("Guest with id={} not found.", id),
        })
}

// Get all guests 
#[ic_cdk::query]
fn get_all_guests() -> Result<Vec<Guest>, Error>{
    let guests_map: Vec<(u64,Guest)> = GUEST_STORAGE.with(|service| service.borrow().iter().collect());
    let guests: Vec<Guest> = guests_map.into_iter().map(|(_, task)|task).collect();

    if !guests.is_empty() {
        Ok(guests)
    } else {
        Err(Error::NotFound {
            msg: "No tasks found.".to_string(),
         })
    }
}




// Function to update the guest details
#[ic_cdk::update]
fn update_guest(id: u64, payload: GuestPayload) -> Result<Guest, Error> {
    GUEST_STORAGE.with(|s| {
        if let Some(mut guest) = s.borrow_mut().get(&id) {
            guest.name = payload.name;
            guest.email = payload.email;
            // Update other fields...
            s.borrow_mut().insert(id, guest.clone());
            Ok(guest)
        } else {
            Err(Error::NotFound {
                msg: format!("Guest with id={} not found.", id),
            })
        }
    })
}

// Function to delete a guest 
#[ic_cdk::update]
fn delete_guest(id: u64) -> Result<Guest, Error> {
    GUEST_STORAGE.with(|s| s.borrow_mut().remove(&id))
        .ok_or(Error::NotFound {
            msg: format!("Guest with id={} not found.", id),
        })
}

// Define the error enum for handling errors
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    RoomUnavailable { msg: String },
    InvalidDateRange { msg: String },
    Overbooking { msg: String },
}


// Export candid interface
ic_cdk::export_candid!();
