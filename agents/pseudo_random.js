// Author: Jeroen
// Description: Pseudo random agent.
// Name: Pseudo Random

function (round, my, opp, rnd) {
    return "RPS"[Math.floor(rnd() * 3)];
};