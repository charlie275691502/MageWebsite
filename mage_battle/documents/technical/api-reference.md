# API Reference

Complete REST API documentation for MageBattle backend server.

## Base URL

```
http://localhost:3000
```

## Response Format

All endpoints return JSON in this format:

**Success Response**:
```json
{
  "success": true,
  "data": { /* response data */ }
}
```

**Error Response**:
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": "Additional details (optional)"
  }
}
```

## Game Management

### Create New Game

**Endpoint**: `POST /api/game/new`

**Description**: Creates a new 4-player game with specified names and characters.

**Request Body**:
```json
{
  "player_names": ["Alice", "Bob", "Charlie", "Diana"],
  "characters": ["FlamePoison", "WoodWind", "ThunderPoison", "WaterWind"]
}
```

**Parameters**:
- `player_names`: Array of 4 strings (player names)
- `characters`: Array of 4 character types:
  - `"FlamePoison"` - Fire/Poison Mage
  - `"WoodWind"` - Wood/Wind Mage
  - `"ThunderPoison"` - Thunder/Poison Mage
  - `"WaterWind"` - Water/Wind Mage

**Success Response (200)**:
```json
{
  "success": true,
  "data": {
    "game_id": "550e8400-e29b-41d4-a716-446655440000",
    "state": "Playing",
    "current_player_index": 0,
    "turn_number": 1,
    "turn_phase": "AllocateAttribute",
    "players": [ /* player data */ ],
    "deck_remaining": 40,
    "action_log": []
  }
}
```

**Error Codes**:
- `INVALID_INPUT`: Wrong number of players or invalid character types

---

### Get Game Info

**Endpoint**: `GET /api/game/{game_id}`

**Description**: Retrieves current game state.

**Path Parameters**:
- `game_id`: UUID string

**Success Response (200)**:
```json
{
  "success": true,
  "data": {
    "game_id": "550e8400-e29b-41d4-a716-446655440000",
    "state": "Playing",
    "current_player_index": 2,
    "turn_number": 5,
    "turn_phase": "PlayCard",
    "players": [
      {
        "id": 0,
        "name": "Alice",
        "character": {
          "character_type": "FlamePoison",
          "name": "火毒法師",
          "title": "燃燒殆盡",
          "is_liberated": false,
          "liberation_name": "Burning Out"
        },
        "team": "A",
        "hp": 50,
        "max_hp": 50,
        "shield": 0,
        "attributes": {
          "fire": 3,
          "wood": 0,
          "thunder": 1,
          "water": 0,
          "wind": 0,
          "poison": 2
        },
        "buffs": [
          {
            "buff_type": "Regeneration",
            "name": "再生",
            "duration": "Turns(2)",
            "data": null
          }
        ],
        "hand": [1, 5, 12, 23, 44],
        "hand_count": 5,
        "discard_pile_count": 2,
        "is_dead": false,
        "death_turns": 0,
        "can_act": true,
        "can_liberate": false
      }
      /* ... 3 more players */
    ],
    "deck_remaining": 35,
    "action_log": [
      "Alice 分配了 1 點到 火",
      "Alice 使用了技能 火屬性彈"
    ]
  }
}
```

**Error Codes**:
- `GAME_NOT_FOUND`: Game ID doesn't exist

---

### Get All Players

**Endpoint**: `GET /api/game/{game_id}/players`

**Description**: Get all players in the game.

**Path Parameters**:
- `game_id`: UUID string

**Success Response (200)**:
```json
{
  "success": true,
  "data": [ /* array of player objects */ ]
}
```

---

### Get Single Player

**Endpoint**: `GET /api/game/{game_id}/players/{player_id}`

**Description**: Get specific player information.

**Path Parameters**:
- `game_id`: UUID string
- `player_id`: Integer (0-3)

**Success Response (200)**:
```json
{
  "success": true,
  "data": { /* player object */ }
}
```

**Error Codes**:
- `INVALID_PLAYER_ID`: Player ID out of range

## Game Actions

### Allocate Attribute

**Endpoint**: `POST /api/game/{game_id}/allocate`

**Description**: Allocate 1 attribute point during Allocate Attribute phase.

**Path Parameters**:
- `game_id`: UUID string

**Request Body**:
```json
{
  "attribute": "Fire"
}
```

**Valid Attributes**:
- `"Fire"`, `"Wood"`, `"Thunder"`, `"Water"`, `"Wind"`, `"Poison"`

**Success Response (200)**:
```json
{
  "success": true,
  "data": {
    "success": true,
    "message": "Allocated 1 point to Fire",
    "events": [
      {
        "event_type": "AttributeAllocated",
        "message": "Alice 分配了 1 點到 火",
        "player_id": 0,
        "value": 1
      }
    ]
  }
}
```

**Error Codes**:
- `WRONG_PHASE`: Not in AllocateAttribute phase
- `ATTRIBUTE_MAXED`: Attribute already at level 5
- `NOT_YOUR_TURN`: Not current player's turn
- `PLAYER_DEAD`: Current player is dead

---

### Play Attribute Bolt

**Endpoint**: `POST /api/game/{game_id}/play_bolt`

**Description**: Use attribute bolt attack (requires discarding a card).

**Path Parameters**:
- `game_id`: UUID string

**Request Body**:
```json
{
  "card_id": 5,
  "attribute": "Fire",
  "targets": [3]
}
```

**Parameters**:
- `card_id`: Integer, card ID from player's hand
- `attribute`: String, one of the 6 attributes
- `targets`: Array of integers, target player IDs

**Success Response (200)**:
```json
{
  "success": true,
  "data": {
    "success": true,
    "message": "Attribute bolt successful",
    "events": [
      {
        "event_type": "Damage",
        "message": "Bob took 4 damage",
        "player_id": 3,
        "value": 4
      }
    ]
  }
}
```

**Error Codes**:
- `WRONG_PHASE`: Not in PlayCard phase
- `CARD_NOT_IN_HAND`: Card not in player's hand
- `ATTRIBUTE_TOO_LOW`: Attribute level is 0
- `INVALID_TARGET`: Target is dead or invalid

---

### Play Spell Card

**Endpoint**: `POST /api/game/{game_id}/play_card`

**Description**: Cast a spell from a card (top or bottom side).

**Path Parameters**:
- `game_id`: UUID string

**Request Body**:
```json
{
  "card_id": 12,
  "side": "Top",
  "targets": [1, 2]
}
```

**Parameters**:
- `card_id`: Integer, card ID from hand
- `side`: String, `"Top"` or `"Bottom"`
- `targets`: Array of integers, target player IDs

**Success Response (200)**:
```json
{
  "success": true,
  "data": {
    "success": true,
    "message": "Spell cast successfully",
    "events": [
      {
        "event_type": "SpellCast",
        "message": "Alice cast 火球術",
        "player_id": 0
      },
      {
        "event_type": "Damage",
        "message": "Bob took 10 damage",
        "player_id": 1,
        "value": 10
      }
    ]
  }
}
```

**Error Codes**:
- `WRONG_PHASE`: Not in PlayCard phase
- `CARD_NOT_IN_HAND`: Card not in hand
- `REQUIREMENTS_NOT_MET`: Don't have required attribute levels
- `INVALID_SIDE`: Bottom side doesn't exist for this card
- `INVALID_TARGETS`: Wrong number or type of targets

---

### Use Liberation Skill

**Endpoint**: `POST /api/game/{game_id}/liberate`

**Description**: Use character's liberation skill (once per game).

**Path Parameters**:
- `game_id`: UUID string

**Request Body**:
```json
{
  "targets": []
}
```

**Parameters**:
- `targets`: Array of integers (varies by character)

**Success Response (200)**:
```json
{
  "success": true,
  "data": {
    "success": true,
    "message": "Liberation skill activated",
    "events": [
      {
        "event_type": "Liberation",
        "message": "Alice 使用了解放技能",
        "player_id": 0
      }
    ]
  }
}
```

**Error Codes**:
- `REQUIREMENTS_NOT_MET`: Don't meet attribute requirements
- `ALREADY_LIBERATED`: Already used liberation
- `SEALED`: Have Seal debuff
- `PLAYER_DEAD`: Player is dead

---

### Draw Card

**Endpoint**: `POST /api/game/{game_id}/draw`

**Description**: Draw cards during Draw Card phase.

**Path Parameters**:
- `game_id`: UUID string

**Request Body**: None (empty POST)

**Success Response (200)**:
```json
{
  "success": true,
  "data": {
    "success": true,
    "message": "Drew 1 card",
    "events": [
      {
        "event_type": "CardDrawn",
        "message": "Alice drew a card",
        "player_id": 0,
        "value": 1
      }
    ]
  }
}
```

**Error Codes**:
- `WRONG_PHASE`: Not in DrawCard phase
- `DECK_EMPTY`: Deck and discard pile both empty (rare)

## Data Types

### GameInfo

```typescript
interface GameInfo {
  game_id: string;
  state: "Playing" | "Finished";
  current_player_index: number;  // 0-3
  turn_number: number;
  turn_phase: "AllocateAttribute" | "PlayCard" | "DrawCard";
  players: Player[];
  deck_remaining: number;
  action_log: string[];
}
```

### Player

```typescript
interface Player {
  id: number;  // 0-3
  name: string;
  character: Character;
  team: "A" | "B";
  hp: number;
  max_hp: number;
  shield: number;
  attributes: AttributePoints;
  buffs: Buff[];
  hand: number[];  // Card IDs (only visible for your player)
  hand_count: number;
  discard_pile_count: number;
  is_dead: boolean;
  death_turns: number;
  can_act: boolean;
  can_liberate: boolean;
}
```

### Character

```typescript
interface Character {
  character_type: "FlamePoison" | "WoodWind" | "ThunderPoison" | "WaterWind";
  name: string;  // Chinese name
  title: string;  // Chinese liberation name
  is_liberated: boolean;
  liberation_name: string;  // English liberation name
}
```

### AttributePoints

```typescript
interface AttributePoints {
  fire: number;     // 0-5
  wood: number;     // 0-5
  thunder: number;  // 0-5
  water: number;    // 0-5
  wind: number;     // 0-5
  poison: number;   // 0-5
}
```

### Buff

```typescript
interface Buff {
  buff_type: string;  // "Immune", "Regeneration", etc.
  name: string;  // Chinese name
  duration: string;  // "Turns(2)", "Permanent", etc.
  data: number | null;  // Optional data (e.g., target ID)
}
```

### ActionResult

```typescript
interface ActionResult {
  success: boolean;
  message: string;
  events: Event[];
}
```

### Event

```typescript
interface Event {
  event_type: string;
  message: string;
  player_id?: number;
  value?: number;
}
```

## Error Handling

### Common Error Codes

- `GAME_NOT_FOUND`: Invalid game ID
- `NOT_YOUR_TURN`: Action attempted by non-current player
- `WRONG_PHASE`: Action attempted in wrong turn phase
- `PLAYER_DEAD`: Player is dead and cannot act
- `INVALID_INPUT`: Malformed request data
- `REQUIREMENTS_NOT_MET`: Attribute requirements not satisfied
- `INVALID_TARGET`: Target player is invalid or dead

### Example Error Response

```json
{
  "success": false,
  "error": {
    "code": "NOT_YOUR_TURN",
    "message": "It's not your turn",
    "details": "Current player index: 2, You are: 0"
  }
}
```

## Rate Limiting

**Currently**: No rate limiting implemented

**Recommended for Production**:
- 100 requests per minute per IP
- 500 requests per hour per game ID
- Exponential backoff on repeated errors

## CORS

**Development**: Allows all origins (`*`)

**Production**: Should restrict to specific domains:
```rust
CorsLayer::new()
    .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([CONTENT_TYPE])
```

## WebSocket Support

**Currently**: Not implemented

**Polling Instead**:
- Frontend polls GET /api/game/{id} every 2 seconds
- Simple, reliable, works everywhere
- Higher latency than WebSocket

**Future Enhancement**:
- Add WebSocket endpoint for real-time updates
- Broadcast game state changes
- Reduce polling overhead

## Example Usage

### Creating and Playing a Game

```bash
# 1. Create game
curl -X POST http://localhost:3000/api/game/new \
  -H "Content-Type: application/json" \
  -d '{
    "player_names": ["Alice", "Bob", "Charlie", "Diana"],
    "characters": ["FlamePoison", "WoodWind", "ThunderPoison", "WaterWind"]
  }'
# Returns: { "success": true, "data": { "game_id": "abc123", ... } }

# 2. Get game state
curl http://localhost:3000/api/game/abc123

# 3. Allocate attribute (Alice's turn)
curl -X POST http://localhost:3000/api/game/abc123/allocate \
  -H "Content-Type: application/json" \
  -d '{"attribute": "Fire"}'

# 4. Play attribute bolt
curl -X POST http://localhost:3000/api/game/abc123/play_bolt \
  -H "Content-Type: application/json" \
  -d '{"card_id": 1, "attribute": "Fire", "targets": [3]}'

# 5. Draw card
curl -X POST http://localhost:3000/api/game/abc123/draw
```

## Next Topics

- [Architecture](architecture.md) - System design overview
- [Testing Guide](testing.md) - Testing the API
- [Project Status](../development/project-status.md) - Current features

---

For game rules, see [Gameplay Documentation](../gameplay/README.md).
