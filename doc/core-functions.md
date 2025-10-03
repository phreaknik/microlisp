# Core Functions

Core functions provide fundamental language constructs including variable definition, control flow, and iteration.

## def

**Syntax**: `(def symbol value)`

Define a global variable.

**Parameters**:
- `symbol` - The name of the variable (must be a symbol)
- `value` - The value to assign to the variable

**Returns**: `nil`

**Examples**:
```lisp
(def x 42)           ; Define x with value 42
(def name "hello")   ; Define name with string value
(def result (+ 1 2)) ; Define result with computed value 3
```

**Source**: `src/builtins/core.rs:13`

---

## let

**Syntax**: `(let [bindings] expressions...)`

Create local variable bindings that are only visible within the let expression.

**Parameters**:
- `bindings` - Vector of alternating symbols and values `[var1 val1 var2 val2 ...]`
- `expressions...` - Zero or more expressions to evaluate with the bindings in scope

**Returns**: The result of the last expression, or `nil` if no expressions are provided

**Examples**:
```lisp
(let [a 10 b 20] (+ a b))           ; Returns 30
(let [x 5] (* x x) (+ x 1))         ; Returns 6 (last expression)
(let [temp (+ 1 2)] (* temp temp))  ; Returns 9
```

**Notes**:
- Bindings must come in pairs (symbol-value pairs)
- Each binding can reference previously defined bindings within the same let
- Variables are scoped only within the let expression

**Source**: `src/builtins/core.rs:25`

---

## if

**Syntax**: `(if test then-expr else-expr)`

Conditional expression that evaluates one of two branches based on a test condition.

**Parameters**:
- `test` - Expression to evaluate for truthiness
- `then-expr` - Expression to evaluate if test is true
- `else-expr` - Expression to evaluate if test is false

**Returns**: The result of either `then-expr` or `else-expr`

**Examples**:
```lisp
(if true 1 2)              ; Returns 1
(if false 1 2)             ; Returns 2
(if (> 5 3) "yes" "no")    ; Returns "yes"
(if nil "true" "false")    ; Returns "false"
```

**Truthiness**:
- Only `nil` and `false` are considered false
- All other values (including `0`) are considered true

**Source**: `src/builtins/core.rs:73`

---

## do

**Syntax**: `(do expressions...)`

Execute a sequence of expressions in order, returning the value of the last expression.

**Parameters**:
- `expressions...` - Zero or more expressions to evaluate sequentially

**Returns**: The result of the last expression, or `nil` if no expressions are provided

**Examples**:
```lisp
(do)                           ; Returns nil
(do 1 2 3)                     ; Returns 3
(do (def x 5) (+ x 10))        ; Returns 15, x is defined globally
(do (print "hello") (+ 1 2))   ; Prints hello, returns 3
```

**Use Cases**:
- Execute side effects in sequence
- Group multiple expressions where only one is allowed
- Initialize multiple variables

**Source**: `src/builtins/core.rs:85`

---

## while

**Syntax**: `(while test-expr body-expressions...)`

Loop while a condition is true, executing body expressions on each iteration.

**Parameters**:
- `test-expr` - Expression evaluated before each iteration for truthiness
- `body-expressions...` - Zero or more expressions executed on each iteration

**Returns**: `nil`

**Examples**:
```lisp
(def counter 0)
(while (< counter 5)
  (def counter (inc counter)))  ; Loop 5 times

(def sum 0)
(def i 0)
(while (< i 10)
  (def sum (+ sum i))
  (def i (inc i)))              ; Sum numbers 0-9
```

**Notes**:
- Test expression is evaluated before each iteration
- Loop continues while test expression is truthy
- Use `def` within the loop to modify variables for loop termination

**Source**: `src/builtins/core.rs:97`

---

## doseq

**Syntax**: `(doseq [bindings] expressions...)`

Iterate over sequences, binding variables to values from vectors.

**Parameters**:
- `bindings` - Vector of alternating symbols and vectors `[var1 [vals...] var2 [vals...] ...]`
- `expressions...` - Zero or more expressions executed on each iteration

**Returns**: `nil`

**Examples**:
```lisp
(doseq [x [1 2 3]] (print x))              ; Prints 1, 2, 3

(doseq [a [1 2] b [10 20]]
  (print (+ a b)))                         ; Prints 11, 22

(def sum 0)
(doseq [x [1 2 3 4 5]]
  (def sum (+ sum x)))                     ; Sum becomes 15
```

**Requirements**:
- All vectors in bindings must have the same length
- Binding must alternate between symbols and vectors
- Vectors are iterated in parallel (corresponding elements are bound together)

**Source**: `src/builtins/core.rs:110`

---

## dotimes

**Syntax**: `(dotimes [var count] body-expression)`

Loop with a counter variable from 0 to count-1.

**Parameters**:
- `var` - Symbol for the loop counter variable
- `count` - Number of iterations (integer)
- `body-expression` - Expression executed on each iteration

**Returns**: `nil`

**Examples**:
```lisp
(dotimes [i 5] (print i))          ; Prints 0, 1, 2, 3, 4

(def sum 0)
(dotimes [i 10]
  (def sum (+ sum i)))             ; Sum becomes 45 (0+1+...+9)

(dotimes [n 3]
  (print (* n n)))                 ; Prints 0, 1, 4
```

**Notes**:
- Counter starts at 0 and increments by 1 each iteration
- Counter variable is only available within the loop body
- Loop executes exactly `count` times

**Source**: `src/builtins/core.rs:181`

---

## vector

**Syntax**: `(vector elements...)`

Create a vector containing the given elements.

**Parameters**:
- `elements...` - Zero or more expressions that become vector elements

**Returns**: A vector containing the evaluated elements

**Examples**:
```lisp
(vector)              ; Returns []
(vector 1 2 3)        ; Returns [1 2 3]
(vector (+ 1 2) 4)    ; Returns [3 4]
(def x 5)
(vector x (* x 2))    ; Returns [5 10]
```

**Use Cases**:
- Create collections of data
- Pass multiple values to functions expecting vectors
- Build data structures

**Source**: `src/builtins/core.rs:199`

---

## nth

**Syntax**: `(nth vector index)` or `(nth vector index default)`

Get an element from a vector by index.

**Parameters**:
- `vector` - The vector to index into
- `index` - Zero-based index (integer)
- `default` - (Optional) Value to return if index is out of bounds

**Returns**: The element at the given index, or the default value, or `nil`

**Examples**:
```lisp
(nth [1 2 3] 0)        ; Returns 1
(nth [1 2 3] 1)        ; Returns 2
(nth [1 2 3] 5)        ; Returns nil (out of bounds)
(nth [1 2 3] 5 "none") ; Returns "none" (default)
```

**Notes**:
- Indices are zero-based
- Negative indices are not supported
- Out-of-bounds access returns default value or `nil`

**Source**: `src/builtins/core.rs:209`

---

## peek

**Syntax**: `(peek vector)`

Get the last element of a vector without removing it.

**Parameters**:
- `vector` - The vector to peek at

**Returns**: The last element of the vector

**Examples**:
```lisp
(peek [1 2 3])      ; Returns 3
(peek [42])         ; Returns 42
(peek [])           ; Error: Empty vector
```

**Notes**:
- Vector must not be empty
- Does not modify the original vector
- Equivalent to `(nth vector (- (count vector) 1))`

**Source**: `src/builtins/core.rs:239`

---

## pop

**Syntax**: `(pop vector)`

Remove the last element from a vector, returning the modified vector.

**Parameters**:
- `vector` - The vector to remove the last element from

**Returns**: A new vector with the last element removed

**Examples**:
```lisp
(pop [1 2 3])       ; Returns [1 2]
(pop [42])          ; Returns []
(pop [])            ; Error: Empty vector
```

**Notes**:
- Vector must not be empty
- Returns a new vector; does not modify the original
- Use with `def` to update a variable: `(def v (pop v))`

**Source**: `src/builtins/core.rs:253`