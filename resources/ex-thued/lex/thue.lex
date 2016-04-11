###
# @name Production Operator
# @description Describes the relation between an lhs rule and a rhs rule
###
5 : T_PRODUCER : ::=

###
# @name Section Delimiter
# @description Separates the definition section from the input section. This
# differs from the original Thue in that the producer element is not used for
# this purpose
###
8 : T_SPLIT : ::=::

###
# @name String
# @description Matches any arbitrary sequence of characters
# @capture The full sequence of characters
###
10 : T_STRING : ([^\s]+(?:\b\s*[^\s]+)*)
