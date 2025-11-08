
# LQL - LilDB Query Language

```
db.table("Users").ensure_exists();

Users.ensure_exists();

<Users>.ensure_exists();

(Users).ensure_exists();

[Users].ensure_exists();
```

## Formal grammar

LL(1) I think

```txt
<query> ::= <table> <function-call> ";"

<function-call> ::=
	"." <function-name> "(" <args> ")" <function-call> |
	null

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