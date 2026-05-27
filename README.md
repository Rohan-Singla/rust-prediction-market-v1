# Prediction Market Exchange Engine (Rust, Axum, Tokio)

> **Note:** This is *not* a blockchain smart contract.  
> This project is a centralized prediction market exchange engine inspired by Polymarket and Kalshi.

---

## Project Goal

Build a **binary prediction market backend** where users can:

1. Create prediction markets
2. Split USD into YES/NO shares
3. Trade YES/NO shares using a limit orderbook
4. Match trades automatically
5. Merge YES+NO back into USD
6. Resolve markets
7. Claim winnings

The architecture is a simplified exchange engine.

---

## Tech Stack

- **Rust**
- **Axum** (web framework)
- **Tokio** (async runtime)
- **Serde** (serialization/deserialization)
- **UUID**
- **chrono** (date/time)
- **State Management:** `Arc<Mutex<AppState>>`
- No database initially — all data stored **in-memory**

---

## Core Concepts

Each binary market has:
- **YES shares**
- **NO shares**

At settlement:
- Winning side: **$1/share**
- Losing side: **$0/share**
- **Invariant:** YES + NO = $1

Users can:
- **Split** cash into YES + NO
- **Merge** YES + NO back into cash

---

## Project Structure

A clean, modular architecture:

```
src/
 ├── main.rs
 ├── routes/
 │    ├── markets.rs
 │    ├── orders.rs
 │    ├── users.rs
 │    ├── positions.rs
 ├── engine/
 │    ├── matching.rs
 │    ├── orderbook.rs
 │    ├── settlement.rs
 ├── models/
 │    ├── market.rs
 │    ├── order.rs
 │    ├── trade.rs
 │    ├── user.rs
 │    ├── position.rs
 ├── state.rs
 ├── errors.rs
 └── utils.rs
```

---

## Models

- **Market**
- **Order**
- **Trade**
- **User**
- **Position**

---

### Market Model

Fields:
- `id`
- `question`
- `status` (`Active` | `Resolved`)
- `resolved_outcome` (`Yes` | `No`)
- `created_at`

### User Model

Fields:
- `id`
- `name`
- `usd_balance`

### Position Model

Tracks YES/NO ownership per market.

Fields:
- `user_id`
- `market_id`
- `yes_shares`
- `no_shares`

### Order Model

Fields:
- `id`
- `user_id`
- `market_id`
- `side` (`Buy` | `Sell`)
- `outcome` (`Yes` | `No`)
- `price`
- `quantity`
- `remaining`
- `timestamp`

### Trade Model

Fields:
- `id`
- `market_id`
- `buyer_id`
- `seller_id`
- `outcome` (`Yes` | `No`)
- `price`
- `quantity`
- `timestamp`

---

## Orderbook Design

- Use: `BTreeMap<u64, Vec<Order>>`
- Maintain:
  - `bids_yes`
  - `asks_yes`
  - `bids_no`
  - `asks_no`
- Implement price-time priority matching

---

## Matching Engine Rules

- **Trades execute when:** `best_bid >= best_ask`

**Matching flow:**

1. Incoming order checks opposite book
2. Execute trades
3. Support partial fills
4. Update positions
5. Update balances
6. Store any remaining order in the book

---

## Trading Rules

- Only **limit** orders
- Price range: **1-99**
- Quantity > 0
- Market must be **active**

---

## Split Logic

User converts USD into YES + NO shares.

**Example:**  
Split $100:
- Subtract 100 USD
- Add 100 YES
- Add 100 NO

---

## Merge Logic

User redeems paired YES+NO into USD.

**Example:**  
Merge 50:
- Remove 50 YES
- Remove 50 NO
- Add 50 USD

---

## Settlement Logic

- Admin resolves market (YES or NO)
- Winning shares convert into USD at $1/share

---

## API Endpoints

### Markets

- `POST   /markets`         - Create market
- `GET    /markets`         - List markets
- `GET    /markets/:id`     - Get market details
- `POST   /markets/:id/resolve` - Resolve market

### Orders

- `POST   /orders`                - Place order
- `DELETE /orders/:id`            - Cancel order
- `GET    /markets/:id/orderbook` - Get orderbook
- `GET    /markets/:id/trades`    - Get trades

### Tokens

- `POST   /markets/:id/split`     - Split USD → YES/NO shares
- `POST   /markets/:id/merge`     - Merge YES+NO → USD

### Users

- `GET    /users/:id/balance`     - Get user balances
- `GET    /users/:id/positions`   - Get user positions

### Settlement

- `POST   /markets/:id/claim`     - Claim winnings

---

## Important Requirements

- Clean, modular code
- Proper Rust ownership patterns
- No `unwrap()` abuse
- Good error handling
- Serde serialization
- Async handlers
- Reusable engine functions
- Deterministic matching logic

---

## Optional Features (Post-MVP)

- WebSocket orderbook streaming
- Candlestick generation
- Persistent database
- Authentication
- Market orders
- Event sourcing

---
