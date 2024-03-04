// Description: Reverse Cycler agent.
// Name: Reverse Cycler
// Author: Jeroen

function (round, my, other) {
    return "SRP"[round % 3]
};