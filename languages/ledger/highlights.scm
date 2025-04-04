[
  (block_comment)
  (comment)
  (note)
  (test)
] @comment

[
  (negative_quantity)
  (quantity)
] @number

[
  (date)
  (effective_date)
  (interval)
  (time)
] @string

[
  (check_in)
  (check_out)
  (commodity)
  (option)
  (option_value)
] @string.special

(payee) @property
(account) @property
(filename) @link_uri

(code) @number
(code
    "(" @punctuation.bracket
    ")" @punctuation.bracket
)

"include" @keyword.import

[
  "account"
  "alias"
  "assert"
  "check"
  "comment"
  "commodity"
  "def"
  "default"
  "end"
  "eval"
  "format"
  "nomarket"
  "note"
  "payee"
  "tag"
  "test"
  "A"
  "C"
  "D"
  "N"
  "P"
  "Y"
] @keyword
