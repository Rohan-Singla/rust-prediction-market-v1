The goal is to build a clean, production-style MVP in pure Rust using Axum, Tokio, and in-memory state.

This is NOT a blockchain smart contract.
This is a centralized prediction market exchange engine inspired by Polymarket/Kalshi.

========================
PROJECT GOAL
========================

Build a binary prediction market backend where users can:

1. Create prediction markets
2. Split USD into YES/NO shares
3. Trade YES/NO shares using a limit orderbook
4. Match trades automatically
5. Merge YES+NO back into USD
6. Resolve markets
7. Claim winnings

The architecture should resemble a simplified exchange engine.

========================
TECH STACK
========================

- Rust
- Axum
- Tokio
- Serde
- UUID
- chrono

State management:
- Arc<Mutex<AppState>>

No database initially.
All data stored in-memory.

========================
CORE CONCEPTS
========================

Each binary market has:
- YES shares
- NO shares

At settlement:
- winning side = $1/share
- losing side = $0/share

Invariant:
YES + NO = $1

Users can:
- SPLIT cash into YES + NO
- MERGE YES + NO back into cash

========================
PROJECT STRUCTURE
========================

Create a clean modular architecture:

src/
 ├── main.rs
 ├── routes/
 │    ├── markets.rs
 │    ├── orders.rs
 │    ├── users.rs
 │    ├── positions.rs
 │
 ├── engine/
 │    ├── matching.rs
 │    ├── orderbook.rs
 │    ├── settlement.rs
 │
 ├── models/
 │    ├── market.rs
 │    ├── order.rs
 │    ├── trade.rs
 │    ├── user.rs
 │    ├── position.rs
 │
 ├── state.rs
 ├── errors.rs
 └── utils.rs

========================
MODELS
========================

Implement these models:

1. Market
2. Order
3. Trade
4. User
5. Position

========================
MARKET MODEL
========================

Fields:
- id
- question
- status
- resolved_outcome
- created_at

Market status enum:
- Active
- Resolved

Outcome enum:
- Yes
- No

========================
USER MODEL
========================

Fields:
- id
- name
- usd_balance

========================
POSITION MODEL
========================

Track YES/NO ownership per market.

Fields:
- user_id
- market_id
- yes_shares
- no_shares

========================
ORDER MODEL
========================

Fields:
- id
- user_id
- market_id
- side (Buy/Sell)
- outcome (Yes/No)
- price
- quantity
- remaining
- timestamp

========================
TRADE MODEL
========================

Fields:
- id
- market_id
- buyer_id
- seller_id
- outcome
- price
- quantity
- timestamp

========================
ORDERBOOK DESIGN
========================

Use:
BTreeMap<u64, Vec<Order>>

Maintain:
- bids_yes
- asks_yes
- bids_no
- asks_no

Implement price-time priority.

========================
MATCHING ENGINE RULES
========================

Trade executes when:
best_bid >= best_ask

Matching flow:
1. incoming order checks opposite book
2. execute trades
3. support partial fills
4. update positions
5. update balances
6. store remaining order in book

========================
TRADING RULES
========================

- limit orders only
- price range: 1-99
- quantity > 0
- market must be active

========================
SPLIT LOGIC
========================

User converts USD into YES + NO shares.

Example:
Split $100:
- subtract 100 USD
- add 100 YES
- add 100 NO

========================
MERGE LOGIC
========================

User redeems paired YES+NO into USD.

Example:
Merge 50:
- remove 50 YES
- remove 50 NO
- add 50 USD

========================
SETTLEMENT LOGIC
========================

Admin resolves market:
- YES or NO

Claim flow:
winning shares convert into USD.

========================
API ENDPOINTS
========================

MARKETS
POST   /markets
GET    /markets
GET    /markets/:id
POST   /markets/:id/resolve

ORDERS
POST   /orders
DELETE /orders/:id
GET    /markets/:id/orderbook
GET    /markets/:id/trades

TOKENS
POST   /markets/:id/split
POST   /markets/:id/merge

USERS
GET    /users/:id/balance
GET    /users/:id/positions

SETTLEMENT
POST   /markets/:id/claim

========================
IMPORTANT REQUIREMENTS
========================

- clean modular code
- proper Rust ownership patterns
- no unwrap() abuse
- proper error handling
- serde serialization
- async handlers
- reusable engine functions
- deterministic matching logic

========================
OPTIONAL FEATURES
========================

After MVP:
- websocket orderbook streaming
- candlestick generation
- persistent database
- auth
- market orders
- event sourcing
