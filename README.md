# Overview

A [shunting-yard](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) implementation for transforming infix expressions to postfix.
This crate _only_ implements the algorithm, and does not evaluate expressions.

It is generic over the token type, so you can convert streams of whatever kinds of token you need.

It is also generic over the backing storage, so it is appropriate for use in no-alloc environments.

## Features

- Infix operators (left-associative)
  - Precedence between infix operators
- Unary operators
- Functions with 1 or more arguments

## Use case: Embedded Scripting

An embedded system may want to evaluate mathematical expressions provided at run-time by a user.
For the user's convenience, you want to let them write infix expressions.
However for computational convenience, you want to convert those to postfix to be run.

Once you have converted the input into tokens, `maschen` can be used to do this conversion.
Then you will have an easily evaluated postfix token stream.

Since `maschen` leaves the evaluation of the expression to the caller and doesn't care what the tokens actually are, they can be complex values such as handles to variables in your scripting language.

# How to use

Implement `maschen::Token` for your Token type. Then instantiate an instance of `maschen::ShuntingYard`, and `process()` each token in turn. Finally call `finish()` to retrieve the output.

## No-std

If you disable the `std` feature, the `ShuntingYard::new` method will be unavailable, and you will have to implement `maschen::Stack` on some type to use it as the backing storage for the yard.
In an embedded environment, this will presumably be a static or stack-allocated array.

# Maschen?

https://en.wikipedia.org/wiki/Maschen_Marshalling_Yard
