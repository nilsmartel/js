```js
onload = () => {
    setInterval(onUpdate, 100)
}

let player = {
    health: 100,
    pos: {
        x: 12,
        y: 7
    },
    name: "Markus"
}


function onUpdate() {
   actions.apply(player)
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
                        "player",
                    ),
                    assign: Some(
                        Value(
                            Map(
                                {
                                    Identifier(
                                        "health",
                                    ): Value(
                                        Number(
                                            100.0,
                                        ),
                                    ),
                                    Identifier(
                                        "pos",
                                    ): Value(
                                        Map(
                                            {
                                                Identifier(
                                                    "x",
                                                ): Value(
                                                    Number(
                                                        12.0,
                                                    ),
                                                ),
                                                Identifier(
                                                    "y",
                                                ): Value(
                                                    Number(
                                                        7.0,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                    Identifier(
                                        "name",
                                    ): Value(
                                        String(
                                            StringTemplate {
                                                start: "Markus",
                                                end: [],
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                    ),
                },
            ],
            functions: [
                Function {
                    identifier: Identifier(
                        "onUpdate",
                    ),
                    arguments: [],
                    body: FunctionBody {
                        scope: [],
                        functions: [],
                        instructions: [
                            Expression(
                                Identifier {
                                    path: [
                                        Identifier(
                                            "actions",
                                        ),
                                        Identifier(
                                            "apply",
                                        ),
                                    ],
                                    action: Some(
                                        Call {
                                            arguments: [
                                                Identifier {
                                                    path: [
                                                        Identifier(
                                                            "player",
                                                        ),
                                                    ],
                                                    action: None,
                                                },
                                            ],
                                        },
                                    ),
                                },
                            ),
                        ],
                    },
                },
            ],
            instructions: [
                Expression(
                    Mutate {
                        variable: Identifier(
                            "onload",
                        ),
                        mutation: Assign,
                        assign: Value(
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
                                                        "setInterval",
                                                    ),
                                                ],
                                                action: Some(
                                                    Call {
                                                        arguments: [
                                                            Identifier {
                                                                path: [
                                                                    Identifier(
                                                                        "onUpdate",
                                                                    ),
                                                                ],
                                                                action: None,
                                                            },
                                                            Value(
                                                                Number(
                                                                    100.0,
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
                    },
                ),
            ),
        },
    ),
)
```
