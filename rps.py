import js2py
from js2py.internals import seval
import argparse


def play(code1, code2):
    moves = [[], []]
    codes = [code1, code2]
    scores = [0, 0]
    p1, p2 = js2py.eval_js(codes[0]), js2py.eval_js(codes[1])
    for r in range(1, 11):
        m1 = p1(r, moves[0], moves[1])
        m2 = p2(r, moves[1], moves[0])
        moves[0].append(m1)
        moves[1].append(m2)
        if m1 == m2:
            pass
        elif m1 == "S" and m2 == "P":
            scores[0] += 1
        elif m1 == "P" and m2 == "R":
            scores[0] += 1
        elif m1 == "R" and m2 == "S":
            scores[0] += 2
        elif m2 == "R":
            scores[1] += 2
        else:
            scores[1] += 1
    print(f"\nFinal score: {scores[0]}:{scores[1]}")
    return scores
