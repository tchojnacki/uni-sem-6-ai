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
  - [Tabu Search](https://en.wikipedia.org/wiki/Tabu_search)
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
