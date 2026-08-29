#import "@preview/nutthead-ebnf:0.3.1": *
#set page(height: auto)

#let space = h(1em)

#context[
  #ebnf[
    #[
      #syntax-rule(
        meta-id: [Expression <expression>],
        rule-comment: "The start of a nix expression",
        definition-list: ([
          
          #optional-sequence[#link(<preamble>)[Preamble]]
          #grouped-sequence(
            single-definition[#link(<expression_no_assignment>)[ExpressionNoAssignment]],
            single-definition[#link(<assignment>)[Assignment]],
          )
        ],),
      )

      #space
      
      #syntax-rule(
        meta-id: [ExpressionNoAssignment <expression_no_assignment>],
        definition-list: ([
          
          #grouped-sequence(
            single-definition[#link(<attrset>)[Attrset]],
            single-definition[#link(<function>)[Function]],
            single-definition[#link(<arithmetic>)[Arithmetic]],
          )
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Preamble <preamble>],
        rule-comment: "A let .. in or with",
        definition-list: ([
            #repeated-sequence(qualifier: "some",
              grouped-sequence(
                single-definition[#link(<let_in>)[let-in]],
                single-definition[#link(<with>)[with]]
              )
            ),
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [let-in <let_in>],
        rule-comment: "Let in",
        rule-example: code-example(```nix let a = 5; in ...```),
        definition-list: ([
          #terminal[let]
          #repeated-sequence(qualifier: "some",
            grouped-sequence(
              
              single-definition[#link(<assignment>)[Assignment]],
              single-definition[#link(<inherit>)[Inherit]]
            )
          )
          #terminal[in]
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [with <with>],
        definition-list: ([
          #terminal[with]
            
          #grouped-sequence(
            
            single-definition[#link(<attrset>)[Attrset]],
            special-sequence[Identifier of Attrset]
          )
          #terminal[;]
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Inherit <inherit>],
        definition-list: ([
          #terminal[inherit]
          #optional-sequence(
            special-sequence[Identifier of Attrset],
            single-definition[#link(<attrset>)[Attrset]]
          )
          #repeated-sequence(qualifier: "some")[
            #special-sequence[Identifier]
          ]
          #terminal[;]
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Attrset <attrset>],
        rule-comment: "Expression of an attrset",
        definition-list: ([
          #terminal[{]
          #repeated-sequence(qualifier: "some",
            grouped-sequence(
              single-definition[#link(<assignment>)[Assignment]],
              single-definition[#link(<inherit>)[Inherit]]
              ),
            )
          #terminal[}]
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Assignment <assignment>],
        rule-comment: "An assignment without the preamble",
        rule-example: code-example(```nix a = 5;```),
        definition-list: ([
          #special-sequence[someIdentifier]
          #terminal[=]
          #single-definition[#link(<expression_no_assignment>)[ExpressionNoAssignment]]
          #terminal[;]
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Primary <primary>],
        rule-comment: "A string, integer or float",
        definition-list: ([
          #grouped-sequence(
            special-sequence[string],
            special-sequence[integer],
            special-sequence[float],
            terminal[true],
            terminal[false],
            single-definition[#link(<list>)[List]],
            grouped-sequence[
              #terminal[(]
              #single-definition[#link(<expression_no_assignment>)[ExpressionNoAssignment]]
              #terminal[)]
            ]
          )
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [List <list>],
        definition-list: ([
          #terminal("[")
          #repeated-sequence[#link(<expression_no_assignment>)[ExpressionNoAssignment]]
          #terminal("]")
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Function <function>],
        rule-comment: "A function",
        rule-example: [#code-example(```nix input: output```),],
        definition-list: ([
          #special-sequence[InputIdentifier]
          #terminal[:]
          #optional-sequence[#link(<preamble>)[Preamble]]
          #single-definition[#link(<expression_no_assignment>)[ExpressionNoAssignment]]
          #terminal[;]
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Arithmetic <arithmetic>],
        definition-list: ([
            #single-definition[#link(<arithmetic_mul>)[ArithmeticMul]]
            #repeated-sequence[
              #grouped-sequence(
                terminal[-],
                terminal[+],
              )
              #single-definition[#link(<arithmetic_mul>)[ArithmeticMul]]
            ]
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [ArithmeticMul <arithmetic_mul>],
        definition-list: ([
          #single-definition[#link(<unary>)[Unary]]
          #repeated-sequence[
            #grouped-sequence(
              terminal[\*],
              terminal[/]
            )
            #single-definition[#link(<unary>)[Unary]]
          ]
          
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [Unary <unary>],
        definition-list: ([
          #grouped-sequence(
            
            single-definition[            
                #grouped-sequence(
                  terminal[!],
                  terminal[-],
                )
                #single-definition[#link(<unary>)[Unary]]
              ],
            single-definition[#link(<primary_or_identifier>)[PrimaryOrIdentifier]]
            )
        ],)
      )

      #space
      
      #syntax-rule(
        meta-id: [PrimaryOrIdentifier <primary_or_identifier>],
        definition-list: ([
          #grouped-sequence(
            single-definition[#link(<primary>)[Primary]],
            single-definition[Identifier],
          )
        ],)
      )
    ]
  ]
]
