# Math
Additions and Subtractions cant be easily used with - and +, while more advanced stuff requires the math package for now.

```rust
let a = 5;
let b = -5;
let c = a - b;

print(a);
print(b);
print(c);

// Outputs
//
// 5
// -5
// 10
```

## Math Package

You can import the package with
```rust
import "math" as math;

math.div(1,2); // can be used like this
```
or
```rust
import "math" as *; // can be accessed without any prefix

div(1,2); // is not a property now
```

You can do the following with the math package:
```rust
print("mul(2, 3)    = 6 = ", math.mul(2, 3));
print("abs(-5)      = 5 = ", math.abs(-5));
print("pow(2, 3)    = 8 = ", math.pow(2, 3));
print("sqrt(9)      = 3 = ", math.sqrt(9));
print("ceil(1.1)    = 2 = ", math.ceil(1.5));
print("floor(1.9)   = 1 = ", math.floor(1.5));
print("random()     = ", math.random());
print("random_int() = ", math.random_int());
print("cbrt(27)     = 3 = ", math.cbrt(27));
print("acos(1)      = 0 = ", math.acos(1));
print("acosh(1)     = 0 = ", math.acosh(1));
print("asin(0)      = 0 = ", math.asin(0));
print("asinh(0)     = 0 = ", math.asinh(0));
print("atan(1)      = 0.785398... = ", math.atan(1));
print("atanh(0)     = 0 = ", math.atanh(0));
print("cos(0)       = 1 = ", math.cos(0));
print("cosh(0)      = 1 = ", math.cosh(0));
print("exp(1)       = e = ", math.exp(1));
print("exp2(3)      = 8 = ", math.exp2(3));
print("exmp1(1)     = e - 1 = ", math.expm1(1));
print("fract(1.5)   = 0.5 = ", math.fract(1.5));
print("ln(e)        = 1 = ", math.ln(2.718281828459045));
print("ln1p(e - 1)  = 1 = ", math.ln1p(2.718281828459045 - 1));
print("log10(100)   = 2 = ", math.log10(100));
print("log2(8)      = 3 = ", math.log2(8));
print("sin(π/2)     = 1 = ", math.sin(div(PI, 2)));
print("sinh(0)      = 0 = ", math.sinh(0));
print("tan(π/4)     = 1 = ", math.tan(div(PI, 4)));
print("tanh(0)      = 0 = ", math.tanh(0));
print("trunc(1.9)   = 1 = ", math.trunc(1.9));
print("PI           = 3.14... = ", math.PI);
```

## Joining Strings

You can use '+' to join strings, like this:
```rust
let a = "Hello,";

print(a + " World!");
```

This should output "Hello, World!"