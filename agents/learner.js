// Author: Jeroen
// Description: Predicts opponent moves
// Name: Learner

function (round, my, opp, rnd) {
    if (opp.length < 3) {
        return "R";
    }
    for (i = opp.length / 2; i > 0; i--) {
        pattern = opp.substr(opp.length - i, opp.length)
        search = opp.substr(0, opp.length - 1)
        if (search.lastIndexOf(pattern) != -1) {
            predict = opp.charAt(search.lastIndexOf(pattern) + pattern.length);
            return predict == "R" ? "P" : predict == "P" ? "S" : "R";
        }
    }
    return "RPS"[round % 3]
};