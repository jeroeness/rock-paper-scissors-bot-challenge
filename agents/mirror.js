// Author: Jeroen
// Description: Mirrors the opponent's previous move, starts with Rock
// Name: Mirror

function (round, my, opp, rnd) {
    return other[other.length - 1] || "RPS"[round % 3]
};