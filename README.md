# Rock Paper Scissors

This is a simple Rock Paper Scissors bot programming challange. This is the current top 20 bots:

<!-- START TABLE -->
| Rank | Name                | Author | Description                       | Score   |
|------|---------------------|--------|-----------------------------------|---------|
| 1    | Learner             | Jeroen | Cycles through Rock, Paper, Scis… | 1028.14 |
| 2    | Delayed Mirror      | Jeroen | Mirrors the opponent's second pr… | 704.57  |
| 3    | Cycler              | Jeroen | Cycles through Rock, Paper, Scis… | 488.57  |
| 4    | Cycler with a phase | Jeroen | Cycles through Rock, Paper, Scis… | 488.14  |
| 5    | Reverse Cycler      | Jeroen | Reverse Cycler agent.             | 488.0   |
| 6    | RockScissors        | Jeroen | Loves Rock, deviates with Scisso… | 392.71  |
| 7    | Rock                | Jeroen | Always Rocks                      | 357.14  |
| 8    | Mirror              | Jeroen | Mirrors the opponent's previous … | 127.57  |
<!-- END TABLE -->

## Rules

Agents play 1000 rounds of rock paper scissors with the standard rules, except for one small change:

- Rock beats scissors (**two points** are awarded)
- Scissors beats paper (one point is awarded)
- Paper beats rock (one point is awarded)

## Goal
The goal of the agent is to score on average the most points against all opponents. Come up with a strategy which maximizes score over all possible opponent strategies

## Installation

1. Clone the repository: `git clone https://github.com/your-username/rock-paper-scissors.git`
2. Navigate to the project directory: `cd rock-paper-scissors`
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

function (round, my_moves, other_moves) {
    return "RPS"[round % 3]
};
```

The function takes three arguments:
1. the round number we are in (int)
2. A list of previously played moves of the agent
3. A list of previously played moves of the opponent agent

## Testing your agent

