# Logical Operations

Logical operations work with boolean values and follow MicroLisp's truthiness rules. Only `nil` and `false` are considered false; all other values are considered true.

## and

**Syntax**: `(and values...)`

Logical AND operation on zero or more values.

**Parameters**:
- `values...` - Zero or more expressions to evaluate

**Returns**: `true` if all values are truthy, `false` if any value is falsy

**Examples**:
```lisp
(and)                    ; Returns true (identity)
(and true)               ; Returns true
(and true true)          ; Returns true
(and true false)         ; Returns false
(and false true)         ; Returns false
(and 1 2 3)              ; Returns true (all numbers are truthy)
(and 1 nil 3)            ; Returns false (nil is falsy)
```

**Truthiness Rules**:
- `nil` and `false` are falsy
- All other values (including `0`, empty strings, empty vectors) are truthy

**Short-Circuit Evaluation**:
The implementation evaluates all arguments, but conceptually AND short-circuits on the first falsy value.

**Source**: `src/builtins/operators.rs:195`

---

## or

**Syntax**: `(or values...)`

Logical OR operation on zero or more values.

**Parameters**:
- `values...` - Zero or more expressions to evaluate

**Returns**: `true` if any value is truthy, `false` if all values are falsy

**Examples**:
```lisp
(or)                     ; Returns false (identity)
(or false)               ; Returns false
(or false false)         ; Returns false
(or true false)          ; Returns true
(or false true)          ; Returns true
(or nil false)           ; Returns false (both are falsy)
(or nil 1)               ; Returns true (1 is truthy)
(or 0 false)             ; Returns true (0 is truthy)
```

**Truthiness Rules**:
- `nil` and `false` are falsy
- All other values (including `0`, empty strings, empty vectors) are truthy

**Short-Circuit Evaluation**:
The implementation evaluates all arguments, but conceptually OR short-circuits on the first truthy value.

**Source**: `src/builtins/operators.rs:207`

---

## not

**Syntax**: `(not value)`

Logical NOT operation on a single value.

**Parameters**:
- `value` - The expression to negate

**Returns**: `true` if the value is falsy, `false` if the value is truthy

**Examples**:
```lisp
(not true)               ; Returns false
(not false)              ; Returns true
(not nil)                ; Returns true
(not 0)                  ; Returns false (0 is truthy)
(not 42)                 ; Returns false (42 is truthy)
(not "")                 ; Returns false (empty string is truthy)
(not [])                 ; Returns false (empty vector is truthy)
```

**Truthiness Rules**:
- `nil` and `false` are falsy, so `(not nil)` and `(not false)` return `true`
- All other values are truthy, so `(not value)` returns `false` for any other value

**Source**: `src/builtins/operators.rs:219`

---

## Truthiness in MicroLisp

Understanding truthiness is crucial for logical operations:

### Falsy Values
Only these values are considered false:
- `nil`
- `false`

### Truthy Values
All other values are considered true, including:
- `true`
- All numbers (`0`, `1`, `-1`, etc.)
- All strings (including empty strings)
- All vectors (including empty vectors)
- All other data types

### Examples with Different Data Types

```lisp
; Numbers (all truthy)
(if 0 "truthy" "falsy")         ; Returns "truthy"
(if -1 "truthy" "falsy")        ; Returns "truthy"
(if 42 "truthy" "falsy")        ; Returns "truthy"

; Booleans
(if true "truthy" "falsy")      ; Returns "truthy"
(if false "truthy" "falsy")     ; Returns "falsy"

; Nil
(if nil "truthy" "falsy")       ; Returns "falsy"

; Vectors (all truthy, even empty ones)
(if [] "truthy" "falsy")        ; Returns "truthy"
(if [1 2 3] "truthy" "falsy")   ; Returns "truthy"
```

## Combining Logical Operations

Logical operations can be combined to create complex conditions:

```lisp
; Complex AND condition
(and (> x 0) (< x 100) (== (rem x 2) 0))  ; x is even and between 0-100

; Complex OR condition
(or (== status "active") (== status "pending") (== status "ready"))

; Negation with compound conditions
(not (and (< x 0) (> x 100)))  ; x is NOT (negative OR greater than 100)

; De Morgan's law examples
(not (and a b))     ; Equivalent to (or (not a) (not b))
(not (or a b))      ; Equivalent to (and (not a) (not b))
```

## Common Patterns

### Default Values
```lisp
(or user-input default-value)   ; Use default if user-input is falsy
```

### Validation
```lisp
(and (> age 0) (< age 150))     ; Validate age is reasonable
```

### Conditional Assignment
```lisp
(if (and valid-user logged-in)
    "Welcome!"
    "Please log in")
```