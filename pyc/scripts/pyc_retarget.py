#! /usr/bin/env python
# -*- coding: utf-8 -*-

import sys
import os
import argparse


def add_import():
    import sys
    import os
    dir_path = os.path.dirname(os.path.realpath(__file__))

    sys.path.insert(0, os.path.join(dir_path, "modules"))


add_import()


import pymarshal_remap  # noqa

PYTHON3 = sys.version_info >= (3, 0)


class dotdict(dict):
    """dot.notation access to dictionary attributes"""
    __getattr__ = dict.get
    __setattr__ = dict.__setitem__
    __delattr__ = dict.__delitem__


def get_messiah_opcodes():
    opmap = dotdict({})
    opname = [''] * 256
    for op in range(256):
        opname[op] = '<%r>' % (op,)
    del op

    def def_op(name, op):
        assert(op not in opname)
        opname[op] = name
        assert(name not in opmap)
        opmap[name] = op

    # Instruction opcodes for compiled code
    # Blank lines correspond to available opcodes

    def_op('POP_TOP', 68)
    def_op('ROT_TWO', 58)
    def_op('ROT_THREE', 62)
    def_op('DUP_TOP', 84)
    def_op('ROT_FOUR', 56)

    def_op('NOP', 9)
    def_op('UNARY_POSITIVE', 10)
    def_op('UNARY_NEGATIVE', 11)
    def_op('UNARY_NOT', 12)
    def_op('UNARY_CONVERT', 13)

    def_op('UNARY_INVERT', 15)

    def_op('BINARY_POWER', 19)
    def_op('BINARY_MULTIPLY', 80)
    def_op('BINARY_DIVIDE', 22)
    def_op('BINARY_MODULO', 83)
    def_op('BINARY_ADD', 89)
    def_op('BINARY_SUBTRACT', 1)
    def_op('BINARY_SUBSCR', 24)
    def_op('BINARY_FLOOR_DIVIDE', 26)
    def_op('BINARY_TRUE_DIVIDE', 27)
    def_op('INPLACE_FLOOR_DIVIDE', 28)
    def_op('INPLACE_TRUE_DIVIDE', 29)
    def_op('SLICE_0', 30)
    def_op('SLICE_1', 31)
    def_op('SLICE_2', 32)
    def_op('SLICE_3', 33)

    def_op('STORE_SLICE_0', 40)
    def_op('STORE_SLICE_1', 41)
    def_op('STORE_SLICE_2', 42)
    def_op('STORE_SLICE_3', 43)

    def_op('DELETE_SLICE_0', 50)
    def_op('DELETE_SLICE_1', 51)
    def_op('DELETE_SLICE_2', 52)
    def_op('DELETE_SLICE_3', 53)

    def_op('STORE_MAP', 78)
    def_op('INPLACE_ADD', 2)
    def_op('INPLACE_SUBTRACT', 20)
    def_op('INPLACE_MULTIPLY', 60)
    def_op('INPLACE_DIVIDE', 23)
    def_op('INPLACE_MODULO', 63)
    def_op('STORE_SUBSCR', 3)
    def_op('DELETE_SUBSCR', 75)
    def_op('BINARY_LSHIFT', 61)
    def_op('BINARY_RSHIFT', 0)
    def_op('BINARY_AND', 57)
    def_op('BINARY_XOR', 65)
    def_op('BINARY_OR', 55)
    def_op('INPLACE_POWER', 64)
    def_op('GET_ITER', 59)

    def_op('PRINT_EXPR', 70)
    def_op('PRINT_ITEM', 71)
    def_op('PRINT_NEWLINE', 72)
    def_op('PRINT_ITEM_TO', 73)
    def_op('PRINT_NEWLINE_TO', 74)
    def_op('INPLACE_LSHIFT', 85)
    def_op('INPLACE_RSHIFT', 66)
    def_op('INPLACE_AND', 86)
    def_op('INPLACE_XOR', 21)
    def_op('INPLACE_OR', 4)
    def_op('BREAK_LOOP', 5)
    def_op('WITH_CLEANUP', 81)
    def_op('LOAD_LOCALS', 76)
    def_op('RETURN_VALUE', 88)
    def_op('IMPORT_STAR', 54)
    def_op('EXEC_STMT', 67)
    def_op('YIELD_VALUE', 79)
    def_op('POP_BLOCK', 82)
    def_op('END_FINALLY', 87)
    def_op('BUILD_CLASS', 77)

    def_op('STORE_NAME', 135)
    def_op('DELETE_NAME', 120)
    def_op('UNPACK_SEQUENCE', 92)
    def_op('FOR_ITER', 121)
    def_op('LIST_APPEND', 124)
    def_op('STORE_ATTR', 126)
    def_op('DELETE_ATTR', 107)
    def_op('STORE_GLOBAL', 106)
    def_op('DELETE_GLOBAL', 96)
    def_op('DUP_TOPX', 115)
    def_op('LOAD_CONST', 100)
    def_op('LOAD_NAME', 101)
    def_op('BUILD_TUPLE', 102)
    def_op('BUILD_LIST', 99)
    def_op('BUILD_SET', 134)
    def_op('BUILD_MAP', 93)
    def_op('LOAD_ATTR', 114)
    def_op('COMPARE_OP', 146)
    def_op('IMPORT_NAME', 108)
    def_op('IMPORT_FROM', 109)
    def_op('JUMP_FORWARD', 110)
    # Target byte offset from beginning of code
    def_op('JUMP_IF_FALSE_OR_POP', 111)
    def_op('JUMP_IF_TRUE_OR_POP', 112)
    def_op('JUMP_ABSOLUTE', 113)
    def_op('POP_JUMP_IF_FALSE', 94)
    def_op('POP_JUMP_IF_TRUE', 104)

    def_op('LOAD_GLOBAL', 116)

    def_op('CONTINUE_LOOP', 90)
    def_op('SETUP_LOOP', 105)
    def_op('SETUP_EXCEPT', 137)
    def_op('SETUP_FINALLY', 147)

    def_op('LOAD_FAST', 95)
    def_op('STORE_FAST', 103)
    def_op('DELETE_FAST', 97)

    def_op('RAISE_VARARGS', 130)
    def_op('CALL_FUNCTION', 131)
    def_op('MAKE_FUNCTION', 132)
    def_op('BUILD_SLICE', 133)
    def_op('MAKE_CLOSURE', 119)
    def_op('LOAD_CLOSURE', 91)
    def_op('LOAD_DEREF', 125)
    def_op('STORE_DEREF', 136)

    def_op('CALL_FUNCTION_VAR', 140)
    def_op('CALL_FUNCTION_KW', 141)
    def_op('CALL_FUNCTION_VAR_KW', 142)

    def_op('SETUP_WITH', 143)

    def_op('EXTENDED_ARG', 145)
    def_op('SET_ADD', 98)
    def_op('MAP_ADD', 122)

    def_op('POP_THREE', 6)
    def_op('RETURN_SUBSCR', 7)
    def_op('POP_TWO', 8)

    def_op('LOAD_LOCALS_RETURN_VALUE', 49)
    def_op('POP_TOP_POP_BLOCK', 69)
    def_op('RETURN_CONST', 117)
    def_op('POP_TOP_LOAD_GLOBAL', 118)

    def_op('POP_TOP_JUMP_FORWARD', 123)

    def_op('LOAD_CONST_BINARY_SUBSCR', 127)
    def_op('POP_TOP_LOAD_FAST', 128)
    def_op('LOAD_CONST_STORE_MAP', 129)

    def_op('CALL_FUNCTION_POP_TOP', 138)
    def_op('POP_TOP_LOAD_CONST', 139)

    def_op('LOAD_CONST_LOAD_CONST', 150)

    def_op('STORE_FAST_LOAD_FAST', 151)
    def_op('LOAD_ATTR_LOAD_GLOBAL', 152)
    def_op('LOAD_FAST_CALL_FUNCTION_POP_TOP', 153)
    def_op('COMPARE_OP_JUMP_IF_FALSE', 154)
    def_op('LOAD_CONST_CALL_FUNCTION', 155)
    def_op('LOAD_FAST_LOAD_CONST', 156)
    def_op('STORE_NAME_LOAD_CONST', 157)
    def_op('LOAD_ATTR_LOAD_FAST', 158)
    def_op('MAKE_FUNCTION_STORE_NAME', 159)
    def_op('LOAD_ATTR_CALL_FUNCTION', 160)
    def_op('LOAD_CONST_COMPARE_OP', 161)
    def_op('LOAD_ATTR_LOAD_ATTR', 162)
    def_op('SKIP_CONST', 163)
    def_op('LOAD_CONST_LOAD_CONST_BUILD_TUPLE', 164)
    def_op('LOAD_GLOBAL_CALL_FUNCTION', 165)
    def_op('LOAD_CONST_LOAD_FAST', 166)
    def_op('STORE_FAST_LOAD_GLOBAL', 167)
    def_op('LOAD_FAST_CALL_FUNCTION', 168)
    def_op('CALL_FUNCTION_STORE_FAST', 169)
    def_op('LOAD_FAST_LOAD_ATTR', 170)
    def_op('LOAD_ATTR_CALL_FUNCTION_POP_TOP', 171)
    def_op('LOAD_FAST_LOAD_FAST', 172)
    def_op('LOAD_FAST_ZERO_LOAD_CONST', 173)
    def_op('LOAD_FAST_STORE_ATTR', 174)
    def_op('LOAD_CONST_LOAD_CONST_STORE_MAP', 175)
    def_op('LOAD_GLOBAL_CALL_FUNCTION_POP_TOP', 176)
    def_op('LOAD_GLOBAL_LOAD_FAST', 177)
    def_op('CALL_FUNCTION_POP_TOP_LOAD_FAST', 178)
    def_op('CALL_FUNCTION_CALL_FUNCTION', 179)
    def_op('LOAD_CONST_MAKE_FUNCTION', 180)
    def_op('LOAD_CONST_IMPORT_NAME', 181)
    def_op('LOAD_FAST_LOAD_CONST_BINARY_SUBSCR_LOAD_FAST_LOAD_CONST_BINARY_SUBSCR_CALL_FUNCTION', 182)
    # def_op('UNUSED', 183)
    # def_op('UNUSED', 184)
    # def_op('UNUSED', 185)
    # def_op('UNUSED', 186)
    # def_op('UNUSED', 187)
    def_op('LOAD_GLOBAL_LOAD_ATTR_LOAD_FAST_LOAD_ATTR_LOAD_FAST_LOAD_FAST', 188)
    def_op('LOAD_GLOBAL_LOAD_ATTR_LOAD_ATTR_LOAD_GLOBAL_LOAD_ATTR_LOAD_ATTR', 189)
    def_op('LOAD_FAST_LOAT_ATTR_LOAD_CONST_LOAD_CONST_CALL_FUNCTION', 190)
    def_op('LOAD_GLOABL_LOAD_ATTR_LOAD_ATTR_COMPARE_OP_LOAD_FAST', 191)
    # def_op('UNUSED', 192)
    def_op('LOAD_FAST_LOAD_ATTR_LOAD_FAST_CALL_FUNCTION', 193)
    def_op('LOAD_FAST_LOAD_ATTR_LOAD_FAST_LOAD_ATTR', 194)
    def_op('LOAD_FAST_LOAD_FAST_LOAD_FAST_CALL_FUNCTION', 195)
    def_op('LOAD_ATTR_LOAD_FAST_LOAD_FAST_CALL_FUNCTION', 196)
    def_op('LOAD_FAST_LOAD_ATTR_LOAD_ATTR', 197)
    def_op('LOAD_FAST_LOAD_ATTR_CALL_FUNCTION', 198)
    def_op('LOAD_FAST_LOAD_ATTR_RETURN_VALUE', 199)
    def_op('LOAD_FAST_LOAD_ATTR_JUMP_IF_FALSE', 200)
    def_op('LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST', 201)
    def_op('LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST', 202)
    def_op('LOAD_FAST_LOAD_ATTR_LOAD_FAST', 203)
    def_op('LOAD_GLOBAL_LOAD_ATTR_LOAD_ATTR', 204)
    def_op('LOAD_FAST_LOAD_ATTR_LOAD_CONST', 205)
    def_op('LOAD_GLOBAL_LOAD_FAST_LOAD_CONST', 206)
    def_op('LOAD_FAST_LOAD_FAST_POP_JUMP_IF_FALSE', 207)
    def_op('STORE_FAST_LOAD_FAST_LOAD_CONST_COMPARE_OP', 208)
    def_op('LOAD_FAST_LOAD_CONST_COMPARE_OP_LOAD_FAST', 209)
    def_op('LOAD_DEREF_LOAD_ATTR_LOAD_FAST_BINARY_SUBSCR', 210)
    def_op('STORE_FAST_LOAD_FAST_POP_JUMP_IF_FALSE', 211)
    def_op('LOAD_FAST_LOAD_CONST_BINARY_SUBSCR', 212)
    def_op('LOAD_ATTR_LOAD_FAST_CALL_FUNCTION', 213)
    # def_op('UNUSED', 214)
    def_op('POP_TOP_LOAD_CONST_RETURN_VALUE', 215)
    def_op('LOAD_GLOBAL_LOAD_ATTR_LOAD_FAST', 216)
    def_op('CALL_FUNCTION_POP_TOP_JUMP_ABSOLUTE', 217)
    def_op('STORE_FAST_LOAD_FAST_LOAD_FAST', 218)
    def_op('LOAD_GLOBAL_LOAD_ATTR', 219)
    def_op('LOAD_DEREF_LOAD_ATTR', 220)
    def_op('LOAD_FAST_STORE_FAST', 221)
    def_op('LOAD_FAST_POP_JUMP_IF_FALSE', 222)
    def_op('LOAD_ATTR_COMPARE_OP', 223)
    def_op('STORE_FAST_STORE_FAST', 224)
    def_op('POP_JUMP_IF_FALSE_2', 225)
    def_op('LOAD_FAST_POP_JUMP_IF_TRUE', 226)
    def_op('LOAD_CONST_STORE_FAST', 227)
    def_op('LOAD_FAST_RETURN_VALUE', 228)
    def_op('LOAD_FAST_LOAD_GLOBAL', 229)
    def_op('LOAD_GLOBAL_RETURN_VALUE', 230)
    def_op('LOAD_FAST_BUILD_TUPLE_STORE_FAST', 231)
    def_op('STORE_FAST_LOAD_FAST_LOAD_GLOBAL', 232)

    return opmap


def get_python_27_opcodes():
    opmap = dotdict({})
    opname = [''] * 256
    for op in range(256):
        opname[op] = '<%r>' % (op,)
    del op

    def def_op(name, op):
        assert(op not in opname)
        opname[op] = name
        assert(name not in opmap)
        opmap[name] = op

    # Instruction opcodes for compiled code
    # Blank lines correspond to available opcodes

    def_op('POP_TOP', 1)
    def_op('ROT_TWO', 2)
    def_op('ROT_THREE', 3)
    def_op('DUP_TOP', 4)
    def_op('ROT_FOUR', 5)

    def_op('NOP', 9)
    def_op('UNARY_POSITIVE', 10)
    def_op('UNARY_NEGATIVE', 11)
    def_op('UNARY_NOT', 12)
    def_op('UNARY_CONVERT', 13)

    def_op('UNARY_INVERT', 15)

    def_op('BINARY_POWER', 19)
    def_op('BINARY_MULTIPLY', 20)
    def_op('BINARY_DIVIDE', 21)
    def_op('BINARY_MODULO', 22)
    def_op('BINARY_ADD', 23)
    def_op('BINARY_SUBTRACT', 24)
    def_op('BINARY_SUBSCR', 25)
    def_op('BINARY_FLOOR_DIVIDE', 26)
    def_op('BINARY_TRUE_DIVIDE', 27)
    def_op('INPLACE_FLOOR_DIVIDE', 28)
    def_op('INPLACE_TRUE_DIVIDE', 29)
    def_op('SLICE_0', 30)
    def_op('SLICE_1', 31)
    def_op('SLICE_2', 32)
    def_op('SLICE_3', 33)

    def_op('STORE_SLICE_0', 40)
    def_op('STORE_SLICE_1', 41)
    def_op('STORE_SLICE_2', 42)
    def_op('STORE_SLICE_3', 43)

    def_op('DELETE_SLICE_0', 50)
    def_op('DELETE_SLICE_1', 51)
    def_op('DELETE_SLICE_2', 52)
    def_op('DELETE_SLICE_3', 53)

    def_op('STORE_MAP', 54)
    def_op('INPLACE_ADD', 55)
    def_op('INPLACE_SUBTRACT', 56)
    def_op('INPLACE_MULTIPLY', 57)
    def_op('INPLACE_DIVIDE', 58)
    def_op('INPLACE_MODULO', 59)
    def_op('STORE_SUBSCR', 60)
    def_op('DELETE_SUBSCR', 61)
    def_op('BINARY_LSHIFT', 62)
    def_op('BINARY_RSHIFT', 63)
    def_op('BINARY_AND', 64)
    def_op('BINARY_XOR', 65)
    def_op('BINARY_OR', 66)
    def_op('INPLACE_POWER', 67)
    def_op('GET_ITER', 68)

    def_op('PRINT_EXPR', 70)
    def_op('PRINT_ITEM', 71)
    def_op('PRINT_NEWLINE', 72)
    def_op('PRINT_ITEM_TO', 73)
    def_op('PRINT_NEWLINE_TO', 74)
    def_op('INPLACE_LSHIFT', 75)
    def_op('INPLACE_RSHIFT', 76)
    def_op('INPLACE_AND', 77)
    def_op('INPLACE_XOR', 78)
    def_op('INPLACE_OR', 79)
    def_op('BREAK_LOOP', 80)
    def_op('WITH_CLEANUP', 81)
    def_op('LOAD_LOCALS', 82)
    def_op('RETURN_VALUE', 83)
    def_op('IMPORT_STAR', 84)
    def_op('EXEC_STMT', 85)
    def_op('YIELD_VALUE', 86)
    def_op('POP_BLOCK', 87)
    def_op('END_FINALLY', 88)
    def_op('BUILD_CLASS', 89)

    def_op('STORE_NAME', 90)
    def_op('DELETE_NAME', 91)
    def_op('UNPACK_SEQUENCE', 92)
    def_op('FOR_ITER', 93)
    def_op('LIST_APPEND', 94)
    def_op('STORE_ATTR', 95)
    def_op('DELETE_ATTR', 96)
    def_op('STORE_GLOBAL', 97)
    def_op('DELETE_GLOBAL', 98)
    def_op('DUP_TOPX', 99)
    def_op('LOAD_CONST', 100)
    def_op('LOAD_NAME', 101)
    def_op('BUILD_TUPLE', 102)
    def_op('BUILD_LIST', 103)
    def_op('BUILD_SET', 104)
    def_op('BUILD_MAP', 105)
    def_op('LOAD_ATTR', 106)
    def_op('COMPARE_OP', 107)
    def_op('IMPORT_NAME', 108)
    def_op('IMPORT_FROM', 109)
    def_op('JUMP_FORWARD', 110)
    # Target byte offset from beginning of code
    def_op('JUMP_IF_FALSE_OR_POP', 111)
    def_op('JUMP_IF_TRUE_OR_POP', 112)
    def_op('JUMP_ABSOLUTE', 113)
    def_op('POP_JUMP_IF_FALSE', 114)
    def_op('POP_JUMP_IF_TRUE', 115)

    def_op('LOAD_GLOBAL', 116)

    def_op('CONTINUE_LOOP', 119)
    def_op('SETUP_LOOP', 120)
    def_op('SETUP_EXCEPT', 121)
    def_op('SETUP_FINALLY', 122)

    def_op('LOAD_FAST', 124)
    def_op('STORE_FAST', 125)
    def_op('DELETE_FAST', 126)

    def_op('RAISE_VARARGS', 130)
    def_op('CALL_FUNCTION', 131)
    def_op('MAKE_FUNCTION', 132)
    def_op('BUILD_SLICE', 133)
    def_op('MAKE_CLOSURE', 134)
    def_op('LOAD_CLOSURE', 135)
    def_op('LOAD_DEREF', 136)
    def_op('STORE_DEREF', 137)

    def_op('CALL_FUNCTION_VAR', 140)
    def_op('CALL_FUNCTION_KW', 141)
    def_op('CALL_FUNCTION_VAR_KW', 142)

    def_op('SETUP_WITH', 143)

    def_op('EXTENDED_ARG', 145)
    def_op('SET_ADD', 146)
    def_op('MAP_ADD', 147)

    return opmap


class PYCRetargeter(object):
    def __init__(self):
        py27_opcodes = get_python_27_opcodes()
        opc = py27_opcodes
        messiah_opcodes = get_messiah_opcodes()

        # LEFT (Messiah) RIGHT (Native)

        self.opcode_map = {}

        for op in py27_opcodes:
            self.opcode_map[messiah_opcodes[op]] = py27_opcodes[op]

        self.opcode_expansion = {
            messiah_opcodes.POP_THREE: [opc.POP_TOP, opc.POP_TOP, opc.POP_TOP],
            messiah_opcodes.RETURN_SUBSCR: [
                opc.BINARY_SUBSCR, opc.RETURN_VALUE],
            messiah_opcodes.POP_TWO: [opc.POP_TOP, opc.POP_TOP],
            messiah_opcodes.LOAD_LOCALS_RETURN_VALUE: [opc.LOAD_LOCALS, opc.RETURN_VALUE],
            messiah_opcodes.POP_TOP_POP_BLOCK: [opc.POP_TOP, opc.POP_BLOCK],
            messiah_opcodes.RETURN_CONST: [opc.LOAD_CONST, opc.RETURN_VALUE],
            messiah_opcodes.POP_TOP_LOAD_GLOBAL: [opc.POP_TOP, opc.LOAD_GLOBAL],
            messiah_opcodes.POP_TOP_JUMP_FORWARD: [opc.POP_TOP, opc.JUMP_FORWARD],
            messiah_opcodes.LOAD_CONST_BINARY_SUBSCR: [opc.LOAD_CONST, opc.BINARY_SUBSCR],
            messiah_opcodes.POP_TOP_LOAD_FAST: [opc.POP_TOP, opc.LOAD_FAST],
            messiah_opcodes.LOAD_CONST_STORE_MAP: [opc.LOAD_CONST, opc.STORE_MAP],
            messiah_opcodes.CALL_FUNCTION_POP_TOP: [opc.CALL_FUNCTION, opc.POP_TOP],
            messiah_opcodes.POP_TOP_LOAD_CONST: [opc.POP_TOP, opc.LOAD_CONST],
            messiah_opcodes.LOAD_CONST_LOAD_CONST: [opc.LOAD_CONST, opc.LOAD_CONST],
            messiah_opcodes.STORE_FAST_LOAD_FAST: [opc.STORE_FAST, opc.LOAD_FAST],
            messiah_opcodes.LOAD_ATTR_LOAD_GLOBAL: [opc.LOAD_ATTR, opc.LOAD_GLOBAL],
            messiah_opcodes.LOAD_FAST_CALL_FUNCTION_POP_TOP: [opc.LOAD_FAST, opc.CALL_FUNCTION, opc.POP_TOP],
            messiah_opcodes.COMPARE_OP_JUMP_IF_FALSE: [opc.COMPARE_OP, opc.POP_JUMP_IF_FALSE],
            messiah_opcodes.LOAD_CONST_CALL_FUNCTION: [opc.LOAD_CONST, opc.CALL_FUNCTION],
            messiah_opcodes.LOAD_FAST_LOAD_CONST: [opc.LOAD_FAST, opc.LOAD_CONST],
            messiah_opcodes.STORE_NAME_LOAD_CONST: [opc.STORE_NAME, opc.LOAD_CONST],
            messiah_opcodes.LOAD_ATTR_LOAD_FAST: [opc.LOAD_ATTR, opc.LOAD_FAST],
            messiah_opcodes.MAKE_FUNCTION_STORE_NAME: [opc.MAKE_FUNCTION, opc.STORE_NAME],
            messiah_opcodes.LOAD_ATTR_CALL_FUNCTION: [opc.LOAD_ATTR, opc.CALL_FUNCTION],
            messiah_opcodes.LOAD_CONST_COMPARE_OP: [opc.LOAD_CONST, opc.COMPARE_OP],
            messiah_opcodes.LOAD_ATTR_LOAD_ATTR: [opc.LOAD_ATTR, opc.LOAD_ATTR],
            # SKIP CONST
            messiah_opcodes.LOAD_CONST_LOAD_CONST_BUILD_TUPLE: [opc.LOAD_CONST, opc.LOAD_CONST, opc.BUILD_TUPLE],
            messiah_opcodes.LOAD_GLOBAL_CALL_FUNCTION: [opc.LOAD_GLOBAL, opc.CALL_FUNCTION],
            messiah_opcodes.LOAD_CONST_LOAD_FAST: [opc.LOAD_CONST, opc.LOAD_FAST],
            messiah_opcodes.STORE_FAST_LOAD_GLOBAL: [opc.STORE_FAST, opc.LOAD_GLOBAL],
            messiah_opcodes.LOAD_FAST_CALL_FUNCTION: [opc.LOAD_FAST, opc.CALL_FUNCTION],
            messiah_opcodes.CALL_FUNCTION_STORE_FAST: [opc.CALL_FUNCTION, opc.STORE_FAST],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR: [opc.LOAD_FAST, opc.LOAD_ATTR],
            messiah_opcodes.LOAD_ATTR_CALL_FUNCTION_POP_TOP: [opc.LOAD_ATTR, opc.CALL_FUNCTION, opc.POP_TOP],
            messiah_opcodes.LOAD_FAST_LOAD_FAST: [opc.LOAD_FAST, opc.LOAD_FAST],
            # TODO: this needs something special :)
            messiah_opcodes.LOAD_FAST_ZERO_LOAD_CONST: [
                (opc.LOAD_FAST, int.to_bytes(0, 2, byteorder='little')),
                opc.LOAD_CONST
            ],
            messiah_opcodes.LOAD_FAST_STORE_ATTR: [opc.LOAD_FAST, opc.STORE_ATTR],
            messiah_opcodes.LOAD_CONST_LOAD_CONST_STORE_MAP: [opc.LOAD_CONST, opc.LOAD_CONST, opc.STORE_MAP],
            messiah_opcodes.LOAD_GLOBAL_CALL_FUNCTION_POP_TOP: [opc.LOAD_GLOBAL, opc.CALL_FUNCTION, opc.POP_TOP],
            messiah_opcodes.LOAD_GLOBAL_LOAD_FAST: [opc.LOAD_GLOBAL, opc.LOAD_FAST],
            messiah_opcodes.CALL_FUNCTION_POP_TOP_LOAD_FAST: [opc.CALL_FUNCTION, opc.POP_TOP, opc.LOAD_FAST],
            messiah_opcodes.CALL_FUNCTION_CALL_FUNCTION: [opc.CALL_FUNCTION, opc.CALL_FUNCTION],
            messiah_opcodes.LOAD_CONST_MAKE_FUNCTION: [opc.LOAD_CONST, opc.MAKE_FUNCTION],
            messiah_opcodes.LOAD_CONST_IMPORT_NAME: [opc.LOAD_CONST, opc.IMPORT_NAME],
            messiah_opcodes.LOAD_FAST_LOAD_CONST_BINARY_SUBSCR_LOAD_FAST_LOAD_CONST_BINARY_SUBSCR_CALL_FUNCTION: [
                opc.LOAD_FAST,
                opc.LOAD_CONST,
                opc.BINARY_SUBSCR,
                opc.LOAD_FAST,
                opc.LOAD_CONST,
                opc.BINARY_SUBSCR,
                opc.CALL_FUNCTION
            ],
            messiah_opcodes.LOAD_GLOBAL_LOAD_ATTR_LOAD_FAST_LOAD_ATTR_LOAD_FAST_LOAD_FAST: [
                opc.LOAD_GLOBAL,
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
                opc.LOAD_FAST
            ],
            messiah_opcodes.LOAD_GLOBAL_LOAD_ATTR_LOAD_ATTR_LOAD_GLOBAL_LOAD_ATTR_LOAD_ATTR: [
                opc.LOAD_GLOBAL,
                opc.LOAD_ATTR,
                opc.LOAD_ATTR,
                opc.LOAD_GLOBAL,
                opc.LOAD_ATTR,
                opc.LOAD_ATTR,
            ],
            messiah_opcodes.LOAD_FAST_LOAT_ATTR_LOAD_CONST_LOAD_CONST_CALL_FUNCTION: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.LOAD_CONST,
                opc.LOAD_CONST,
                opc.CALL_FUNCTION,
            ],
            messiah_opcodes.LOAD_GLOABL_LOAD_ATTR_LOAD_ATTR_COMPARE_OP_LOAD_FAST: [
                opc.LOAD_GLOBAL,
                opc.LOAD_ATTR,
                opc.LOAD_ATTR,
                opc.COMPARE_OP,
                opc.LOAD_FAST,
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_LOAD_FAST_CALL_FUNCTION: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
                opc.CALL_FUNCTION,
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_LOAD_FAST_LOAD_ATTR: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
            ],
            messiah_opcodes.LOAD_FAST_LOAD_FAST_LOAD_FAST_CALL_FUNCTION: [
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.CALL_FUNCTION,
            ],
            messiah_opcodes.LOAD_ATTR_LOAD_FAST_LOAD_FAST_CALL_FUNCTION: [
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.CALL_FUNCTION
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_LOAD_ATTR: [
                opc.LOAD_FAST, opc.LOAD_ATTR, opc.LOAD_ATTR
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_CALL_FUNCTION: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.CALL_FUNCTION,
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_RETURN_VALUE: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.RETURN_VALUE
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_JUMP_IF_FALSE: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.POP_JUMP_IF_FALSE
            ],
            messiah_opcodes.LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST: [
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST
            ],
            messiah_opcodes.LOAD_FAST_LOAD_FAST_LOAD_FAST_LOAD_FAST: [
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_LOAD_FAST: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.LOAD_FAST
            ],
            messiah_opcodes.LOAD_GLOBAL_LOAD_ATTR_LOAD_ATTR: [
                opc.LOAD_GLOBAL,
                opc.LOAD_ATTR,
                opc.LOAD_ATTR
            ],
            messiah_opcodes.LOAD_FAST_LOAD_ATTR_LOAD_CONST: [
                opc.LOAD_FAST,
                opc.LOAD_ATTR,
                opc.LOAD_CONST,
            ],
            messiah_opcodes.LOAD_GLOBAL_LOAD_FAST_LOAD_CONST: [
                opc.LOAD_GLOBAL,
                opc.LOAD_FAST,
                opc.LOAD_CONST
            ],
            messiah_opcodes.LOAD_FAST_LOAD_FAST_POP_JUMP_IF_FALSE: [
                opc.LOAD_FAST,
                opc.LOAD_FAST,
                opc.POP_JUMP_IF_FALSE
            ],
            messiah_opcodes.STORE_FAST_LOAD_FAST_LOAD_CONST_COMPARE_OP: [
                opc.STORE_FAST,
                opc.LOAD_FAST,
                opc.LOAD_CONST,
                opc.COMPARE_OP
            ],
            messiah_opcodes.LOAD_FAST_LOAD_CONST_COMPARE_OP_LOAD_FAST: [
                opc.LOAD_FAST,
                opc.LOAD_CONST,
                opc.COMPARE_OP,
                opc.LOAD_FAST,
            ],
            messiah_opcodes.LOAD_DEREF_LOAD_ATTR_LOAD_FAST_BINARY_SUBSCR: [
                opc.LOAD_DEREF,
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
                opc.BINARY_SUBSCR
            ],
            messiah_opcodes.STORE_FAST_LOAD_FAST_POP_JUMP_IF_FALSE: [
                opc.STORE_FAST,
                opc.LOAD_FAST,
                opc.POP_JUMP_IF_FALSE
            ],
            messiah_opcodes.LOAD_FAST_LOAD_CONST_BINARY_SUBSCR: [
                opc.LOAD_FAST,
                opc.LOAD_CONST,
                opc.BINARY_SUBSCR
            ],
            messiah_opcodes.LOAD_ATTR_LOAD_FAST_CALL_FUNCTION: [
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
                opc.CALL_FUNCTION
            ],
            messiah_opcodes.POP_TOP_LOAD_CONST_RETURN_VALUE: [
                opc.POP_TOP,
                opc.LOAD_CONST,
                opc.RETURN_VALUE,
            ],
            messiah_opcodes.LOAD_GLOBAL_LOAD_ATTR_LOAD_FAST: [
                opc.LOAD_GLOBAL,
                opc.LOAD_ATTR,
                opc.LOAD_FAST,
            ],
            messiah_opcodes.CALL_FUNCTION_POP_TOP_JUMP_ABSOLUTE: [
                opc.CALL_FUNCTION,
                opc.POP_TOP,
                opc.JUMP_ABSOLUTE
            ],
            messiah_opcodes.STORE_FAST_LOAD_FAST_LOAD_FAST: [
                opc.STORE_FAST,
                opc.LOAD_FAST,
                opc.LOAD_FAST
            ],
            messiah_opcodes.LOAD_GLOBAL_LOAD_ATTR: [
                opc.LOAD_GLOBAL,
                opc.LOAD_ATTR
            ],
            messiah_opcodes.LOAD_DEREF_LOAD_ATTR: [
                opc.LOAD_DEREF,
                opc.LOAD_ATTR
            ],
            messiah_opcodes.LOAD_FAST_STORE_FAST: [
                opc.LOAD_FAST,
                opc.STORE_FAST,
            ],
            messiah_opcodes.LOAD_FAST_POP_JUMP_IF_FALSE: [
                opc.LOAD_FAST,
                opc.POP_JUMP_IF_FALSE
            ],
            messiah_opcodes.LOAD_ATTR_COMPARE_OP: [
                opc.LOAD_ATTR,
                opc.COMPARE_OP
            ],
            messiah_opcodes.STORE_FAST_STORE_FAST: [
                opc.STORE_FAST,
                opc.STORE_FAST
            ],
            messiah_opcodes.POP_JUMP_IF_FALSE_2: [
                opc.POP_JUMP_IF_FALSE
            ],
            messiah_opcodes.LOAD_FAST_POP_JUMP_IF_TRUE: [
                opc.LOAD_FAST,
                opc.POP_JUMP_IF_TRUE
            ],
            messiah_opcodes.LOAD_CONST_STORE_FAST: [
                opc.LOAD_CONST,
                opc.STORE_FAST
            ],
            messiah_opcodes.LOAD_FAST_RETURN_VALUE: [
                opc.LOAD_FAST,
                opc.RETURN_VALUE,
            ],
            messiah_opcodes.LOAD_FAST_LOAD_GLOBAL: [
                opc.LOAD_FAST,
                opc.LOAD_GLOBAL
            ],
            messiah_opcodes.LOAD_GLOBAL_RETURN_VALUE: [
                opc.LOAD_GLOBAL,
                opc.RETURN_VALUE
            ],
            messiah_opcodes.LOAD_FAST_BUILD_TUPLE_STORE_FAST: [
                opc.LOAD_FAST,
                opc.BUILD_TUPLE,
                opc.STORE_FAST
            ],
            messiah_opcodes.STORE_FAST_LOAD_FAST_LOAD_GLOBAL: [
                opc.STORE_FAST,
                opc.LOAD_FAST,
                opc.LOAD_GLOBAL
            ]
        }
        self.pyc27_header = "\x03\xf3\x0d\x0a\xff\xff\xff\xff"

    def _retarget_file(self, filename):
        content = open(filename, "rb").read()
        return self.retarget_buffer(content)

    def retarget_buffer(self, content):
        try:
            m = pymarshal_remap.loads(content[8:])
        except RuntimeError as e:
            print("[!] error: %s" % str(e))
            return None
        return m.co_filename.replace('\\', '/'), pymarshal_remap.dumps(m, self.opcode_map, self.opcode_expansion)

    def retarget_file(self, input_file, output_file=None):
        result = self._retarget_file(input_file)
        if not result:
            return
        pyc_filename, pyc_content = result
        if not output_file:
            output_file = os.path.basename(pyc_filename) + '.pyc'
        with open(output_file, 'wb') as fd:
            if not PYTHON3:
                fd.write(self.pyc27_header + pyc_content)
            else:
                fd.write(bytearray(
                    map(lambda x: int(ord(x)), self.pyc27_header)) + pyc_content)


def main():
    parser = argparse.ArgumentParser(description='messiah pyc de-opt tool')
    parser.add_argument("INPUT_NAME", help='input file')
    parser.add_argument("OUTPUT_NAME", help='output file')
    args = parser.parse_args()
    retargeter = PYCRetargeter()
    retargeter.retarget_file(args.INPUT_NAME, args.OUTPUT_NAME)


def retarget_buffer(data):
    retargeter = PYCRetargeter()
    pyc_content = retargeter.retarget_buffer(data)
    return bytearray(map(lambda x: int(ord(x)), retargeter.pyc27_header)) + pyc_content


def retarget_file(input, output):
    retargeter = PYCRetargeter()
    retargeter.retarget_file(input, output)


def retarget_file_async(filename, target_filename):
    import sys
    try:
        lock.acquire()
        print("\r", end="")
        sys.stdout.write("\033[K")
        print(filename, end="\r")
        sys.stdout.flush()
    finally:
        lock.release()

    try:
        file = open(filename, 'rb')
        head = file.read(8)
        if head != b"\x03\xf3\x0d\x0a\xff\xff\xff\xff":
            retarget_file(filename, filename)
    except:
        pass

    import gc
    gc.collect()

    try:
        lock.acquire()
        print("\r", end="")
        sys.stdout.write("\033[K")
        print(filename, end='\r')
        sys.stdout.flush()
    finally:
        lock.release()


def retarget_file_async_unpack(args):
    return retarget_file_async(*args)


def init(l):
    global lock
    lock = l


if __name__ == "__main__":
    import sys
    import argparse
    import glob
    parser = argparse.ArgumentParser(description='')
    parser.add_argument('glob_pattern', type=str,  help='')

    args = parser.parse_args()
    files = []
    for filename in glob.glob(args.glob_pattern, recursive=True):
        files.append((filename, filename))

    from multiprocessing import Pool, Lock

    # Create pool for parallel execution
    # The lock here is used to synchronize directory create calls
    lock = Lock()
    import multiprocessing
    pool = Pool(int(multiprocessing.cpu_count() / 2),
                initializer=init, initargs=(lock,))

    init(lock)

    pool.map_async(retarget_file_async_unpack, files).get(999999)


# if __name__ == '__main__':
#     main()
