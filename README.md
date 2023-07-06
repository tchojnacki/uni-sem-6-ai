# Artificial Intelligence Coursework 🧠
<table align="center">
  <thead>
    <tr><th colspan="2">🏫 University Coursework - 6th Term 🏫</th></tr>
    <tr><td colspan="2"><a href="https://pwr.edu.pl/en/">Wrocław University of Science and Technology</a></td></tr>
  </thead>
</table>
<table align="center">
  <tbody>
    <tr>
      <th>Course Name (EN)</th>
      <td>Artificial Intelligence and Knowledge Engineering</td>
    </tr>
    <tr>
      <th>Course Name (PL)</th>
      <td>Sztuczna inteligencja i inżynieria wiedzy</td>
    </tr>
    <tr>
      <th>Technologies</th>
      <td>
        <img src="https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white" alt="Rust" />
        <img src="https://img.shields.io/badge/Python-3670A0?logo=python&logoColor=ffdd54" alt="Python" />
        <img src="https://img.shields.io/badge/Prolog-E11B22" alt="Prolog" />
        <br/>
        <img src="https://img.shields.io/badge/scikit--learn-F7931E?logo=scikit-learn&logoColor=white" alt="scikit-learn" />
        <img src="https://img.shields.io/badge/NumPy-013243?logo=numpy&logoColor=white" alt="NumPy" />
        <img src="https://img.shields.io/badge/pandas-150458?logo=pandas&logoColor=white" alt="pandas" />
        <img src="https://img.shields.io/badge/Jupyter-FA0F00?logo=jupyter&logoColor=white" alt="Jupyter" />
      </td>
    </tr>
  </tbody>
</table>

## Tasks 📝
### LAB1: Solving problems by searching
- **Assignment:** Build a CLI, which finds the most convenient (by time, distance or bus change count) public transport connections in Wrocław between two user given points. The app should also find the most optimal paths containing multiple user defined intermediate destinations.
- **Subdirectory:** `path-finding`
- **Covered algorithms:**
  - [Depth-First Search (DFS)](https://en.wikipedia.org/wiki/Depth-first_search)
  - [Breadth-First Search (BFS)](https://en.wikipedia.org/wiki/Breadth-first_search)
  - [Dijkstra's Algorithm / Uniform Cost Search](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
  - [A* Search](https://en.wikipedia.org/wiki/A*_search_algorithm) (with various heuristics and optimizations)
  - [Tabu Search](https://en.wikipedia.org/wiki/Tabu_search) (with modifications)
- **Key features:**
  - Parsing and building a giant digraph from a `.csv` edge file.
    - Automatic transformation from a [time-dependent graph](https://link.springer.com/article/10.1007/s41019-019-00105-0) to a static graph.
    - Time- and memory-efficient node representation.
      - Use of stack-allocated [`smol_str`](https://github.com/rust-analyzer/smol_str) for strings smaller than 24 bytes.
      - Custom-built [string interning](https://en.wikipedia.org/wiki/String_interning) solution for stop names.
      - Public interface and architecture based on numeric node identifiers.
    - Custom transformation from [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#WGS84) to a local kilometer-based coordinate system.
      - Memory footprint of a single position down from 24 to 8 bytes.
      - Less than 2 centimeters of error in comparison with real-life distance results.
      - Major speed up for any future distance calculations (the transformation makes [haversine](https://en.wikipedia.org/wiki/Haversine_formula) redundant).
      - Researched and built together with [@jakubzehner](https://github.com/jakubzehner).
    - Index-based [adjacency list](https://en.wikipedia.org/wiki/Adjacency_list) representation with $O(1)$ stop name lookups using hash tables.
  - Separation of bus stops with same names but different locations.
    - Ability to walk between different stops of the same name with a small time penalty.
    - Waiting the night if all buses for a given day already left.
  - Path finding using A* and Dijkstra with various optimization criteria.
  - Ability to fine-tune heuristics for better computation time or accuracy (with option to use [non-admissible](https://en.wikipedia.org/wiki/Admissible_heuristic) heuristics).
  - Path finding with intermediate destinations using a hybrid of Tabu Search and A* ([travelling salesman problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem)).
  - A total of 1.5k lines of Rust source code.
  - Covered by around 20 unit tests checking all of the core app logic.

### LAB2: Adversarial search and games
- **Assignment:** Create a terminal implementation of the game of [Reversi](https://en.wikipedia.org/wiki/Reversi) (AKA Othello). You should be able to play against a computer using various Minimax strategies. You should be able to run two instances of the program and make them play against each other.
- **Subdirectory:** `game-theory`
- **Covered algorithms:**
  - [Minimax](https://en.wikipedia.org/wiki/Minimax)
  - [Alpha-Beta Pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning)
  - [Flood Fill](https://en.wikipedia.org/wiki/Flood_fill) ([Dumb7Fill](https://www.chessprogramming.org/Dumb7Fill))
  - [Genetic Algorithms](https://en.wikipedia.org/wiki/Genetic_algorithm)
  - [Monte Carlo Method](https://en.wikipedia.org/wiki/Monte_Carlo_method)
  - [Bidirectional Search](https://en.wikipedia.org/wiki/Bidirectional_search)
- **Key features:**
  - Fully rulebook compliant and blazingly fast Reversi logic implementation extended to support different game variants.
    - Based on optimized [sliding piece attack](https://www.chessprogramming.org/Sliding_Piece_Attacks) chess algorithms.
    - Memory efficient [bitboard](https://www.chessprogramming.org/Bitboards) game state representation.
    - Custom move generation algorithm, which is over 25 times faster than naive implementation.
    - Invalid game states made unrepresentable.
    - Game state reachability checks based on piece stability inspired by [Rosenbloom 1982](https://stacks.stanford.edu/file/druid:wk764yw7162/wk764yw7162.pdf).
  - Player AI using the Minimax algorithm optimized with alpha-beta pruning.
    - Almost 20 available heuristics, including well known trained basic heuristic combinations.
    - Five heuristics trained using genetic algorithms, monte carlo methods and the [Elo rating system](https://en.wikipedia.org/wiki/Elo_rating_system).
    - Other, naive strategies also available.
  - Colored game board view based on [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).
  - Over 2.5k lines of Rust source code.
  - Logic covered with over 80 tests.
    - Unit tests covering basic game logic.
    - End-to-end tests checking the implementation against real world tournament games.
    - [Property testing](https://en.wikipedia.org/wiki/Software_testing#Property_testing) of complicated game logic invariants.

### Notes
The course included a fifth assignment covering **neural networks**, however this exercise was done during the lab and is too insignificant to include here. The goal was to train a [multilayer perceptron](https://en.wikipedia.org/wiki/Multilayer_perceptron) to rate jokes from the [Jester dataset](https://eigentaste.berkeley.edu/dataset/) using [BERT](https://en.wikipedia.org/wiki/BERT_(language_model)) word embeddings.
