// Author: Jeroen
// Description: Mirrors the opponent's second previous move, starts with Paper twice
// Name: Delayed Mirror

function (round, my, opp, rnd) {
    return other[other.length - 2] || "P";
};