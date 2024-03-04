# Rock Paper Scissors

This is a simple Rock Paper Scissors bot programming challange. This is the current top 20 bots:

<!-- START TABLE -->
| Rank | Name                | Author | Description                       | Score  |
|------|---------------------|--------|-----------------------------------|--------|
| 1    | Learner             | Jeroen | Cycles through Rock, Paper, Scis… | 952.5  |
| 2    | Delayed Mirror      | Jeroen | Mirrors the opponent's second pr… | 668.88 |
| 3    | Reverse Cycler      | Jeroen | Reverse Cycler agent.             | 484.62 |
| 4    | Cycler with a phase | Jeroen | Cycles through Rock, Paper, Scis… | 482.75 |
| 5    | Cycler              | Jeroen | Cycles through Rock, Paper, Scis… | 481.25 |
| 6    | Pseudo Random       | Jeroen | Pseudo random agent.              | 429.62 |
| 7    | RockScissors        | Jeroen | Loves Rock, deviates with Scisso… | 420.0  |
| 8    | Rock                | Jeroen | Always Rocks                      | 396.5  |
| 9    | Mirror              | Jeroen | Mirrors the opponent's previous … | 161.12 |
<!-- END TABLE -->

## Game Rules

Agents play 1000 rounds of rock paper scissors with the standard rules, except for one small change:

- Rock beats scissors (**two points** are awarded)
- Scissors beats paper (one point is awarded)
- Paper beats rock (one point is awarded)
- Zero points are awarded in other cases

Every turn the bot has to return one of the three choices: Rock, Paper or Scissors indicated by an "R", "P" or "S" respectively.

## Goal
The goal of the agent is to score on average the most points against all opponents. Come up with a strategy which maximizes score over all possible opponent strategies.

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

function (round, my_moves, opponent_moves) {
    return "RPS"[round % 3]
};
```

The function takes three arguments:
1. the round number we are in (int)
2. An array of previously played moves of the agent
3. An array of previously played moves of the opponent agent

## Testing your agent
You can test your agent by running it against a specific other againt:
```bash
python rps.py agents/delayed_mirror.js -o agents/learner.js -v -r 10
```

Or against all other agents:
```bash
python rps.py agents/pseudo_random.js -r 100
```

## Participate!
Please do participate in this contest. Submit your agent as a js file in the agents folder with a pull request.

We use fair play rules:
* Random is not allowed, only use deterministicly random functions.
* Do not make the bot halt for longer than 2ms
* Do not use libraries other than the standard library.
* Don't use exploits
* The bot must be completely autonomous
* Do not use massive amounts of memory or CPU resources.
* Malicious code is not allowed
* Only regular ECMAScript 5.1 is allowed

## Running the tournament
You could run the tournament to determine how all agents fare against eachother. The final ranking will be published in the top of this readme file.

Running the tournament might take several minutes:
```bash
python tournament.py
```

Bot tournament rules:
* If a bot does not return either "R", "P", "S" in time, it is disqualified from the tournament.
* If an exception occurs in an agent, it is disqualified from the tournament.