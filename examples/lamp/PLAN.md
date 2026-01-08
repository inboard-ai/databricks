# Genie CLI REPL Plan

A Claude Code-inspired TUI for chatting with Databricks Genie.

## UI Flow

1. **Startup**: List available Genie spaces, let user select one
2. **REPL Loop**:
   - Prompt for question (text input)
   - Show "Thinking..." while Genie processes
   - Display generated SQL (if any)
   - Execute SQL and show results in pretty table
   - Show suggested follow-up questions as selectable options
   - User can select a suggestion OR type a new question
   - Repeat

## Crates

- `inquire` - Interactive prompts (Select for space/suggestions, Text for questions)
- `comfy-table` - Pretty table output for SQL results
- `syntect` - SQL syntax highlighting (nice to have)

## Package Structure

```
examples/cli/
├── Cargo.toml
└── src/
    └── main.rs
```

## Key Features

1. **Space Selection**
   ```
   ? Select a Genie space:
   > Bakehouse Sales Starter Space
     Other Space...
   ```

2. **Question Input**
   ```
   > What are the top 5 products?
   ```

3. **Response Display**
   ```
   Generated SQL:
   ┌─────────────────────────────────────────┐
   │ SELECT product, SUM(totalPrice)...     │
   └─────────────────────────────────────────┘

   Results:
   ┌────────────────────────┬───────────────┐
   │ product                │ total_revenue │
   ├────────────────────────┼───────────────┤
   │ Golden Gate Ginger     │ 11595         │
   │ Outback Oatmeal        │ 11199         │
   └────────────────────────┴───────────────┘
   ```

4. **Follow-up Selection**
   ```
   ? Continue:
   > What are the top 5 franchises by total revenue?
     What is the total revenue by product category?
     [Enter new question]
   ```

## Implementation Steps

1. Create `examples/cli/` with Cargo.toml
2. Basic REPL with inquire Text prompt
3. Add space selection with inquire Select
4. Add comfy-table for results
5. Add follow-up question selection
6. Polish: colors, spinners, error handling
