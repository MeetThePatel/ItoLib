# ItoLib-Types floats

Below is a diagram representing the relationships between the different floats provided in this module.

An arrow pointing from `A` to `B` means that `A` can be cast to `B` using `Into` (infallible), but a cast from `B`
to `A` requires a `TryInto` (fallible).

```mermaid
flowchart TD
    ff[FiniteFloat] --> f64[f64]
    nnf[NonNegativeFloat] --> f64
    pf[PositiveFloat] --> f64
    nnff[NonNegativeFiniteFloat] --> f64
    pff[PositiveFiniteFloat] --> f64
    pff --> pf
    pff --> nnf
    pff --> nnff
    pff --> ff
    pf --> nnf
    nnff --> nnf
    nnff --> ff
    style f64 fill: #EEE, stroke: #000, stroke-dasharray: 5 5

    subgraph finites [Finite]
        ff
        nnff
        pff
    end
```

One thing to note is that while all types defined in this module can be converted to `f64`, `Into<f64>` is intentionally
not provided. The reason for this is that we want to be very explicit when converting out of our typed float system, and
we have `.value()` for this reason.
