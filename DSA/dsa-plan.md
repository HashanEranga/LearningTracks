# C# DSA & Algorithms — Interview Prep Plan

> **Language:** C# (.NET 8+). It's the language you think in fastest — keep all working memory on the algorithm, not the syntax.
> **Goal:** Be able to solve any LeetCode Medium reliably and most Hards, including full DP coverage and advanced topics, for coding interviews and programming exams.
> **Structure:** Pattern-based, not problem-count-based. You "finish" a phase when you can recognize the pattern on sight and code the template from memory — not when you've done N problems.

---

## How to use this plan

1. **Phases are ordered by dependency, not by difficulty.** Don't jump to DP (Phase 8) before Phase 3 (recursion/trees).
2. **Per topic, do this loop:**
   - Learn the concept + the C# idiom (≤30 min).
   - Solve 2–3 *Easy/Medium* "template" problems slowly, by hand first.
   - Solve 4–6 *Medium* until the pattern is automatic.
   - Solve 1–2 *Hard* to stress-test.
   - Write the **reusable template** into `templates/` (see below).
3. **Spaced repetition:** every problem you struggled on goes into `review.md` with a re-do date (+3 days, +1 week, +3 weeks). Re-doing beats new volume.
4. **Timed mode** from Phase 4 onward: Medium in ≤25 min, Hard in ≤45 min. If you blow the clock, it's a review item.

---

## Phase 0 — C#-for-DSA Toolkit (warm-up, ~1 week)

Master the stdlib so you never lose time looking things up mid-interview.

| Need | C# tool |
|------|---------|
| Dynamic array | `List<T>` |
| Hash map / set | `Dictionary<K,V>`, `HashSet<T>` |
| Ordered map / set | `SortedDictionary<K,V>`, `SortedSet<T>` (red-black tree, O(log n)) |
| Min/Max heap | `PriorityQueue<TElement,TPriority>` (.NET 6+) — invert priority or use `Comparer` for max-heap |
| Stack / Queue | `Stack<T>`, `Queue<T>` |
| Deque | `LinkedList<T>` or `List<T>` with index tricks; for monotonic deque use `LinkedList<T>` |
| Sorting | `Array.Sort`, `list.Sort((a,b)=>...)`, `OrderBy` (stable) |
| Char/string | `StringBuilder`, `string.AsSpan()`, `char.IsDigit`, etc. |
| Tuples | `(int, int)`, `var (a, b) = pair;` |

**Gotchas to internalize:** `PriorityQueue` is NOT stable and has no decrease-key; `Dictionary` ordering is undefined; `int` overflow → use `long` for sums/products; `string` is immutable → `StringBuilder` in loops.

**Deliverable:** a `Snippets.cs` with: max-heap, 2D grid init, `int[][]` vs `int[,]`, fast input parsing, `Comparer` examples.

---

## Phase 1 — Arrays, Strings & the Core Patterns

- **Two Pointers** — opposite ends, fast/slow, partitioning
- **Sliding Window** — fixed + variable size, "longest/shortest substring with X"
- **Prefix Sums / Difference Arrays** — range sums, subarray-sum-equals-k
- **Hashing patterns** — frequency maps, anagrams, grouping

Key Mediums: Longest Substring Without Repeating Chars, 3Sum, Container With Most Water, Subarray Sum Equals K, Product of Array Except Self, Group Anagrams, Longest Repeating Char Replacement.
Hards: Minimum Window Substring, Sliding Window Maximum, Substring with Concatenation of All Words.

---

## Phase 2 — Linked Lists, Stacks, Queues

- Linked list manipulation (reverse, merge, cycle detection — Floyd's), dummy-node pattern
- **Monotonic Stack** — next greater/smaller element, histogram
- Stack-based parsing / expression evaluation
- Queue / deque fundamentals

Key Mediums: Reverse Linked List II, LRU Cache (design), Daily Temperatures, Evaluate RPN, Min Stack, Reorder List.
Hards: Merge k Sorted Lists, Largest Rectangle in Histogram, Trapping Rain Water (stack variant).

---

## Phase 3 — Trees, BST & Recursion

- Traversals (pre/in/post + iterative with stack), **level-order BFS**
- Recursion mental model: "what does this function return for a node, given its children"
- BST properties, validation, k-th smallest, LCA
- Tree construction from traversals

Key Mediums: Validate BST, LCA of BST/Binary Tree, Kth Smallest in BST, Construct Tree from Preorder+Inorder, Right Side View, Count Good Nodes.
Hards: Binary Tree Maximum Path Sum, Serialize/Deserialize Binary Tree, Recover BST.

---

## Phase 4 — Graphs

- Representations (adjacency list/matrix), BFS/DFS templates
- **Topological sort** (Kahn's + DFS)
- **Union-Find / DSU** (with path compression + union by rank)
- Shortest paths: **Dijkstra** (with `PriorityQueue`), **Bellman-Ford**, **Floyd-Warshall**
- MST: **Kruskal** + **Prim**
- Grid-as-graph problems, multi-source BFS, 0-1 BFS

Key Mediums: Number of Islands, Course Schedule I/II, Clone Graph, Pacific Atlantic Water Flow, Rotting Oranges, Redundant Connection, Network Delay Time.
Hards: Word Ladder II, Alien Dictionary, Swim in Rising Water, Cheapest Flights Within K Stops (Bellman-Ford variant).

---

## Phase 5 — Searching & Sorting

- **Binary search** — on arrays AND **on the answer** (the high-value interview skill)
- Quickselect (k-th largest), merge sort intuition, counting sort
- Search in rotated/2D arrays

Key Mediums: Search in Rotated Sorted Array, Find Min in Rotated Array, Koko Eating Bananas, Capacity to Ship Packages, Kth Largest Element, Find Peak Element.
Hards: Median of Two Sorted Arrays, Split Array Largest Sum, Find K-th Smallest Pair Distance.

---

## Phase 6 — Backtracking

- Template: choose → explore → un-choose
- Subsets, permutations, combinations, combination sum
- Constraint problems (N-Queens, Sudoku, word search), pruning

Key Mediums: Subsets I/II, Permutations I/II, Combination Sum I/II, Word Search, Palindrome Partitioning, Letter Combinations.
Hards: N-Queens, Sudoku Solver, Word Search II (backtracking + Trie).

---

## Phase 7 — Greedy & Intervals

- Greedy exchange argument intuition (when greedy is provably correct)
- **Interval patterns** — sort + sweep, merge, scheduling
- Sweep line, heap-based scheduling

Key Mediums: Merge Intervals, Insert Interval, Non-overlapping Intervals, Meeting Rooms II, Jump Game I/II, Gas Station, Task Scheduler.
Hards: Employee Free Time, Minimum Number of Taps, Candy.

---

## Phase 8 — Dynamic Programming (the big one)

> DP is the highest-leverage and highest-failure topic. Spend the most time here. Always go **brute-force recursion → memoize → tabulate → space-optimize**, and learn to *recognize* which DP family a problem belongs to.

**8.1 1-D DP** — Climbing Stairs, House Robber I/II, Decode Ways, Word Break, Max Subarray (Kadane), Coin Change (min coins).

**8.2 Knapsack family** — 0/1 Knapsack, Subset Sum, Partition Equal Subset Sum, Target Sum, Coin Change II (count ways), Ones and Zeroes.

**8.3 Sequences** — **LIS** (O(n²) and O(n log n) patience), **LCS**, Edit Distance, Distinct Subsequences, Longest Palindromic Subsequence.

**8.4 2-D / Grid DP** — Unique Paths I/II, Minimum Path Sum, Maximal Square, Dungeon Game.

**8.5 Interval DP** — Matrix Chain, Burst Balloons, Stone Game variants, Min Cost to Cut a Stick.

**8.6 DP on Trees** — House Robber III, Binary Tree Cameras, Diameter via DP.

**8.7 Bitmask DP** — Travelling Salesman, Partition to K Equal Sum Subsets, Shortest Path Visiting All Nodes.

**8.8 DP on subsequences / strings, "state machine" DP** — Best Time to Buy/Sell Stock (all 6 variants — the canonical state-machine ladder), Regular Expression Matching, Wildcard Matching.

**8.9 Digit DP & Probability DP** (advanced/competitive) — Numbers With Repeated Digits, Knight Probability, Dice Roll Sum.

Hards to confirm mastery: Burst Balloons, Regular Expression Matching, Edit Distance, Best Time to Buy/Sell with Cooldown + fee, Frog Jump, Cherry Pickup I/II.

---

## Phase 9 — Advanced Data Structures & String Algorithms

- **Trie** (prefix tree) — autocomplete, Word Search II, word dictionaries
- **Heap, advanced** — top-k, merge-k, median from data stream (two-heap)
- **Segment Tree** + lazy propagation — range query/update
- **Fenwick Tree / BIT** — prefix sums with updates, inversion counting
- **String algorithms** — KMP, Z-algorithm, Rabin-Karp (rolling hash), Manacher's (longest palindrome)
- **Sparse Table** — static range-min/max (O(1) query)

Key problems: Implement Trie, Design Add and Search Words, Find Median from Data Stream, Range Sum Query Mutable (BIT/segtree), Count of Smaller Numbers After Self (BIT/merge sort), Shortest Palindrome (KMP), Longest Palindromic Substring (Manacher/expand).

---

## Phase 10 — Math, Bit Manipulation & Misc

- Bit tricks (XOR pairing, subsets via bits, count set bits, power-of-two)
- Number theory: GCD/LCM, sieve of Eratosthenes, modular exponentiation, modular inverse
- Combinatorics, Catalan numbers, fast power
- Matrix exponentiation (Fibonacci in O(log n))

Key problems: Single Number I/II/III, Sum of Two Integers, Counting Bits, Pow(x,n), Count Primes, Excel Column tricks, Random Pick with Weight.

---

## Phase 11 — Consolidation: Mocks & Spaced Repetition

- **Weekly timed set**: 1 Hard + 3 Mediums in 90 min, cold.
- Mock interviews (talk out loud): use [Pramp](https://www.pramp.com) (free peer mocks) or interviewing.io free tier.
- Re-solve everything in `review.md` that's due.
- **Company tagging**: in the final stretch, filter LeetCode by target company + sort by frequency.

---

## Free Resources (ranked)

**Curated problem sheets (follow one, don't sheet-hop):**
- **NeetCode 150 / 250** — neetcode.io — best pattern-grouped list with free C#-friendly video explanations. *Primary spine for Phases 1–8.*
- **Striver's A2Z DSA Sheet & SDE Sheet** — takeuforward.org — extremely thorough, free, great for DP and advanced.
- **Blind 75** — the minimum viable set if time-crunched.

**Practice judges (all free):**
- **LeetCode** — primary. Medium/Hard filters + company tags.
- **CSES Problem Set** — cses.fi/problemset — *the* best free set for DP + graph + advanced data structures (competitive flavor, sharpens Phase 8–9).
- **Codeforces** — for speed/contest pressure (Div 2 A–D).
- **HackerRank / Codewars** — extra reps, gentler.

**Theory & references:**
- **CP-Algorithms** — cp-algorithms.com — gold standard for segment trees, BIT, string algos, number theory (Phases 9–10).
- **William Fiset** (YouTube) — best free deep-dive on data structures & graph algorithms.
- **VisuAlgo** — visualgo.net — visualize every structure/algorithm.
- **USACO Guide** — usaco.guide — free, structured, advanced.
- *Reference book:* CLRS (theory) / "Competitive Programmer's Handbook" by Antti Laaksonen (free PDF, perfect companion to CSES).

---

## Repo layout suggestion

```
DSA/
  dsa-plan.md          (this file)
  progress.md          (track current phase + streak)
  review.md            (spaced-repetition queue, with re-do dates)
  templates/           (memorized templates: bfs.cs, dijkstra.cs, dp-knapsack.cs, ...)
  solutions/
    phase01-arrays/
    phase02-linked/
    ...
```

Keep each solution with a one-line "pattern" comment at the top so `templates/` can be regenerated by grepping.
