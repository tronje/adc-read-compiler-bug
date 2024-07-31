# Possible compiler bug

Basic example of a problem I have with "nightly-2024-03-22". Values read from
a pin and digitized with an ADC seem invalid. When using an older toolchain,
"nightly-2023-12-28", values in the high hundreds are normal. When using the
newer toolchain, the values are mostly `0`, sometimes other, small values like
`1`, or `2`.

So the first behavior is correct, and the second behavior is not, suggesting
a compiler regression.

I found this out with a real-world example where the pin in question reads a
battery voltage. With the newer compiler toolchain, my software always assumed
the battery to be empty, because its voltage was incredibly low.

Unfortunately, for me to reproduce this issue, a MCP23017 GPIO expander needs
to be set up, and two of its outputs pulled. So I'm not sure how easy this may
be for others to reproduce. I'm assuming any analog read on any pin should
cause the same issues.

## Digital Reads

Rather than performing analog reads and digitizing values using the ADC, the pin
can of course be read digitally (`is_high`). This causes the same behavior;
with the new toolchain, `is_high` always returns `false`, but with the old toolchain,
always `true` (which should be correct).
