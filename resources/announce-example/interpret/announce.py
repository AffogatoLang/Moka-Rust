@Pattern("S_VAR_ASSIGN T_EOL")
def S_STATEMENT_VAR_ASSIGN(var, eol):
    """Evaluates an S_STATEMENT node for variable assignment"""
    return var + ";";

@Pattern("S_VAR_DEC T_EOL")
def S_STATEMENT_VAR_DEC(var, eol):
    """Evaluates an S_STATEMENT node for variable decleration"""
    return var + ";";

@Pattern("T_VAR_DEC T_IDENT")
def S_VAR_DEC(dec, ident):
    """Evaluates an S_STATEMENT node"""
    return "var " + ident;

@Pattern("S_VAR_DEC T_ASSIGN S_DATA")
def S_VAR_ASSIGN(vardec, assign, data):
    """Evaluates an S_STATEMENT node"""
    return vardec + "=" + data;

@Token("T_IDENT")
def T_IDENT(identifier):
    """Evaluates an Identifier"""
    return identifier;
