# Rock Paper Scissors

This is a simple Rock Paper Scissors bot programming challange.

## Hall of fame

<!-- START TABLE -->
| Rank | Name                | Author | Description                       | Score  |
|------|---------------------|--------|-----------------------------------|--------|
| 1    | Learner             | Jeroen | Predicts opponent moves           | 725.48 |
| 2    | Delayed Mirror      | Jeroen | Mirrors the opponent's second pr… | 587.84 |
| 3    | Cycler with a phase | Jeroen | Cycles through Paper, Scissors, … | 420.01 |
| 4    | Reverse Cycler      | Jeroen | Reverse Cycler agent.             | 311.44 |
| 5    | Cycler              | Jeroen | Cycles through Rock, Paper, Scis… | 310.56 |
| 6    | RockScissors        | Jeroen | Loves Rock, deviates with Scisso… | 122.5  |
| 7    | Pseudo Random       | Jeroen | Pseudo random agent.              | 79.82  |
| 8    | Rock                | Jeroen | Always Rocks                      | 52.63  |
| 9    | Mirror              | Jeroen | Mirrors the opponent's previous … | 33.72  |
<!-- END TABLE -->

## Game Rules

Agents play 1000 rounds of rock paper scissors with the standard rules, except for one small change:

- Rock beats scissors (**two points** are awarded)
- Scissors beats paper (one point is awarded)
- Paper beats rock (one point is awarded)
- Zero points are awarded in other cases

Every turn the bot has to return one of the three choices: Rock, Paper or Scissors indicated by an "R", "P" or "S" respectively. The table below shows the rewards for both agents after playing their move.
|   \   | **R** | **P** | **S** |
|-------|-------|-------|-------|
| **R** | 0 \ 0 | 0 \ 1 | 2 \ 0 |
| **P** | 1 \ 0 | 0 \ 0 | 0 \ 1 |
| **S** | 0 \ 2 | 1 \ 0 | 0 \ 0 |



## Goal
The goal of the agent is to score on average the most points against all opponents. Come up with a strategy which maximizes score over all possible opponent strategies. Also consider maximizing the score of the agent playing against itself.

## Installation

1. Clone the repository: `git clone https://github.com/jeroeness/rock-paper-scissors-bot-challenge.git`
2. Navigate to the project directory: `cd rock-paper-scissors-bot-challenge`
3. Install the dependencies: `pip install -r requirements.txt`

## Creating Your Own Agent

To create your own agent, follow these steps:

1. Navigate to the `agents` folder in the project directory: `cd agents`
2. Create a new javascript for your agent, e.g., `my_agent.js` which contains a single anonymous function
3. Implement your agent's logic in the script. Here's a sample script to get you started:

```js
// Author: <<Your name here>>
// Description: <<Brief description here>>
// Name: <<Agent name here>>

function (round, moves, opp_moves, rnd) {
    return "RPS"[Math.floor(rnd * 3)]
};
```

The function takes three arguments:
1. `round`  the round number we are in (int)
2. `moves` An array of previously played moves of the agent
3. `opp_moves` An array of previously played moves of the opponent agent
4. `rnd` a randomly generated number: $0 \leq rnd \leq 1$ (float)

## Testing your agent
You can test your agent by running it against a specific other againt:
```bash
python rps.py agents/delayed_mirror.js -o agents/learner.js -v -r 10
```

Or against all other agents:
```bash
python rps.py agents/pseudo_random.js -r 100
```

An example with self play:
```bash
python rps.py agents/delayed_mirror.js -o agents/delayed_mirror.js -v -r20
```

## Participate!
Please do participate in this contest. Submit your agent as a js file in the agents folder with a pull request.

We use fair play rules for agents:
* Random without a deterministic seed is not allowed, use the random number `rnd` that is provided by the function parameters. This number is deterministic but different for every round and for both agents.
* Do not make the bot halt for longer than 2ms
* Do not use libraries other than the standard library.
* Don't use exploits
* The bot must be completely autonomous
* Do not use massive amounts of memory or CPU resources.
* Malicious code is not allowed
* Only regular ECMAScript 5.1 is allowed
* Do not read/write to persistent storage or memory

## Running the tournament
You could run the tournament to determine how all agents fare against eachother. The final ranking will be published in the top of this readme file.

Running the tournament might take several minutes:
```bash
python tournament.py
```

Bot tournament rules:
* If a bot does not return either "R", "P", "S" in time, it is disqualified from the tournament.
* If an exception occurs in an agent, it is disqualified from the tournament.