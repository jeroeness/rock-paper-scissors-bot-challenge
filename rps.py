import js2py
import argparse
import glob


def play(code1, code2, verbose=False):
    moves = [[], []]
    codes = [code1, code2]
    scores = [0, 0]
    p1, p2 = js2py.eval_js(codes[0]), js2py.eval_js(codes[1])
    for r in range(1, 1001):
        m1 = p1(r, moves[0], moves[1])
        m2 = p2(r, moves[1], moves[0])
        assert m1 in ["R", "P", "S"], f"Invalid move from {codes[0]}: {m1}"
        assert m2 in ["R", "P", "S"], f"Invalid move from {codes[1]}: {m2}"
        if verbose:
            print(f"Round {r}: {m1} vs {m2}")
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
    if verbose:
        print(f"\nFinal score: {scores[0]}:{scores[1]}")
    return scores

    if __name__ == "__main__":
        parser = argparse.ArgumentParser()
        parser.add_argument("agent", help="path to agent js file")
        parser.add_argument("--opponent", help="path to opponent js file")
        parser.add_argument(
            "-v", "--verbose", help="print game state", action="store_true"
        )
        args = parser.parse_args()

        code = open(args.agent, "r").read()
        if args.opponent:
            code2 = open(args.opponent, "r").read()
            scores = play(code, args.code2, verbose=args.verbose)
            print(f"Final score: {scores[0]}:{scores[1]}")
        else:
            js_files = glob.glob("./agents/*.js")
            for file in js_files:
                code2 = open(file, "r").read()
                scores = play(code, code2, verbose=args.verbose)
                print(f"agent:{file} => {scores[0]}:{scores[1]}")
