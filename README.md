# Rock Paper Scissors

This is a simple Rock Paper Scissors bot programming challange. This is the current top 20 bots:

@

## Rules

- Rock beats scissors (*two points* are awarded)
- Scissors beats paper (one point are awarded)
- Paper beats rock (one point are awarded)

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

