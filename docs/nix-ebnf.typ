#import "@preview/nutthead-ebnf:0.3.1": *

#context[
  #ebnf[
    #[
      #syntax-rule(
        meta-id: [Expression],
        rule-comment: "The start of a nix expression",
        definition-list: ([
          
          #grouped-sequence(
            single-definition[ExpressionNoAssignment],
            single-definition[Assignment],
          )
        ],),
      )

      #syntax-rule(
        meta-id: [ExpressionNoAssignment],
        definition-list: ([
          
          #grouped-sequence(
            single-definition[Attrset],
            single-definition[Function],
            single-definition[Arithmetic],
          )
        ],)
      )

      #syntax-rule(
        meta-id: [Attrset],
        rule-comment: "Expression of an attrset",
        definition-list: ([
          #optional-sequence(
            single-definition[let-in]
          )
          #terminal[{]
          #repeated-sequence(single-definition[Assignment])
          #terminal[}]
        ],)
      )

      #syntax-rule(
        meta-id: [let-in],
        rule-comment: "Let in",
        rule-example: code-example(```nix let a = 5; in ...```),
        definition-list: ([
          #terminal[let]
          #repeated-sequence(
            single-definition[Assignment]
          )
          #terminal[in]
        ],)
      )

      #syntax-rule(
        meta-id: [Assignment],
        rule-comment: "An assignment",
        rule-example: code-example(```nix a = 5;```),
        definition-list: ([
          #optional-sequence(
            single-definition[let-in]
          )
          #special-sequence[someIdentifier]
          #terminal[=]
          #single-definition[ExpressionNoAssignment]
          #terminal[;]
        ],)
      )

      #syntax-rule(
        meta-id: [Primary],
        rule-comment: "A string, integer or float",
        definition-list: ([
          #grouped-sequence(
            special-sequence[string],
            special-sequence[integer],
            special-sequence[float],
            terminal[true],
            terminal[false],
          )
        ],)
      )

      #syntax-rule(
        meta-id: [Function],
        rule-comment: "A function",
        rule-example: [#code-example(```nix input: output```),],
        definition-list: ([
          #optional-sequence[let-in]
          #special-sequence[InputIdentifier]
          #terminal[:]
          #optional-sequence[let-in]
          #single-definition[ExpressionNoAssignment]
          #terminal[;]
        ],)
      )

      #syntax-rule(
        meta-id: [Arithmetic],
        definition-list: ([
          #single-definition[ArithmeticOrPrimary]
          #repeated-sequence[
            #grouped-sequence(
              terminal[-],
              terminal[+],
              terminal[\*],
              terminal[/],
            )
            #single-definition[ArithmeticOrPrimary]
          ]
        ],)
      )

      #syntax-rule(
        meta-id: [ArithmeticOrPrimary],
        definition-list: ([
          #grouped-sequence(
            single-definition[Primary],
            grouped-sequence[
              #terminal[(]
              #single-definition[Arithmetic]
              #terminal[)]
            ]
          )
        ],)
      )
    ]
  ]
]
