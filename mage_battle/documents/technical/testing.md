# Testing Guide

How to run and write tests for MageBattle.

## Running Tests

### Run All Tests

```bash
cd mage_battle
cargo test
```

### Run Specific Test File

```bash
cargo test buffs_debuffs
cargo test proficiency
cargo test battle_logic_issues
```

### Run Single Test

```bash
cargo test test_immune_blocks_damage
```

### Show Test Output

```bash
cargo test -- --nocapture
```

### Run Tests in Release Mode

```bash
cargo test --release
```

## Test Files

### tests/buffs_debuffs.rs

**Coverage**: All buff and debuff mechanics

**Tests**:
- Immune blocks damage and debuffs
- Invincible blocks damage
- Regeneration heals at turn start
- Burning Out bonus and cost
- Guard Wood Carving damage reduction
- Health Drain effect
- Paralysis prevents actions
- Seal prevents liberation
- Silent blocks spell cards
- Master Disable removes proficiency
- Defense Invalidation prevents healing/shield
- Confuse changes turn order
- Duration mechanics (Turns, UntilHit)

**Example**:
```rust
#[test]
fn test_immune_blocks_damage() {
    let mut game = Game::new(names, chars);
    game.players[0].hp = 50;
    game.players[0].buffs.add(Buff::new(BuffType::Immune, BuffDuration::Turns(2)));

    // Player 1 attacks Player 0
    game.current_player_index = 1;
    let _ = game.play_attribute_bolt(2, AttributeType::Fire, vec![0]);

    // Player 0's HP unchanged due to Immune
    assert_eq!(game.players[0].hp, 50);
}
```

### tests/proficiency.rs

**Coverage**: All attribute proficiency effects

**Tests**:
- Fire Lv3 bolt damage (+1)
- Fire Lv5 turn start damage (1 to all enemies)
- Thunder Lv3/5 damage bonuses (+1/+3)
- Wood Lv3 HP cost and shield gain
- Wood Lv5 spell damage reduction (-1)
- Water Lv3/5 healing bonuses
- Wind Lv2 Defense Invalidation
- Wind Lv5 free targeting
- Poison Lv2 Confuse debuff
- Poison Lv5 Silent debuff
- Burning Out buff
- Proficiency stacking (multi-attribute)
- Master Disable negates proficiencies

**Example**:
```rust
#[test]
fn test_fire_lv3_bolt_bonus() {
    let mut game = Game::new(names, chars);
    game.players[0].attributes.fire = 3;
    game.players[0].hand.push(1);

    let initial_hp = game.players[1].hp;
    let _ = game.play_attribute_bolt(1, AttributeType::Fire, vec![1]);

    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 4, "Fire Lv3 should deal 3 + 1 = 4 damage");
}
```

### tests/battle_logic_issues.rs

**Coverage**: Bug fixes and edge cases

**Tests**:
- Wood Lv3 HP cost bypasses shield
- Shield surplus damage doesn't carry over
- Liberation requirements
- Death and revival mechanics
- Turn order and phase transitions

### tests/new_mechanics_tests.rs

**Coverage**: New features

**Tests**:
- Attribute bolt implementation
- Card drawing mechanics
- Spell casting
- Target validation

## Writing Tests

### Test Structure

```rust
#[test]
fn test_name_describes_behavior() {
    // 1. Setup game state
    let names = vec!["P0".to_string(), "P1".to_string(),
                     "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::ThunderPoison,
        CharacterType::WaterWind,
    ];
    let mut game = Game::new(names, chars);

    // 2. Configure initial state
    game.players[0].hp = 50;
    game.players[0].attributes.fire = 3;

    // 3. Perform action
    let result = game.play_attribute_bolt(
        card_id: 1,
        AttributeType::Fire,
        targets: vec![1]
    );

    // 4. Assert expected behavior
    assert_eq!(game.players[1].hp, 46, "Expected 4 damage");
    assert!(result.is_ok(), "Action should succeed");
}
```

### Best Practices

1. **Descriptive Names**: Test name should describe the behavior
   - Good: `test_fire_lv5_damages_all_enemies`
   - Bad: `test_fire_proficiency`

2. **Single Responsibility**: Test one thing
   - Good: Separate tests for damage and debuff
   - Bad: Test that checks damage, debuff, and healing

3. **Clear Assertions**: Use descriptive failure messages
   ```rust
   assert_eq!(hp, 47, "Fire Lv5 should deal 1 damage");
   ```

4. **Setup Helpers**: Extract common setup code
   ```rust
   fn create_test_game() -> Game {
       let names = vec!["P0".to_string(), /* ... */];
       let chars = vec![CharacterType::FlamePoison, /* ... */];
       Game::new(names, chars)
   }
   ```

5. **Test Edge Cases**:
   - Minimum values (0 HP, 0 attributes)
   - Maximum values (max HP, Lv5 attributes)
   - Boundary conditions (exactly 0 HP, exactly Lv5)

### Testing Buffs

```rust
#[test]
fn test_buff_behavior() {
    let mut game = create_test_game();

    // Add buff
    let buff = Buff::new(BuffType::Regeneration, BuffDuration::Turns(2));
    game.players[0].buffs.add(buff);

    // Verify buff is present
    assert!(game.players[0].buffs.has(BuffType::Regeneration));

    // Trigger buff effect
    game.handle_turn_start();

    // Verify effect applied
    assert_eq!(game.players[0].hp, expected_hp);
}
```

### Testing Proficiency

```rust
#[test]
fn test_proficiency_effect() {
    let mut game = create_test_game();

    // Set attribute level
    game.players[0].attributes.fire = 3;

    // Perform action that triggers proficiency
    let initial_hp = game.players[1].hp;
    game.play_attribute_bolt(1, AttributeType::Fire, vec![1]);

    // Calculate and verify damage
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, expected_damage, "Proficiency bonus incorrect");
}
```

### Testing Error Cases

```rust
#[test]
fn test_error_condition() {
    let mut game = create_test_game();

    // Setup invalid state
    game.players[0].attributes.fire = 0;

    // Attempt action
    let result = game.play_attribute_bolt(1, AttributeType::Fire, vec![1]);

    // Verify error
    assert!(result.is_err(), "Should fail with 0 attribute level");
    assert_eq!(
        result.unwrap_err(),
        "Fire level is 0",
        "Error message should be clear"
    );
}
```

## Test Coverage

### Coverage by Feature

| Feature | Coverage | Tests |
|---------|----------|-------|
| Buffs/Debuffs | ✅ Comprehensive | 16 tests |
| Proficiency | ✅ Comprehensive | 15+ tests |
| Combat System | ✅ Good | 10+ tests |
| Death/Revival | ⚠️ Partial | 3 tests |
| Card System | ⚠️ Partial | 5 tests |
| Liberation | ⚠️ Partial | 2 tests |
| API Endpoints | ❌ Manual only | 0 tests |

### Generating Coverage Report

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html --output-dir coverage

# Open coverage/index.html in browser
```

## Integration Testing

### Testing API Endpoints

**Currently**: Manual testing only

**Future**: Add integration tests
```rust
#[tokio::test]
async fn test_create_game_endpoint() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/game/new")
                .header("content-type", "application/json")
                .body(Body::from(json_body))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

### Testing with curl

```bash
# Create game
curl -X POST http://localhost:3000/api/game/new \
  -H "Content-Type: application/json" \
  -d '{"player_names":["A","B","C","D"],"characters":["FlamePoison","WoodWind","ThunderPoison","WaterWind"]}'

# Get game state
curl http://localhost:3000/api/game/{game_id}

# Allocate attribute
curl -X POST http://localhost:3000/api/game/{game_id}/allocate \
  -H "Content-Type: application/json" \
  -d '{"attribute":"Fire"}'
```

## Frontend Testing

### Currently

**Status**: Manual testing only

### Potential Setup

**Unit Tests (Jest + React Testing Library)**:
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import App from './App';

test('renders game lobby', () => {
  render(<App />);
  expect(screen.getByText(/Create Game/i)).toBeInTheDocument();
});

test('allocates attribute on button click', async () => {
  render(<App />);
  const fireButton = screen.getByText(/火/i);
  fireEvent.click(fireButton);
  // Assert API call made
});
```

**E2E Tests (Playwright)**:
```typescript
test('complete game flow', async ({ page }) => {
  await page.goto('http://localhost:3001');

  // Create game
  await page.fill('input[name="player1"]', 'Alice');
  await page.click('button:has-text("創建遊戲")');

  // Allocate attribute
  await page.click('button:has-text("火")');

  // Verify game state updated
  await expect(page.locator('.attribute-fire')).toHaveText('1');
});
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all
      - name: Check formatting
        run: cargo fmt -- --check
      - name: Run clippy
        run: cargo clippy -- -D warnings
```

## Debugging Tests

### Print Debug Info

```rust
#[test]
fn test_with_debug() {
    let game = create_test_game();

    println!("Game state: {:#?}", game);
    println!("Player HP: {}", game.players[0].hp);

    // Run with: cargo test -- --nocapture
}
```

### Conditional Breakpoints

```rust
#[test]
fn test_with_breakpoint() {
    let mut game = create_test_game();

    for turn in 0..10 {
        game.next_turn();

        if game.players[0].hp < 20 {
            // Set breakpoint here in debugger
            panic!("HP too low on turn {}", turn);
        }
    }
}
```

### Test Isolation

Ensure tests don't affect each other:
- Create new game for each test
- Don't share mutable state
- Reset static/global state if any

## Performance Testing

### Benchmark Example

```rust
#[bench]
fn bench_play_attribute_bolt(b: &mut Bencher) {
    let mut game = create_test_game();

    b.iter(|| {
        game.play_attribute_bolt(1, AttributeType::Fire, vec![1]);
    });
}
```

## Next Topics

- [Architecture](architecture.md) - System design
- [API Reference](api-reference.md) - API endpoints to test
- [Contributing](../development/contributing.md) - How to contribute tests

---

For gameplay rules being tested, see [Gameplay Documentation](../gameplay/README.md).
