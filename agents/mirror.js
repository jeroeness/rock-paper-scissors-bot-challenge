// Author: Jeroen
// Description: Mirrors the opponent's previous move, starts with Rock
// Name: Mirror

function (round, my, opp, rnd) {
    return opp[opp.length - 1] || "R";
};