// Author: Jeroen
// Description: Mirrors the opponent's second previous move
// Name: Delayed Mirror

function (round, my, opp, rnd) {
    if (round == 1) return rnd < 0.5 ? "P" : "R";
    if (round <= 10 && opp[round - 2] == my[round - 2]) return rnd < 0.5 ? "P" : "R";
    if (round <= 10 && opp[round - 2] != my[round - 2] && opp[round - 2] != "S") return opp[round - 2] == "R" ? "R" : "P";
    return opp[opp.length - 2] || "P";
};