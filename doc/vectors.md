# Vector Operations

Vectors are ordered collections of values in MicroLisp. They are created with square brackets `[1 2 3]` and provide indexed access to elements.

## vector

**Syntax**: `(vector elements...)`

Create a vector containing the given elements.

**Parameters**:
- `elements...` - Zero or more expressions that become vector elements

**Returns**: A vector containing the evaluated elements

**Examples**:
```lisp
(vector)                    ; Returns []
(vector 1)                  ; Returns [1]
(vector 1 2 3)              ; Returns [1 2 3]
(vector (+ 1 2) (* 3 4))    ; Returns [3 12]
(vector true false nil)     ; Returns [true false nil]
```

**Notes**:
- Elements are evaluated before being added to the vector
- Can contain mixed data types
- Empty vectors are valid

**Source**: `src/builtins/core.rs:199`

---

## nth

**Syntax**: `(nth vector index)` or `(nth vector index default)`

Get an element from a vector by zero-based index.

**Parameters**:
- `vector` - The vector to index into
- `index` - Zero-based integer index
- `default` - (Optional) Value returned if index is out of bounds

**Returns**:
- The element at the specified index
- The default value if index is out of bounds and default is provided
- `nil` if index is out of bounds and no default is provided

**Examples**:
```lisp
(nth [10 20 30] 0)          ; Returns 10
(nth [10 20 30] 1)          ; Returns 20
(nth [10 20 30] 2)          ; Returns 30
(nth [10 20 30] 3)          ; Returns nil (out of bounds)
(nth [10 20 30] 3 "none")   ; Returns "none" (default value)
(nth [10 20 30] -1)         ; Returns nil (negative index)
```

**Index Rules**:
- Indices are zero-based (first element is at index 0)
- Negative indices are not supported
- Out-of-bounds indices return the default value or `nil`

**Source**: `src/builtins/core.rs:209`

---

## peek

**Syntax**: `(peek vector)`

Get the last element of a vector without modifying the vector.

**Parameters**:
- `vector` - The vector to examine

**Returns**: The last element of the vector

**Examples**:
```lisp
(peek [1 2 3])              ; Returns 3
(peek [42])                 ; Returns 42
(peek ["a" "b" "c"])        ; Returns "c"
```

**Error Cases**:
```lisp
(peek [])                   ; Error: Empty vector
```

**Notes**:
- Does not modify the original vector
- Equivalent to `(nth vector (- (count vector) 1))` for non-empty vectors
- More efficient than calculating the last index manually

**Source**: `src/builtins/core.rs:239`

---

## pop

**Syntax**: `(pop vector)`

Remove the last element from a vector, returning a new vector without that element.

**Parameters**:
- `vector` - The vector to remove the last element from

**Returns**: A new vector with the last element removed

**Examples**:
```lisp
(pop [1 2 3])               ; Returns [1 2]
(pop [1 2])                 ; Returns [1]
(pop [42])                  ; Returns []
```

**Error Cases**:
```lisp
(pop [])                    ; Error: Empty vector
```

**Usage Patterns**:
```lisp
; Update a variable with the popped vector
(def my-vec [1 2 3])
(def my-vec (pop my-vec))   ; my-vec is now [1 2]

; Process elements from the end
(def stack [1 2 3 4 5])
(while (> (count stack) 0)
  (def last-elem (peek stack))
  (def stack (pop stack))
  (print last-elem))
```

**Notes**:
- Returns a new vector; does not modify the original
- Use with `def` to update a variable
- Combines well with `peek` for stack-like operations

**Source**: `src/builtins/core.rs:253`

---

## Vector Literals

Vectors can also be created using literal syntax with square brackets:

```lisp
[]                          ; Empty vector
[1]                         ; Single-element vector
[1 2 3]                     ; Multi-element vector
[true false nil]            ; Mixed types
[(+ 1 2) (* 3 4)]          ; Expressions are evaluated
[[1 2] [3 4]]              ; Nested vectors
```

## Common Vector Patterns

### Stack Operations
```lisp
; Push (add to end)
(def stack [1 2 3])
(def stack (vector (nth stack 0) (nth stack 1) (nth stack 2) 4))

; Pop (remove from end)
(def last-item (peek stack))
(def stack (pop stack))
```

### Safe Access
```lisp
; Get element with default
(nth my-vector index "not-found")

; Check if index is valid
(if (and (>= index 0) (< index (count my-vector)))
    (nth my-vector index)
    "invalid-index")
```

### Iteration
```lisp
; Iterate over vector indices
(def my-vec [10 20 30 40])
(dotimes [i (count my-vec)]
  (print (nth my-vec i)))

; Iterate over vector with doseq (if you have parallel indices)
(doseq [i [0 1 2 3] val [10 20 30 40]]
  (print (str "Index " i " has value " val)))
```

### Vector Building
```lisp
; Build vector incrementally
(def result [])
(dotimes [i 5]
  (def result (vector-append result (* i i))))  ; [0 1 4 9 16]
```

**Note**: MicroLisp doesn't have `count` or `vector-append` functions in the current implementation. These are examples of common patterns that might be implemented as additional built-ins.

## Type Checking

Vectors are a distinct data type in MicroLisp:

```lisp
; Vector functions expect vector arguments
(nth 123 0)                 ; Error: Expected vector
(peek "string")             ; Error: Expected vector
(pop true)                  ; Error: Expected vector
```

## Performance Considerations

- `vector` function evaluates all arguments before creating the vector
- `nth` provides O(1) access to elements by index
- `peek` is O(1) to access the last element
- `pop` creates a new vector, which may involve copying elements