; python 注释匹配规则
(comment) @comment
; 文档开头的多行字符串 (Module Docstring)
(module
  . (expression_statement
      [(string) @comment
       (concatenated_string) @comment]
      (#match? @comment "^(\"\"\"|''')")))

; 函数定义后的多行文档字符串
(function_definition
  body: (block
    . (expression_statement
        [(string) @comment
         (concatenated_string) @comment]
        (#match? @comment "^(\"\"\"|''')"))))

; 类定义后的多行文档字符串
(class_definition
  body: (block
    . (expression_statement
        [(string) @comment
         (concatenated_string) @comment]
        (#match? @comment "^(\"\"\"|''')"))))
