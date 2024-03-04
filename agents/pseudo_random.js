// Author: Jeroen
// Description: Pseudo random agent.
// Name: Pseudo Random

function (round, my, other) {
    function sfc32(a, b, c, d) {
        return function () {
            a |= 0; b |= 0; c |= 0; d |= 0;
            var t = (a + b | 0) + d | 0;
            d = d + 1 | 0;
            a = b ^ b >>> 9;
            b = c + (c << 3) | 0;
            c = (c << 21 | c >>> 11);
            c = c + t | 0;
            return (t >>> 0) / 4294967296;
        }
    }

    var seed = 42 ^ 0xDEADBEEF;
    var rand = sfc32(0x9E3779B9, 0x243F6A88, 0xB7E15162, seed);
    for (i = 0; i < round; i++) {
        rand();
    }
    return "RPS"[Math.floor(rand() * 3)];
};