#import "@preview/nutthead-ebnf:0.3.1": *
#set page(height: auto)

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
        meta-id: [AssignmentPreamble],
        rule-comment: "A let .. in or with",
        definition-list: ([
            #repeated-sequence(qualifier: "some",
              grouped-sequence(
                single-definition[let-in],
                single-definition[with]
              )
            ),
        ],)
      )

      #syntax-rule(
        meta-id: [let-in],
        rule-comment: "Let in",
        rule-example: code-example(```nix let a = 5; in ...```),
        definition-list: ([
          #terminal[let]
          #repeated-sequence(
            single-definition[AssignmentNoPreamble]
          )
          #terminal[in]
        ],)
      )

      #syntax-rule(
        meta-id: [with],
        definition-list: ([
          #terminal[with]
            
          #repeated-sequence(qualifier: "some")[ExpressionNoAssignment]
          #terminal[;]
        ],)
      )

      #syntax-rule(
        meta-id: [Inherit],
        definition-list: ([
          #terminal[inherit]
          #grouped-sequence(
            repeated-sequence(special-sequence[identifier], qualifier: "some"),
          )
          #terminal[;]
        ],)
      )

      #syntax-rule(
        meta-id: [Attrset],
        rule-comment: "Expression of an attrset",
        definition-list: ([
          #optional-sequence(
            single-definition[AssignmentPreamble]
          )
          #terminal[{]
          #repeated-sequence(qualifier: "some",
            grouped-sequence(
              single-definition[Assignment],
              single-definition[Inherit]
              ),
            )
          #terminal[}]
        ],)
      )

      #syntax-rule(
        meta-id: [Assignment],
        rule-comment: "An assignment",
        rule-example: code-example(```nix let x = 5; in a = x;```),
        definition-list: ([
          #optional-sequence(
            single-definition[AssignmentPreamble]
          )
          #single-definition[AssignmentNoPreamble]
        ],)
      )

      #syntax-rule(
        meta-id: [AssignmentNoPreamble],
        rule-comment: "An assignment without the preamble",
        rule-example: code-example(```nix a = 5;```),
        definition-list: ([
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
            single-definition[List],
            grouped-sequence[
              #terminal[(]
              #single-definition[ExpressionNoAssignment]
              #terminal[)]
            ]
          )
        ],)
      )

      #syntax-rule(
        meta-id: [List],
        definition-list: ([
          #terminal("[")
          #repeated-sequence[ExpressionNoAssignment]
          #terminal("]")
        ],)
      )

      #syntax-rule(
        meta-id: [Function],
        rule-comment: "A function",
        rule-example: [#code-example(```nix input: output```),],
        definition-list: ([
          #optional-sequence[AssignmentPreamble]
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
            #single-definition[ArithmeticMul]
            #repeated-sequence[
              #grouped-sequence(
                terminal[-],
                terminal[+],
              )
              #single-definition[ArithmeticMul]
            ]
        ],)
      )

      #syntax-rule(
        meta-id: [ArithmeticMul],
        definition-list: ([
          #single-definition[PrimaryOrIdentifier]
          #repeated-sequence[
            #grouped-sequence(
              terminal[\*],
              terminal[/]
            )
            #single-definition[PrimaryOrIdentifier]
          ]
          
        ],)
      )

      #syntax-rule(
        meta-id: [PrimaryOrIdentifier],
        definition-list: ([
          #grouped-sequence(
            single-definition[Primary],
            single-definition[Identifier],
          )
        ],)
      )
    ]
  ]
]
