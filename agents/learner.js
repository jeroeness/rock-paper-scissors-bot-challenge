// Author: Jeroen
// Description: Predicts opponent moves
// Name: Learner

function (round, my, opp, rnd) {
    seq = other.join('');
    if (seq.length < 3) {
        return "R";
    }
    for (i = seq.length / 2; i > 0; i--) {
        pattern = seq.substr(seq.length - i, seq.length)
        search = seq.substr(0, seq.length - 1)
        if (search.lastIndexOf(pattern) != -1) {
            predict = seq.charAt(search.lastIndexOf(pattern) + pattern.length);
            return predict == "R" ? "P" : predict == "P" ? "S" : "R";
        }
    }
    return "RPS"[round % 3]
};