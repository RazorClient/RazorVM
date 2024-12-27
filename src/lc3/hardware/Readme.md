## Register

### Methods

- `pub fn new() -> Self`  
  Creates a new `Registers` instance with all registers (`R0`-`R7`, `PC`, `COND`) initialized to `0`.

- `pub fn read(&self, reg: RegisterEnum) -> u16`  
  Reads a `u16` value from the specified register:
  - Panics if the `reg` index is out of bounds.

- `pub fn write(&mut self, reg: RegisterEnum, value: u16)`  
  Writes a `u16` value to the specified register:
  - Panics if the `reg` index is out of bounds.

- `pub fn update_flags(&mut self, reg: RegisterEnum)`  
  Updates the `COND` register (`R9`) based on the value of the specified register (`R0`-`R7`):
  - The `COND` register is updated using the `ConditionFlags` derived from the value of the given register.
  - Panics if the specified register index is not within the range of general-purpose registers (`R0`-`R7`).



## Memory

### Methods

- `pub fn new() -> Self`  
  Creates a new `Memory` instance with all memory cells initialized to `0`.

- `pub fn read(&self, address: usize) -> u16`  
  Reads a `u16` value from the specified memory address:
  - Panics if the `address` is out of bounds.

- `pub fn write(&mut self, address: usize, value: u16)`  
  Writes a `u16` value to the specified memory address:
  - Panics if the `address` is out of bounds.



## ConditionFlags


### Methods

- `fn update_from_value(value: i16) -> ConditionFlags`  
  Creates a `ConditionFlags` instance based on the given signed 16-bit integer value:
  - Returns `ConditionFlags::ZRO` if `value` is `0`.
  - Returns `ConditionFlags::POS` if `value` is positive.
  - Returns `ConditionFlags::NEG` if `value` is negative.

- `fn is_condition_met(&self, condition: ConditionFlags) -> bool`  
  Checks if the specified condition flag is active in the current instance.

- `fn to_debug_string(&self) -> String`  
  Returns a human-readable string representation of all active flags, separated by `|`.

