# Contributing Guide

Welcome! Thank you for considering contributing to MageBattle.

## How to Contribute

### 1. Report Bugs

**Before Reporting**:
- Check existing issues on GitHub
- Verify bug exists in latest version
- Check if it's already documented in [Known Limitations](project-status.md#known-limitations)

**Bug Report Should Include**:
- Clear title describing the issue
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, browser, versions)
- Screenshots if applicable
- Relevant logs or error messages

**Template**:
```markdown
**Bug Description**
Brief description of the problem

**Steps to Reproduce**
1. Start a new game
2. Allocate Fire attribute
3. Use Fire bolt
4. Observe error

**Expected**
Fire bolt should deal damage

**Actual**
Game crashes with error "..."

**Environment**
- OS: macOS 14
- Browser: Chrome 120
- Backend: Rust 1.75
```

### 2. Suggest Features

**Before Suggesting**:
- Check [Roadmap](project-status.md#roadmap)
- Search existing feature requests
- Consider if it fits the game's vision

**Feature Request Should Include**:
- Clear use case or problem it solves
- Proposed solution or implementation idea
- Alternative solutions considered
- Impact on existing features

### 3. Code Contributions

#### Getting Started

**Fork and Clone**:
```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/mage_battle.git
cd mage_battle
```

**Set Up Development**:
```bash
# Backend
cargo build
cargo test

# Frontend
cd frontend
npm install
npm start
```

**Create Branch**:
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

#### Code Style

**Rust**:
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Follow Rust naming conventions
- Add comments for complex logic
- Write doc comments for public APIs

**TypeScript/React**:
- Use functional components
- Use TypeScript strict mode
- Follow existing naming patterns
- Add JSDoc comments for complex functions
- Use meaningful variable names

**Example**:
```rust
/// Applies damage to a player considering all modifiers
///
/// # Arguments
/// * `target_id` - The player receiving damage
/// * `base_damage` - Damage before modifiers
/// * `damage_type` - Type of damage (Spell, Skill, Direct)
///
/// # Returns
/// Final damage dealt after all reductions
pub fn apply_damage(
    &mut self,
    target_id: PlayerId,
    base_damage: u32,
    damage_type: DamageType,
) -> u32 {
    // Implementation
}
```

#### Writing Tests

**All New Features Need Tests**:
- Add tests in appropriate file (`tests/` directory)
- Test happy path and edge cases
- Use descriptive test names
- Include comments explaining what's being tested

**Example**:
```rust
#[test]
fn test_new_feature_works_correctly() {
    // Setup
    let mut game = create_test_game();
    game.players[0].attributes.fire = 5;

    // Execute
    let result = game.new_feature();

    // Verify
    assert!(result.is_ok());
    assert_eq!(game.players[0].hp, expected_hp);
}
```

#### Commit Messages

Use conventional commits:
```
type(scope): description

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding/fixing tests
- `refactor`: Code refactoring
- `style`: Formatting changes
- `chore`: Maintenance tasks

**Examples**:
```
feat(proficiency): add Wind Lv5 free targeting

fix(buffs): Immune now correctly blocks debuffs

docs(gameplay): update combat system documentation

test(proficiency): add tests for Thunder Lv5

refactor(game): extract damage calculation to separate function
```

#### Pull Request Process

1. **Update Documentation**:
   - Update relevant docs in `documents/`
   - Add entry to changelog (if exists)
   - Update API docs if changing endpoints

2. **Run Tests**:
   ```bash
   cargo test
   cargo fmt -- --check
   cargo clippy
   ```

3. **Create Pull Request**:
   - Use descriptive title
   - Reference related issues (`Fixes #123`)
   - Describe changes made
   - Include screenshots for UI changes
   - List breaking changes (if any)

4. **PR Template**:
   ```markdown
   ## Description
   Brief description of changes

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing
   - [ ] Tests added/updated
   - [ ] All tests pass
   - [ ] Manual testing performed

   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Documentation updated
   - [ ] No breaking changes (or documented)
   - [ ] Commit messages follow convention
   ```

5. **Code Review**:
   - Address reviewer feedback promptly
   - Push additional commits to same branch
   - Mark conversations as resolved
   - Be open to suggestions

### 4. Documentation Contributions

**Always Welcome**:
- Fix typos and grammar
- Clarify confusing sections
- Add examples
- Translate to other languages (future)
- Improve diagrams

**Documentation Structure**:
```
documents/
├── README.md               # Main entry point
├── setup/                  # Installation and getting started
├── gameplay/               # Game rules and mechanics
├── technical/              # Architecture and implementation
└── development/            # Contributing and status
```

**When Updating Docs**:
- Keep consistent formatting
- Use proper markdown syntax
- Add links between related docs
- Include code examples
- Test all commands/code snippets

## Priority Areas

### High Priority

1. **API Integration Tests**:
   - Test all endpoints
   - Test error cases
   - Test concurrent requests

2. **Frontend Tests**:
   - Unit tests for components
   - Integration tests for user flows
   - E2E tests with Playwright

3. **UI/UX Improvements**:
   - Mobile responsiveness
   - Animations
   - Better error messages
   - Loading states

### Medium Priority

4. **Performance Optimization**:
   - Reduce poll frequency intelligently
   - Optimize render cycles
   - Cache API responses

5. **Code Quality**:
   - Refactor large functions
   - Improve error handling
   - Add more comments
   - Extract reusable components

### Low Priority

6. **New Features** (see [Roadmap](project-status.md#roadmap)):
   - WebSocket support
   - AI opponent
   - Statistics tracking

## Development Workflow

### Local Development

1. **Start Backend**:
   ```bash
   cargo run --release -- --web
   # Runs on http://localhost:3000
   ```

2. **Start Frontend**:
   ```bash
   cd frontend
   npm start
   # Runs on http://localhost:3001
   ```

3. **Make Changes**:
   - Edit code in your branch
   - Test manually in browser
   - Run automated tests
   - Check for warnings

4. **Debug**:
   - Backend: Use `println!` or rust-gdb
   - Frontend: Browser DevTools (F12)
   - Logs: Check both terminal outputs

### Testing Locally

```bash
# Run all tests
cargo test

# Run specific test file
cargo test buffs_debuffs

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt -- --check

# Check for issues
cargo clippy
```

### Before Submitting PR

**Checklist**:
- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] PR description is complete
- [ ] No debug code left in (println!, console.log, etc.)
- [ ] Code is formatted (cargo fmt)
- [ ] No clippy warnings

## Community Guidelines

### Be Respectful

- Be kind and courteous
- Respect different opinions
- Focus on the issue, not the person
- Help others learn

### Communication

- Use clear, concise language
- Provide context in issues and PRs
- Respond to feedback constructively
- Ask questions when unclear

### Code of Conduct

We follow standard open source etiquette:
- Harassment of any kind is not tolerated
- Be welcoming to newcomers
- Give credit where credit is due
- Follow project maintainers' decisions

## Questions?

**Need Help?**:
- Open a GitHub issue with `question` label
- Check existing documentation
- Ask in pull request comments

**Contact**:
- GitHub Issues: For bugs and features
- Pull Requests: For code review
- Discussions: For general questions (if enabled)

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md (if it exists)
- Mentioned in release notes
- Credited in documentation changes

## Thank You!

Every contribution, no matter how small, is valuable. Thank you for helping make MageBattle better!

---

## Useful Links

- [Project Status](project-status.md) - Current state and roadmap
- [Architecture](../technical/architecture.md) - System design
- [Testing Guide](../technical/testing.md) - How to test
- [Game Rules](../gameplay/game-rules.md) - Understanding the game

## Getting Help

Stuck? Check these resources first:
1. [Documentation](../README.md)
2. [Existing Issues](https://github.com/yourusername/mage_battle/issues)
3. [Project Status](project-status.md#known-limitations)
4. Ask a question in new issue

Happy Contributing! 🎉
