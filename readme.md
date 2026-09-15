## DNeSt

This work-in-progress project is an ongoing effort to build a DNS parser in Rust formally verified using the Kani framework.

### What's done:

The basic architecture of DNS parsing using a zero-copy strategy and `no-std`. Parsing structures from byte buffers and verifying some of their invariants: for instance, we verify that if parsing succeeds, the cursor position advances by the number of bytes parsed.

### Still to do:

Invariants related to name parsing and compression pointers. The major goal of this project: to verify some kind of boundedness or termination statement for DNS parsing. I'm making ongoing efforts to understand the internals of constraint solvers and the Kani framework to help me tackle the technical problems involved.

### AI declaration

I had conversations with AI concerning architectural strategies and features of the Rust language and the Kani framework. All code and documentation was handwritten by me.
