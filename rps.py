import js2py
import argparse
import glob
import os


def play(code1, code2, verbose=False, rounds=1000):
    moves = [[], []]
    codes = [code1, code2]
    scores = [0, 0]
    p1, p2 = js2py.eval_js(codes[0]), js2py.eval_js(codes[1])
    for r in range(1, rounds + 1):
        m1 = p1(r, moves[0], moves[1])
        m2 = p2(r, moves[1], moves[0])
        assert m1 in ["R", "P", "S"], f"Invalid move '{m1}' in {codes[0]}"
        assert m2 in ["R", "P", "S"], f"Invalid move '{m2}' in {codes[1]}"
        moves[0].append(m1)
        moves[1].append(m2)
        verbose_score1 = " "
        verbose_score2 = " "
        if m1 == m2:
            pass
        elif m1 == "S" and m2 == "P":
            scores[0] += 1
            verbose_score1 = "*"
        elif m1 == "P" and m2 == "R":
            scores[0] += 1
            verbose_score1 = "*"
        elif m1 == "R" and m2 == "S":
            scores[0] += 2
            verbose_score1 = "*"
        elif m2 == "R":
            scores[1] += 2
            verbose_score2 = "*"
        else:
            scores[1] += 1
            verbose_score2 = "*"
        if verbose:
            print(
                f"Round {r:4}: {verbose_score1}{m1}{verbose_score1}  vs {verbose_score2}{m2}{verbose_score2}"
            )
    if verbose:
        print(f"Scores: {scores[0]}:{scores[1]}")
    return scores


if __name__ == "__main__":
    parser = argparse.ArgumentParser(add_help=True, description="Rock Paper Scissors")
    parser.add_argument("agent", help="path to agent js file")
    parser.add_argument("-o", "--opponent", help="path to opponent js file")
    parser.add_argument(
        "-r", "--rounds", help="number of rounds to play", type=int, default=1000
    )
    parser.add_argument("-v", "--verbose", help="print game state", action="store_true")
    args = parser.parse_args()

    agent_fname = os.path.basename(args.agent)

    code = open(args.agent, "r").read()
    if args.opponent:
        code2 = open(args.opponent, "r").read()
        scores = play(code, code2, verbose=args.verbose, rounds=args.rounds)
        print(f"Final score: {scores[0]}:{scores[1]}")
    else:
        js_files = glob.glob("./agents/*.js")
        total_score = 0
        for file in js_files:
            if os.path.basename(file) == agent_fname:
                continue
            code2 = open(file, "r").read()
            scores = play(code, code2, verbose=args.verbose, rounds=args.rounds)
            total_score += scores[0]
            print(
                f"{scores[0]:4}:{scores[1]:-4} \t {agent_fname} VS {os.path.basename(file)}"
            )
        print(f"Average score: {total_score/(len(js_files)-1):.2f}")
