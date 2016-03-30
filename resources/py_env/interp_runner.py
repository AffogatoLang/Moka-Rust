#!/bin/python

################################################################################
# Notes:
# This program is only intended to be called as main from the Moka compiler
#     and *not* from the command line proper. If you try it and something
#     breaks, not my fault.
# The passed arguments are pre-vetted and correctly formatted by Moka. This
#     enables some efficieny improvements with the lack of checking, but again
#     means that this program is *not* suitable to be called arbitrarily. Don't.
################################################################################

import sys, json;

argv = sys.argv;

if len(argv) != 3:
    raise Exception("Invalid Number Of Arguments Provided");
else:
    sys.path.append(argv[1]);

tokens = {};
patterns = {};
context_dict = {};

# Tokens are defined as single strings
def Token(tok):
    def token_impl(f):
        tokens[tok] = f;
    return token_impl;

# Patterns are arrays of token and pattern names
def Pattern(pat):
    flat_pat = ",".join(pat);
    def pattern_impl(f):
        patterns[flat_pat] = f;
    return pattern_impl;

# Directory containing mokapy has been appended to path
from mokapy.glob import mokadef;
mokadef(Token, Pattern); # Load all defs into dictionaries

leaves = [];
tree_data = json.loads(argv[2]);

#
