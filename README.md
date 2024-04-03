# Interpreter Tutorial

The example files for a simple interpreter in rust.

---

### Example CLI

```
@> some example
some result
---
@> something else
other result
---
@> no result ?
---
@> some kind
#> of multiline
#> thing
some result
```

### Example Math Syntax

```
EXPR ::= TERM op:+ EXPR 
       | TERM op:- EXPR 
       | TERM
       ;

TERM ::= FACTOR op:* TERM 
       | FACTOR op:/ TERM 
       | FACTOR
       ;

FACTOR ::= op:( EXPR op:) 
         | NUM 
         | VAR
         ;

NUM ::= int:
      | float:
      ;

VAR ::= ident:

op:     + - * / ( )
float:  /[0-9]+\.[0-9]+/
int:    /[0-9]+/
ident:  /[a-zA-Z_]+/
```

### Raw code for the above

```
EXPR = [
    Expression([ Expr(TERM), Token(op:+), Expr(EXPR) ]),
    Expression([ Expr(TERM), Token(op:-), Expr(EXPR) ]),
    Expr(TERM),
]

TERM = [
    Expression([ Expr(FACTOR), Token(op:*), Expr(TERM) ]),
    Expression([ Expr(FACTOR), Token(op:/), Expr(TERM) ]),
    Expr(FACTOR),
]

FACTOR = [
    Expression([ Token( op:( ), Expr(EXPR), Token( op:) )]),
    Expr(NUM),
    Expr(VAR),
]

NUM = [
    Token(int:),
    Token(float:),
]

VAR = [
    Token(ident:)
]
```

### Artamis Pseudo Code

The UniOp `!` is the required operator which makes the interpreter throw a compile error if the expression is not present. The UniOp `?` is the maybe operator which will continue matching even if the expression is missing. The brackets `(` and `)` denote sub expressions

```
expr ::= math:expr

// prefered
math:expr ::= math:term op:+ math:expr! { ADD $1 $3 }
            | math:term op:- math:expr! { SUB $1 $3 }
            | math:term                 { EVAL $ }

// allowed but not prefered
math:term ::= math:factor ( ( op:* | op:/ ) math:term! )?

// this expands to
math:term ::= math:factor math:term:0?
math:term:0 ::= math:term:1 math:term!
math:term:1 ::= op:* | op:/

math:factor ::= op:( math:expr ( op:) )!    {( EVAL $2 )}
              | math:num                    { EVAL }
              | math:var                    { EVAL }

math:num ::= int:       { INTEGER } 
           | float:     { FLOAT }
math:var ::= ident:     { GET_IDENT }
```

```
INTEGER -> Value<i32> => {
    $.as_value<i32>
}

FLOAT -> Value<f32> => {
    $.as_value<f32>
}

GET_IDENT -> Value => {
    $$.get_value( $.as_ident )
}

ADD $a $b -> Value => {
    $a.as_value + $b.as_value
}

SUB $a $b -> Value => {
    $a.as_value - $b.as_value
}
```

## Info
- Channel: [Channel Name](#)
- Other Thing: [Thing Name](#)