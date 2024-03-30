// Author: Jeroen
// Description: Random agent biased to Rock.
// Name: Random Biased

function (round, my, opp, rnd) {
    return "RRPS"[Math.floor(rnd * 4)];
};