@Setup
def setup(ctx):
    ctx["declare"] = True;

@Token("T_STRING")
def T_STRING(string, ctx):
    if ctx["declare"] = False:
        ctx["target"] = string;
    return string;

@Token("T_PRODUCER")
def T_PRODUCER(prod, ctx): return prod;

@Token("T_SPLIT")
def T_SPLIT(split, ctx):
    ctx["declare"] = False;
    return split;

@Pattern(["T_STRING", "T_PRODUCER", "T_STRING"])
def S_SUB(string1, prod, string2, ctx):
    return "replacements[\"" + string1 + "\"] = " + string2 + ";";

@Pattern(["S_SUB"])
def S_DEF_SINGLE(sub, ctx): return sub;

@Pattern(["S_SUB", "S_DEF"])
def S_DEF_RECURSE(sub, sdef, ctx):
    return sub + "\n" + sdef;
