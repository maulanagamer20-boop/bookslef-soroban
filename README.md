# 📒 Soroban Todo Notes Smart Contract

## 📖 Description

This project is a decentralized Todo Notes application built using Soroban Smart Contracts on the Stellar network.

Users can create, view, and manage notes directly on-chain. Each note includes a title, content, and completion status.

---

## 🚀 Features

* Create new notes
* View all stored notes
* Delete notes
* Mark notes as completed / uncompleted
* Fully stored on blockchain (Soroban)

---

## 🛠️ Technologies Used

* Rust (Soroban SDK)
* Stellar Soroban Smart Contract
* WASM (WebAssembly)

---

## 📦 Smart Contract ID (Testnet)

```
PASTE YOUR CONTRACT ID HERE
```

---

## 🖼️ Screenshot (Testnet Interaction)

(Add screenshots here showing contract interaction on testnet)

---

## ⚙️ How to Run

### 1. Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 2. Deploy to Testnet

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/notes.wasm \
--source YOUR_ACCOUNT \
--network testnet
```

---

## 📌 Notes

This project was created as part of a Soroban Smart Contract workshop submission.

---

## 👤 Author

Your Name
