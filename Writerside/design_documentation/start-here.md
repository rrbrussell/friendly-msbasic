# Design Documentation

## Objectives

- Output executables for 64-bit Windows 10 or Windows 11.
- Direct memory access isn't supported.

## Nice to haves

- 8086 16-bit Real Mode support targeting FreeDOS.
- Arm64 port

## Subitem documentation

### Basic Parser loop
```
Start from None
Do we have a peeked character?
if Yes use it instead of the next character as current

if current is alphabetic then start looping for an alpha
numeric string. if we don't find any numbers tag the string
as a alphabetical string otherwise tag it as alphanumeric.

if current is numeric then consume all numbers.
```