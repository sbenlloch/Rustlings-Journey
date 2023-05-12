# IEEE 754 Standard

IEEE 754 is a standard for floating-point computation established by the
Institute of Electrical and Electronics Engineers (IEEE). It's used to
represent real numbers in computing devices.

A floating-point number in IEEE 754 format consists of:

- **Sign bit**: 1 bit - 0 for positive numbers, 1 for negative.
- **Exponent**: 8 bits - used to scale the number.
- **Fraction/Mantissa**: 23 bits - represents the precision bits of the number.

```text
+-----+----------------+----------------------------------+
| Sign |    Exponent   |          Fraction/Mantissa       |
+-----+----------------+----------------------------------+
|  1   |     8 bits    |             23 bits              |
+-----+----------------+----------------------------------+
```

This format provides a wide range of values and includes representations for infinity,
negative zero, and "Not a Number" (NaN) for undefined operations.

