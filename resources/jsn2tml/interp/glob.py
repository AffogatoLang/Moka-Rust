def mokadef(Token, Pattern):
    @Token("T_IDENT")
    def T_IDENT(ident):
        return "$" + ident;

    @Token("T_VAR")
    def T_VAR(var):
        return "VAR";

    @Pattern(["T_VAR", "T_IDENT"])
    def p_T_VART_IDENT(var, ident, context):
        return var + " " + ident;
