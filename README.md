# Hotel Management System using Internet Computer(IC)

Welcome to your new hotel_management project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with hotel_management, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd hotel_management/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```


# Explanation of each function in  hotel-management

## Room Functions

`create_room(payload: RoomPayload) -> Option<Room>`

Creates a new room using the provided payload and adds it to the room storage. It returns the created room if successful.

`available_rooms_count(room_type: &str) -> u64`

Counts the number of available rooms for a given room 
type.

`get_room(id: u64) -> Result<Room, Error>`

Retrieves a room by its ID. Returns the room if found; otherwise, returns a `NotFound` error.

`update_room(id: u64, payload: RoomPayload) -> Result<Room, Error>`

Updates room details based on the provided payload. Returns the updated room if successful; otherwise, returns a` NotFound` error.

`delete_room(id: u64) -> Result<Room, Error>`
Deletes a room by its ID. Returns the deleted room if successful; otherwise, returns a `NotFound` error.

## Reservation Functions 
`create_reservation(payload: ReservationPayload) -> Result<Reservation, Error>`

Creates a new reservation based on the provided payload. Validates the date range, checks availability, and creates the reservation if conditions are met. Returns the created reservation if successful; otherwise, returns an appropriate error.

`get_reservation(id: u64) -> Result<Reservation, Error>`

Retrieves a reservation by its ID. Returns the reservation if found; otherwise, returns a `NotFound` error.

`update_room_availability(room_id: u64, is_available: bool) -> Result<(), Error>`

Updates the availability of a room based on the  provided ID. Returns `Ok(())` if successful; otherwise, returns a `NotFound` error.


`delete_reservation(id: u64) -> Result<Reservation, Error>`
Deletes a reservation by its ID. Returns the deleted reservation if successful; otherwise, returns a `NotFound` error.

## Guest Functions
`create_guest(payload: GuestPayload) -> Option<Guest>`

Creates a new guest based on the provided payload and adds it to the guest storage. Returns the created guest if successful.

`get_guest(id: u64) -> Result<Guest, Error>`

Retrieves a guest by their ID. Returns the guest if found; otherwise, returns a `NotFound` error.

`get_all_guests() -> Result<Vec<Guest>, Error>`

Retrieves all guests stored. Returns a vector of guests if there are any; otherwise, returns a `NotFound` error.

`update_guest(id: u64, payload: GuestPayload) -> Result<Guest, Error>`

Updates guest details based on the provided payload. Returns the updated guest if successful; otherwise, returns a `NotFound` error.

`delete_guest(id: u64) -> Result<Guest, Error>`

Deletes a guest by their ID. Returns the deleted guest if successful; otherwise, returns a `NotFound` error.

## Error Handling 

An `Error` enum is used to handle various error types, including `NotFound`, `RoomUnavailable`, `InvalidDateRange`, and `Overbooking`.

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
# hotel-management
