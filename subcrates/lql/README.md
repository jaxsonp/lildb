
# LQL - LilDB Query Language

## Formal grammar

LL(1) I think

```txt
<query> ::= "db" <function> ";"

<function> ::=
	"." <function-name> "(" <args> ")" <function> |
	null

<function-name> ::=
	"table" |
	"read" |
	...

<function_args> ::=
	<value> <more-function-args> |
	null
	
<more-function-args> ::=
	"," <value> <more-function-args> |
	null

<value> ::=
	<query> |
	<keyword> |
	<string-literal> |
	<num-literal> |
```