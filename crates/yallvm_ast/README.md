# yallvm AST

This crate contains the AST data structures used for yallvm's tooling.

It also is the place to implement tools such as visitors and AST related utilities,
but not specifically any transforms that the compiler performs.

Important and AST-wide design choices to be aware of:
 - References to AST nodes which aren't specific (e.g. `Expr`, `Stmt`) are almost always
   `Box`ed to avoid storing the entire AST on the stack (impractical!),
   though this is not a huge performance win, because unlike using `Rc`,
   `clone()`ing the AST will still clone children of the node.
 - All node structs have a `span` field which provides the byte positions and line number of
   every AST node. These are used to provide useful error messages to the compiler.
 - All node structs implement `Debug` and `Clone`, and `Span` implements `Default`.
 - All nodes can be postfix-`Box`ed via `.into()`, as is standard behaviour in Rust!
   * Same for nodes that are in an enum (`DeclStmt.into() -> Stmt`), for ease of use
   * You can also convert to a boxed enum in one go (perhaps most usefully!)
   * Note that this will require the compiler to be able to infer the type,
     either by being in a context where an enum type is required (`Node1 { field: Node2 {}.into() }`),
     or by annotation (`let a: Box<Stmt> = DeclStmt { /* ... */ }.into()`).
 - All string literals are `String`s
 - All collections are `Vec`s
 - Types are generally called `type_` in nodes, as `type` is a keyword in Rust. Same for some others.
 - Most nodes that are functions (arrows, methods, functions, operators, constructors) use the
   `FuncCommon` type to store all the common properties together, to reduce duplication.