// Author: Jeroen
// Description: Cycles through Rock, Paper, Scissors
// Name: Cycler with a phase

function (round, my, other) {
    return "RPS"[(round + 1) % 3]
};