// Author: Jeroen
// Description: Random Counter agent.
// Name: Random Counter

function (round, my, opp, rnd) {
    if (round < 3) return "RPS"[Math.floor(rnd * 3)];
    opp = opp.substr(opp.length - Math.min(30, opp.length))
    pred = opp[Math.floor(rnd * opp.length)]
    return pred == "R" ? "P" : pred == "P" ? "S" : "R";

};