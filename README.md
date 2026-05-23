# Stellar Cat Adoption DApp

**Stellar Cat Adoption DApp** - Blockchain-Based Decentralized Cat Adoption System

## Project Description

Stellar Cat Adoption DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, transparent, and immutable platform for registering cats available for adoption and managing the adoption process directly on the blockchain. 

By utilizing smart contracts, this system eliminates the need for centralized shelter databases, ensuring that every cat's record is permanent, publicly verifiable, and securely handled through predefined contract functions.

## Project Vision

Our vision is to revolutionize animal welfare and pet adoption tracking in the digital age by:

- **Decentralizing Shelter Records**: Moving pet data from fragmented, centralized rescue servers to a global, distributed blockchain.
- **Ensuring Data Immutability**: Providing a permanent, tamper-proof registry of cats, preventing unauthorized alterations or loss of historical data.
- **Enhancing Transparency**: Allowing adopters to trace and verify a cat's registration, breed, description, and status with complete trust.
- **Building Trustless Systems**: Creating an adoption platform where state changes (like successful adoptions) are governed strictly by code, ensuring fair processing.

We envision a future where animal rescue operations are globally connected, highly transparent, and secure, giving every rescue pet a verifiable digital identity.

## Key Features

### 1. **Simple Cat Registration**
- Register a cat with a single smart contract function call.
- Provide comprehensive data including `name`, `breed`, `description`, and a direct link to an `image_url` (Image/GIF).
- Automated unique ID generation using Soroban's built-in Pseudo-Random Number Generator (PRNG) ranging from 100,000 to 1,000,000.
- Safe data persistence utilizing Stellar's persistent storage layout.

### 2. **Transparent Data Retrieval**
- Fetch the entire list of registered and available cats in a single network call.
- Structured data representation (`Cat` struct) designed for seamless frontend integration.
- Real-time synchronization with the on-chain blockchain state.

### 3. **Secure Adoption Processing**
- Adopt and claim a cat by passing its unique `u64` generated ID.
- Automatically and safely removes the cat from the available adoption pool upon successful execution.
- Instant, safe state updates handled outside loop executions to prevent race conditions or storage bloat.

### 4. **Stellar Network Integration**
- Leverages the high speed and near-zero transaction fees of Stellar.
- Built using the modern, safe, and robust Soroban Smart Contract SDK (`#![no_std]`).
- Scalable persistent storage architecture designed to handle long-term data lifecycles.

---

## Contract Details & API Reference

### Data Structures

```rust
#[contracttype]
#[derive(Clone, Debug)]
pub struct Cat {
    pub id: u64,
    pub name: String,
    pub breed: String,
    pub description: String,
    pub image_url: String, // Link to Cat Image/GIF
}