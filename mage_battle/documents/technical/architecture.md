# Architecture

Technical overview of MageBattle's architecture, technology stack, and project structure.

## Technology Stack

### Backend

**Language & Runtime**:
- Rust 2021 Edition
- Compiled to native binary
- Memory-safe without garbage collection

**Web Framework**:
- **Axum 0.6**: Modern async web framework
- Built on Tokio runtime
- Type-safe routing and handlers

**Key Dependencies**:
- **Tokio 1.28**: Async runtime
- **Tower-HTTP 0.4**: CORS middleware
- **DashMap 5.5**: Concurrent state management
- **Serde**: JSON serialization/deserialization

**Database**:
- Currently in-memory (DashMap)
- Game state stored per game ID
- No persistent storage (resets on restart)

### Frontend

**Framework**:
- React 18
- TypeScript
- Functional components with hooks

**HTTP Client**:
- Axios for API calls
- Auto-retry on failures

**State Management**:
- React useState/useEffect
- localStorage for game ID persistence
- Poll-based updates (2-second interval)

**Styling**:
- CSS modules
- Responsive design
- Dark theme optimized

## System Architecture

### High-Level Overview

```
┌─────────────┐         HTTP/JSON          ┌──────────────┐
│   Browser   │ ◄────────────────────────► │ Axum Server  │
│  (React)    │    REST API (Port 3000)    │  (Rust)      │
└─────────────┘                             └──────────────┘
                                                    │
                                                    ▼
                                            ┌──────────────┐
                                            │   DashMap    │
                                            │ (Game State) │
                                            └──────────────┘
```

### Component Diagram

```
Frontend (React)
├── App.tsx                 # Main component & game logic
├── App.css                 # Styles
└── api/
    └── gameApi.ts          # API client functions

Backend (Rust)
├── main.rs                 # Entry point & CLI
├── web_server.rs           # Axum server & routes
├── api_types.rs            # Request/Response DTOs
├── game.rs                 # Core game logic
├── player.rs               # Player state
├── attribute.rs            # Attributes & proficiency
├── character.rs            # Character types
├── buff.rs                 # Buff/Debuff system
├── card.rs                 # Card & spell definitions
├── effect.rs               # Spell effects
└── damage.rs               # Damage types

Data Files
├── spells.json             # 60 spell cards data
└── cards.json              # Card metadata
```

## Backend Architecture

### Game State Management

**Storage**: DashMap (concurrent HashMap)
```rust
type GameStore = Arc<DashMap<String, Game>>;
```

**Key Features**:
- Thread-safe concurrent access
- No locks needed for reads
- Game isolated by unique ID
- Multiple games can run simultaneously

**Game Lifecycle**:
1. Client creates game → POST /api/game/new
2. Backend generates unique game ID
3. Game stored in DashMap
4. Clients poll for updates → GET /api/game/{id}
5. Actions modify game state → POST /api/game/{id}/action

### Request Flow

**Example: Play Attribute Bolt**

```
1. Client: POST /api/game/{id}/play_bolt
   Body: { card_id: 5, attribute: "Fire", targets: [3] }

2. Axum Router: Routes to play_attribute_bolt_handler()

3. Handler:
   - Deserialize request body
   - Get game from DashMap
   - Validate current player
   - Call game.play_attribute_bolt()

4. Game Logic:
   - Validate card in hand
   - Check attribute level
   - Calculate damage with proficiency
   - Apply to target
   - Update game state

5. Response: JSON { success: true, events: [...] }

6. Client: Receives response, updates UI
```

### Concurrency Model

**Async/Await**:
- All handlers are async
- Non-blocking I/O
- Can handle thousands of concurrent requests

**State Access**:
- DashMap provides lock-free reads
- Writes use fine-grained internal locks
- No manual mutex management needed

**Thread Safety**:
- Game state is mutable behind Arc<DashMap>
- Each handler gets immutable Arc reference
- Safe concurrent access guaranteed

## Frontend Architecture

### Component Structure

**Main Component: App.tsx**
```
App Component
├── State Management (useState)
│   ├── gameInfo: GameInfo | null
│   ├── mySlotId: number | null
│   └── previousGameInfo: GameInfo | null
├── Effects (useEffect)
│   ├── Load game ID from localStorage
│   ├── Poll game state every 2 seconds
│   └── Scroll to logs on update
└── Render
    ├── Game Lobby (if no game)
    ├── Player Selection (if game exists, no slot)
    └── Game Board (if game exists, has slot)
```

### State Management Strategy

**Local State**:
- `gameInfo`: Current game state from server
- `mySlotId`: Which player this client controls (0-3)
- `previousGameInfo`: For detecting changes (HP animations)

**Persistent State**:
- `localStorage`: Stores game ID across page refreshes
- Key: "mage_battle_game_id"

**Server State**:
- Source of truth
- Client polls every 2 seconds
- No WebSocket (simpler deployment)

### API Integration

**API Client (gameApi.ts)**:
```typescript
// Axios instance with base URL
const api = axios.create({
  baseURL: 'http://localhost:3000',
  headers: { 'Content-Type': 'application/json' }
});

// Typed API functions
export const gameApi = {
  createGame(names, characters): Promise<GameInfo>,
  getGameInfo(gameId): Promise<GameInfo>,
  allocateAttribute(gameId, attribute): Promise<ActionResult>,
  playAttributeBolt(gameId, cardId, attribute, targets): Promise<ActionResult>,
  // ...
};
```

**Error Handling**:
- Try/catch wraps all API calls
- Display errors in alert (temporary)
- Auto-retry on network failure (Axios)

## Data Models

### Core Types

**Game State**:
```rust
struct Game {
    players: Vec<Player>,
    current_player_index: usize,
    turn_number: u32,
    turn_phase: TurnPhase,
    deck: Vec<CardId>,
    discard_pile: Vec<CardId>,
    action_log: Vec<String>,
}
```

**Player State**:
```rust
struct Player {
    id: PlayerId,
    name: String,
    character: Character,
    team: Team,
    hp: i32,
    max_hp: i32,
    shield: u32,
    attributes: AttributePoints,
    buffs: BuffList,
    hand: Vec<CardId>,
    discard_pile: Vec<CardId>,
    is_dead: bool,
    death_turns: u32,
}
```

**API Response**:
```typescript
interface GameInfo {
  game_id: string;
  state: string;
  current_player_index: number;
  turn_number: number;
  turn_phase: string;
  players: Player[];
  deck_remaining: number;
  action_log: string[];
}
```

## Project Structure

```
mage_battle/
├── src/                           # Rust backend source
│   ├── main.rs                   # Entry point, CLI args
│   ├── web_server.rs             # Axum routes & handlers
│   ├── api_types.rs              # DTO types for API
│   ├── game.rs                   # Game logic & turn management
│   ├── player.rs                 # Player state & actions
│   ├── attribute.rs              # Attributes & proficiency
│   ├── character.rs              # Character types & liberation
│   ├── buff.rs                   # Buff/Debuff system
│   ├── card.rs                   # Card & spell loading
│   ├── effect.rs                 # Spell effect types
│   ├── damage.rs                 # Damage types
│   └── lobby.rs                  # Game creation (legacy)
├── tests/                         # Rust tests
│   ├── buffs_debuffs.rs          # Buff/debuff tests
│   ├── proficiency.rs            # Proficiency tests
│   ├── battle_logic_issues.rs    # Bug fix tests
│   └── new_mechanics_tests.rs    # Feature tests
├── frontend/                      # React frontend
│   ├── public/                   # Static assets
│   ├── src/
│   │   ├── App.tsx               # Main React component
│   │   ├── App.css               # Styles
│   │   ├── index.tsx             # React entry point
│   │   └── api/
│   │       └── gameApi.ts        # API client
│   ├── package.json              # NPM dependencies
│   └── tsconfig.json             # TypeScript config
├── documents/                     # Documentation (this folder)
├── Cargo.toml                     # Rust dependencies
├── spells.json                    # Spell card data
├── cards.json                     # Card metadata
├── start_server.sh/bat           # Backend start scripts
└── start_frontend.sh/bat         # Frontend start scripts
```

## Design Patterns

### Backend Patterns

**Repository Pattern**:
- DashMap acts as in-memory repository
- Game access abstracted through handlers
- Clean separation of concerns

**Builder Pattern**:
- `Game::new()` for game initialization
- `Player::new()` for player creation
- `Buff::new()` and `Buff::new_with_data()`

**Strategy Pattern**:
- Effect types handled polymorphically
- Different damage types processed differently
- Proficiency effects applied conditionally

### Frontend Patterns

**Container/Presentation**:
- App.tsx is container (logic + state)
- Inline components for presentation
- Potential for extraction to separate files

**Polling Pattern**:
- useEffect with setInterval
- Polls server every 2 seconds
- Simple, reliable, no WebSocket complexity

**Optimistic UI** (partial):
- Action buttons disabled during request
- Shows feedback immediately
- Updates from server poll

## Performance Considerations

### Backend

**Optimizations**:
- Release build (`--release`) for production
- DashMap for lock-free reads
- Minimal allocations in hot paths
- Efficient data structures (Vec, HashMap)

**Scaling**:
- Stateless handlers (can run multiple instances)
- Game state sharding by ID possible
- Memory-only (very fast)

**Bottlenecks**:
- Spell effect execution (complex logic)
- JSON serialization (per request)
- Polling creates constant load

### Frontend

**Optimizations**:
- React memoization potential
- Minimize re-renders
- Lazy load components (future)

**Network**:
- 2-second poll interval (balance updates vs load)
- Axios caching possible
- Batch actions in one request

## Deployment

### Local Development

**Backend**:
```bash
cargo run --release -- --web
# Runs on http://localhost:3000
```

**Frontend**:
```bash
cd frontend && npm start
# Runs on http://localhost:3001
```

### Production Considerations

**Backend**:
- Build with `cargo build --release`
- Run behind reverse proxy (nginx)
- Configure CORS for production domain
- Add persistent storage (SQLite/PostgreSQL)

**Frontend**:
- Build with `npm run build`
- Serve static files with nginx/Apache
- Set production API URL in env vars
- Enable gzip compression

**Infrastructure**:
- Deploy backend as systemd service or Docker
- Frontend as static site (Netlify, Vercel)
- Use environment variables for config
- Add logging and monitoring

## Security

### Current Status

**Not Production-Ready**:
- No authentication
- No rate limiting
- No input sanitization (beyond type checking)
- No HTTPS enforcement
- No game access control (anyone can access any game)

### Recommendations

For production deployment:
1. Add user authentication (JWT)
2. Implement rate limiting
3. Add game access tokens
4. Enable HTTPS only
5. Sanitize all inputs
6. Add request validation middleware
7. Implement game timeouts
8. Add audit logging

## Testing

### Backend Tests

**Location**: `tests/` directory

**Test Types**:
- Unit tests (inline in modules)
- Integration tests (tests/ folder)
- Property-based tests (potential)

**Coverage**:
- Buffs & debuffs: Comprehensive
- Proficiency system: Comprehensive
- Battle logic: Partial
- API endpoints: Manual testing

**Running Tests**:
```bash
cargo test                    # All tests
cargo test buffs_debuffs     # Specific test
cargo test -- --nocapture    # With output
```

### Frontend Testing

**Currently**: Manual testing only

**Potential**:
- Jest for unit tests
- React Testing Library
- E2E with Playwright/Cypress

## Next Steps

- [API Reference](api-reference.md) - REST API documentation
- [Proficiency System](proficiency-system.md) - Implementation details
- [Testing Guide](testing.md) - How to run and write tests

---

For more info, see [Project Status](../development/project-status.md).
