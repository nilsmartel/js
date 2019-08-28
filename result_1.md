```js
let sayHello = () => {
    console.log("Hello, World")
}
```

```rust
Ok(
    (
        "",
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
        },
    ),
)
```
