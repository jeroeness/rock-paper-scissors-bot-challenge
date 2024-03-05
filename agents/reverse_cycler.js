// Description: Reverse Cycler agent.
// Name: Reverse Cycler
// Author: Jeroen

function (round, my, opp, rnd) {
    return "SRP"[round % 3]
};