# NEPL 簡易版 試作実装 計画

https://zenn.dev/bem130/articles/cd15285958d45e  
https://zenn.dev/bem130/articles/b26fd70bcb9fe4  
block,複数行の記法以外、この2記事の内容とほぼ同じものを実装する

## 関数
```ebnf
<expr> := "\" <arg_name> <func_body>
<func_body> := <expr>
```

## let
```ebnf
<expr> := "let" <arg_name> <>
```


複数行の記法は変更する

## 複式
一般的なプログラミング言語において `"{" ( <stmt> ";" )* "}"` に相当するもの  
`";" <expr_1> <expr_2>` として、式の値や型は`<expr_2>`  
```ebnf
<expr> := <block_expr>
<block_expr> := ";" <expr_1> <expr_2>
```

## enum,struct
```ebnf
<expr> := "struct" <struct_name> <struct_decl_body>
<struct_decl_body> := "," <struct_decl_field> ( <struct_decl_body> | <struct_decl_field> )
```
```ebnf
<expr> := "enum" <enum_name> <enum_decl_body>
<enum_decl_body> := "," <enum_decl_field> ( <enum_decl_body> | <enum_decl_field> )
```