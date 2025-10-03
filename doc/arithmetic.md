# Arithmetic Operations

Arithmetic operations provide mathematical computations on numbers. All operations work with 64-bit signed integers (`i64`) and include overflow protection.

## +

**Syntax**: `(+ numbers...)`

Addition of zero or more numbers.

**Parameters**:
- `numbers...` - Zero or more numeric expressions to add

**Returns**: The sum of all numbers, or 0 if no arguments

**Examples**:
```lisp
(+)           ; Returns 0 (identity)
(+ 5)         ; Returns 5
(+ 1 2 3)     ; Returns 6
(+ -1 1)      ; Returns 0
(+ 1 2 3 4 5) ; Returns 15
```

**Notes**:
- Addition is commutative and associative
- Supports negative numbers
- Overflow checking prevents integer overflow errors

**Source**: `src/builtins/operators.rs:10`

---

## -

**Syntax**: `(- number)` or `(- x y z...)`

Subtraction or negation.

**Parameters**:
- Single argument: `number` - Number to negate
- Multiple arguments: `x` - Initial value, `y z...` - Numbers to subtract from x

**Returns**:
- Single argument: Negative of the number
- Multiple arguments: x minus the sum of all following numbers

**Examples**:
```lisp
(- 5)         ; Returns -5 (negation)
(- 10 3)      ; Returns 7 (10 - 3)
(- 10 3 2)    ; Returns 5 (10 - 3 - 2)
(- 0 5)       ; Returns -5
```

**Notes**:
- With one argument, returns the negation
- With multiple arguments, subtracts from left to right
- Overflow checking prevents integer overflow errors

**Source**: `src/builtins/operators.rs:24`

---

## *

**Syntax**: `(* numbers...)`

Multiplication of zero or more numbers.

**Parameters**:
- `numbers...` - Zero or more numeric expressions to multiply

**Returns**: The product of all numbers, or 1 if no arguments

**Examples**:
```lisp
(*)           ; Returns 1 (identity)
(* 5)         ; Returns 5
(* 2 3)       ; Returns 6
(* 2 3 4)     ; Returns 24
(* -1 5)      ; Returns -5
```

**Notes**:
- Multiplication is commutative and associative
- Returns 1 for empty argument list (multiplicative identity)
- Overflow checking prevents integer overflow errors

**Source**: `src/builtins/operators.rs:38`

---

## /

**Syntax**: `(/ number)` or `(/ x y z...)`

Division or reciprocal.

**Parameters**:
- Single argument: `number` - Number to take reciprocal of
- Multiple arguments: `x` - Initial value, `y z...` - Numbers to divide x by

**Returns**:
- Single argument: 1 divided by the number
- Multiple arguments: x divided by the product of all following numbers

**Examples**:
```lisp
(/ 4)         ; Returns 0 (1/4 truncated to integer)
(/ 8 2)       ; Returns 4
(/ 24 2 3)    ; Returns 4 (24 / 2 / 3)
(/ 7 2)       ; Returns 3 (integer division)
```

**Notes**:
- Integer division truncates toward zero
- Division by zero raises an error
- With one argument, calculates 1/number

**Source**: `src/builtins/operators.rs:52`

---

## rem

**Syntax**: `(rem dividend divisor)`

Calculate the remainder of integer division.

**Parameters**:
- `dividend` - The number to be divided
- `divisor` - The number to divide by

**Returns**: The remainder of dividend ÷ divisor

**Examples**:
```lisp
(rem 10 3)    ; Returns 1 (10 = 3*3 + 1)
(rem 15 4)    ; Returns 3 (15 = 4*3 + 3)
(rem 8 2)     ; Returns 0 (8 = 2*4 + 0)
(rem -7 3)    ; Returns -1 (-7 = 3*(-3) + 1)
```

**Notes**:
- Follows Rust's remainder semantics
- Sign of result matches the dividend
- Division by zero raises an error
- Only accepts exactly two arguments

**Source**: `src/builtins/operators.rs:72`

---

## inc

**Syntax**: `(inc number)`

Increment a number by 1.

**Parameters**:
- `number` - The number to increment

**Returns**: The number plus 1

**Examples**:
```lisp
(inc 5)       ; Returns 6
(inc 0)       ; Returns 1
(inc -1)      ; Returns 0
```

**Notes**:
- Convenience function equivalent to `(+ number 1)`
- Accepts exactly one argument
- More readable than addition for simple increments

**Source**: `src/builtins/operators.rs:82`

---

## dec

**Syntax**: `(dec number)`

Decrement a number by 1.

**Parameters**:
- `number` - The number to decrement

**Returns**: The number minus 1

**Examples**:
```lisp
(dec 5)       ; Returns 4
(dec 0)       ; Returns -1
(dec 1)       ; Returns 0
```

**Notes**:
- Convenience function equivalent to `(- number 1)`
- Accepts exactly one argument
- More readable than subtraction for simple decrements

**Source**: `src/builtins/operators.rs:87`

---

## max

**Syntax**: `(max numbers...)`

Find the maximum value among the given numbers.

**Parameters**:
- `numbers...` - One or more numeric expressions

**Returns**: The largest number among the arguments

**Examples**:
```lisp
(max 5)           ; Returns 5
(max 1 5 3)       ; Returns 5
(max -1 -5 -2)    ; Returns -1
(max 10 20 15 25) ; Returns 25
```

**Notes**:
- Requires at least one argument
- Works with negative numbers
- Uses standard numeric comparison

**Source**: `src/builtins/operators.rs:92`

---

## min

**Syntax**: `(min numbers...)`

Find the minimum value among the given numbers.

**Parameters**:
- `numbers...` - One or more numeric expressions

**Returns**: The smallest number among the arguments

**Examples**:
```lisp
(min 5)           ; Returns 5
(min 1 5 3)       ; Returns 1
(min -1 -5 -2)    ; Returns -5
(min 10 20 15 25) ; Returns 10
```

**Notes**:
- Requires at least one argument
- Works with negative numbers
- Uses standard numeric comparison

**Source**: `src/builtins/operators.rs:104`