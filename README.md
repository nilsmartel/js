# Rust JavaScript Engine

---

## Please note:
The title is lying! You are being deceived! This is not a complete Javascript Interpreter, this is a mere parser! While there are efforts to transform the AST into bytecode and the underlying stack machine is implemented already, development has halted for now.

---

## Parser
All Structures and Parser can be found in `src/parse/*`
the Parser itself isn't completed, nor is it working correctly, but in a wide margin of cases it already works splendid.
Further testing will make it work better

Exampels can be found in `results` files.

A small example of the Parser in action:

```js
let sayHello = () => {
    console.log("Hello, World")
}
```

This small JavaScript code will yield this enormous [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree):


```rust
        FunctionBody {
            scope: [
                Variable {
                    identifier: Identifier(
                        "sayHello",
                    ),
                    assign: Some(
                        Value(
                            Closure {
                                args: [],
                                body: FunctionBody {
                                    scope: [],
                                    functions: [],
                                    instructions: [
                                        Expression(
                                            Identifier {
                                                path: [
                                                    Identifier(
                                                        "console",
                                                    ),
                                                    Identifier(
                                                        "log",
                                                    ),
                                                ],
                                                action: Some(
                                                    Call {
                                                        arguments: [
                                                            Value(
                                                                String(
                                                                    StringTemplate {
                                                                        start: "Hello, World",
                                                                        end: [],
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            },
                                        ),
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ],
            functions: [],
            instructions: [],
        }
```

## Current Task
- Implement Bytecode compilation
- Implement VM

## TODO
- Better Tests
- Write ByteCode parser
- Evalute

