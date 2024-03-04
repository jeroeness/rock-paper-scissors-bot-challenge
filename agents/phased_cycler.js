// Author: Jeroen
// Description: Cycles through Paper, Scissors, Rock
// Name: Cycler with a phase

function (round, my, other) {
    return "RPS"[(round + 1) % 3]
};