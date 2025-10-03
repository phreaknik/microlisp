# MicroLisp Documentation

This directory contains comprehensive documentation for all Lisp instructions supported by the MicroLisp interpreter.

## Structure

- **[Core Functions](core-functions.md)**: Control flow, variable binding, and core language constructs
- **[Arithmetic Operations](arithmetic.md)**: Mathematical operations and numeric functions
- **[Comparison Operations](comparison.md)**: Comparison and equality operations
- **[Logical Operations](logical.md)**: Boolean logic operations
- **[Vector Operations](vectors.md)**: Vector creation and manipulation functions

## Quick Reference

### Core Functions
- `def` - Define variables
- `let` - Local variable binding
- `if` - Conditional expressions
- `do` - Sequential execution
- `while` - Loop while condition is true
- `doseq` - Iterate over sequences
- `dotimes` - Loop with counter

### Arithmetic Operations
- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division
- `rem` - Remainder
- `inc` - Increment by 1
- `dec` - Decrement by 1
- `max` - Maximum value
- `min` - Minimum value

### Comparison Operations
- `==` - Equality
- `>` - Greater than
- `>=` - Greater than or equal
- `<` - Less than
- `<=` - Less than or equal

### Logical Operations
- `and` - Logical AND
- `or` - Logical OR
- `not` - Logical NOT

### Vector Operations
- `vector` - Create vector
- `nth` - Get element by index
- `peek` - Get last element
- `pop` - Remove last element

## Data Types

MicroLisp supports the following data types:

- **Numbers**: 64-bit signed integers (`i64`)
- **Booleans**: `true` and `false`
- **Symbols**: Identifiers for variables and functions
- **Lists**: Parenthesized expressions for function calls
- **Vectors**: Square-bracketed collections `[1 2 3]`
- **Nil**: Empty/null value

## Truthiness

In MicroLisp, only `nil` and `false` are considered logically false. All other values, including the number `0`, are considered true.