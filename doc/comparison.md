# Comparison Operations

Comparison operations compare values and return boolean results. All comparisons work with numbers and support chaining for multiple comparisons.

## ==

**Syntax**: `(== values...)`

Test if all values are equal.

**Parameters**:
- `values...` - One or more values to compare for equality

**Returns**: `true` if all values are equal, `false` otherwise

**Examples**:
```lisp
(== 5)          ; Returns true (single value)
(== 5 5)        ; Returns true
(== 5 5 5)      ; Returns true
(== 5 6)        ; Returns false
(== 1 1 2)      ; Returns false
```

**Notes**:
- Single argument always returns `true`
- Compares values using structural equality
- Works with numbers, booleans, and other data types
- All arguments must be equal for the result to be `true`

**Source**: `src/builtins/operators.rs:116`

---

## >

**Syntax**: `(> values...)`

Test if values are in strictly decreasing order.

**Parameters**:
- `values...` - One or more numeric values to compare

**Returns**: `true` if each value is greater than all following values, `false` otherwise

**Examples**:
```lisp
(> 5)           ; Returns true (single value)
(> 5 3)         ; Returns true (5 > 3)
(> 5 3 1)       ; Returns true (5 > 3 > 1)
(> 5 3 3)       ; Returns false (3 is not > 3)
(> 3 5)         ; Returns false (3 is not > 5)
```

**Notes**:
- Single argument always returns `true`
- Requires strict inequality (equal values return `false`)
- Comparisons are chained: `(> a b c)` means `a > b AND b > c`
- All comparisons must be true for the result to be `true`

**Source**: `src/builtins/operators.rs:131`

---

## >=

**Syntax**: `(>= values...)`

Test if values are in non-increasing order (decreasing or equal).

**Parameters**:
- `values...` - One or more numeric values to compare

**Returns**: `true` if each value is greater than or equal to all following values, `false` otherwise

**Examples**:
```lisp
(>= 5)          ; Returns true (single value)
(>= 5 3)        ; Returns true (5 >= 3)
(>= 5 5)        ; Returns true (5 >= 5)
(>= 5 3 3 1)    ; Returns true (5 >= 3 >= 3 >= 1)
(>= 3 5)        ; Returns false (3 is not >= 5)
```

**Notes**:
- Single argument always returns `true`
- Allows equal values (non-strict inequality)
- Comparisons are chained: `(>= a b c)` means `a >= b AND b >= c`
- All comparisons must be true for the result to be `true`

**Source**: `src/builtins/operators.rs:147`

---

## <

**Syntax**: `(< values...)`

Test if values are in strictly increasing order.

**Parameters**:
- `values...` - One or more numeric values to compare

**Returns**: `true` if each value is less than all following values, `false` otherwise

**Examples**:
```lisp
(< 3)           ; Returns true (single value)
(< 1 3)         ; Returns true (1 < 3)
(< 1 3 5)       ; Returns true (1 < 3 < 5)
(< 1 3 3)       ; Returns false (3 is not < 3)
(< 5 3)         ; Returns false (5 is not < 3)
```

**Notes**:
- Single argument always returns `true`
- Requires strict inequality (equal values return `false`)
- Comparisons are chained: `(< a b c)` means `a < b AND b < c`
- All comparisons must be true for the result to be `true`

**Source**: `src/builtins/operators.rs:163`

---

## <=

**Syntax**: `(<= values...)`

Test if values are in non-decreasing order (increasing or equal).

**Parameters**:
- `values...` - One or more numeric values to compare

**Returns**: `true` if each value is less than or equal to all following values, `false` otherwise

**Examples**:
```lisp
(<= 3)          ; Returns true (single value)
(<= 1 3)        ; Returns true (1 <= 3)
(<= 3 3)        ; Returns true (3 <= 3)
(<= 1 3 3 5)    ; Returns true (1 <= 3 <= 3 <= 5)
(<= 5 3)        ; Returns false (5 is not <= 3)
```

**Notes**:
- Single argument always returns `true`
- Allows equal values (non-strict inequality)
- Comparisons are chained: `(<= a b c)` means `a <= b AND b <= c`
- All comparisons must be true for the result to be `true`

**Source**: `src/builtins/operators.rs:179`

## Chaining Examples

Comparison operators can be chained to express complex relationships:

```lisp
; Check if x is between 1 and 10 (inclusive)
(and (<= 1 x) (<= x 10))

; Check if values are in sorted order
(< a b c d)

; Check if values are equal or in ascending order
(<= a b c d)

; Check if x is the maximum of three values
(and (>= x y) (>= x z))
```

## Error Handling

- Comparison operations require compatible types
- Comparing incompatible types may result in type errors
- All comparison functions require at least one argument