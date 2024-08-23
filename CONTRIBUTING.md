# Contributing to ItoLib

First off, I want to thank you for your contributions. I'm looking for all the help I can get.

In this document, I'll outline some things about contributing to ItoLib.

## Code Style

In this section, I'll outline some conventions that I would like to enforce for the codebase. These are aimed at improving clarity and maintaining some consistency across the codebase.

### Imports

The import block should follow the following consistent structure. This makes it easy to see the different groups of imports.

```rust
// First, standard library imports.
use std::fmt::Display;
use std::vec::Vec;

// Next, external crates.
use num::Float;
use hifitime::Epoch;

// Then, internal crates.
use itolib_types::float::TypedFloat;
use itolib_money::Money;

// Finally, the current crate. (This example is assuming
// you are working in itolib-instruments.)
use crate::exercises::AmericanExercise;
```
