// ============================================================================
//  Phase 0 — C#-for-DSA Toolkit
// ----------------------------------------------------------------------------
//  Goal: never lose interview time looking up stdlib syntax. Each region is a
//  self-contained demo you can read, run, and eventually reproduce from memory.
//  Run all:           dotnet run
//  The point isn't to memorize this file — it's to do each idiom ONCE by hand
//  until the shape is automatic. The GOTCHA comments are the high-value bits.
// ============================================================================

using System.Buffers;            // ArrayPool (perf, rarely needed in interviews)
using System.Numerics;           // BitOperations
using System.Runtime.InteropServices; // CollectionsMarshal

internal static class Snippets
{
    private static void Main()
    {
        Run(nameof(ListBasics), ListBasics);
        Run(nameof(DictAndSet), DictAndSet);
        Run(nameof(OrderedStructures), OrderedStructures);
        Run(nameof(Heaps), Heaps);
        Run(nameof(StackQueueDeque), StackQueueDeque);
        Run(nameof(Sorting), Sorting);
        Run(nameof(Grids2D), Grids2D);
        Run(nameof(Strings), Strings);
        Run(nameof(TuplesDeconstruction), TuplesDeconstruction);
        Run(nameof(FastInputParsing), FastInputParsing);
        Run(nameof(OverflowAndNumbers), OverflowAndNumbers);
        Run(nameof(BitTricks), BitTricks);
    }

    // ---- 1. List<T> : the default dynamic array --------------------------
    private static void ListBasics()
    {
        var list = new List<int>(capacity: 16);   // pre-size if you know N → avoids regrowth
        list.AddRange([5, 3, 8, 1]);               // collection expression (C# 12)
        list.Add(9);
        list.Insert(0, 0);                          // O(n) — shifts; avoid in hot loops
        list.RemoveAt(list.Count - 1);              // O(1) at the end, O(n) elsewhere

        // Span over a List<T> for zero-copy slicing (read/scan hot paths):
        Span<int> span = CollectionsMarshal.AsSpan(list);
        Console.WriteLine($"list={string.Join(",", list)}  first2={span[..2].ToArray()[0]},{span[..2].ToArray()[1]}");

        // 2D-ish: list of lists (jagged, growable)
        var rows = new List<List<int>> { new() { 1, 2 }, new() { 3 } };
        Console.WriteLine($"rows[0][1]={rows[0][1]}  rowCount={rows.Count}");
    }

    // ---- 2. Dictionary<K,V> + HashSet<T> : O(1) lookup -------------------
    private static void DictAndSet()
    {
        // Frequency map — the single most common interview structure.
        var freq = new Dictionary<char, int>();
        foreach (char c in "mississippi")
        {
            // GOTCHA: there is no auto-default. Three idioms, fastest last:
            freq[c] = freq.GetValueOrDefault(c) + 1;        // clean, slightly slower (double lookup)
        }
        Console.WriteLine($"freq[s]={freq['s']}  freq[z(absent)]={freq.GetValueOrDefault('z')}");

        // TryGetValue avoids the double lookup of ContainsKey + indexer:
        if (freq.TryGetValue('p', out int p)) Console.WriteLine($"p appears {p}x");

        // FASTEST count pattern — ref into the bucket, increments in place, ONE hash:
        var fast = new Dictionary<char, int>();
        foreach (char c in "banana")
            CollectionsMarshal.GetValueRefOrAddDefault(fast, c, out _)++;
        Console.WriteLine($"fast[a]={fast['a']}");

        // HashSet: membership + dedup. Add returns false if already present.
        var seen = new HashSet<int>();
        int[] nums = [1, 2, 2, 3, 3, 3];
        int firstDup = nums.First(n => !seen.Add(n));   // !Add == "was already there"
        Console.WriteLine($"first duplicate = {firstDup}");
    }

    // ---- 3. Ordered structures : O(log n) sorted views -------------------
    private static void OrderedStructures()
    {
        // SortedSet<T>: ordered, unique. Red-black tree. GetViewBetween for ranges.
        var ss = new SortedSet<int> { 5, 1, 9, 1, 3 };          // dup 1 ignored
        Console.WriteLine($"SortedSet min={ss.Min} max={ss.Max} -> {string.Join(",", ss)}");

        // SortedDictionary<K,V>: ordered map, O(log n) insert/lookup.
        var sd = new SortedDictionary<int, string> { [3] = "c", [1] = "a", [2] = "b" };
        Console.WriteLine($"SortedDict in key order: {string.Join(",", sd.Values)}");

        // GOTCHA: SortedList<K,V> is array-backed — O(log n) lookup but O(n) insert;
        // great for build-once-then-read, bad for many mid-inserts. Prefer
        // SortedDictionary when you insert/remove a lot.
        // NEITHER gives you a true "floor/ceiling in O(log n)" as ergonomically as
        // Java's TreeMap — for that, SortedSet.GetViewBetween is your closest tool:
        var view = ss.GetViewBetween(2, 8);                      // {3,5}
        Console.WriteLine($"view[2..8] = {string.Join(",", view)}");
    }

    // ---- 4. PriorityQueue<TElement,TPriority> : the heap -----------------
    private static void Heaps()
    {
        // Default is a MIN-heap: lowest priority dequeues first.
        var minHeap = new PriorityQueue<string, int>();
        minHeap.Enqueue("task-low", 5);
        minHeap.Enqueue("task-high", 1);
        Console.WriteLine($"min-heap pops: {minHeap.Dequeue()}");   // task-high (priority 1)

        // MAX-heap trick #1: negate the priority.
        var maxHeap = new PriorityQueue<int, int>();
        foreach (int x in new[] { 4, 9, 2 }) maxHeap.Enqueue(x, -x);
        Console.WriteLine($"max-heap pop: {maxHeap.Dequeue()}");    // 9

        // MAX-heap trick #2: a reversed comparer (cleaner when priority == element).
        var maxByComparer = new PriorityQueue<int, int>(Comparer<int>.Create((a, b) => b - a));
        foreach (int x in new[] { 4, 9, 2 }) maxByComparer.Enqueue(x, x);
        Console.WriteLine($"max-heap (comparer) pop: {maxByComparer.Dequeue()}");

        // "Top-K largest" pattern: keep a MIN-heap of size k, evict the smallest.
        int k = 2; int[] data = [3, 1, 7, 5, 9, 2];
        var topK = new PriorityQueue<int, int>();
        foreach (int x in data)
        {
            topK.Enqueue(x, x);
            if (topK.Count > k) topK.Dequeue();      // drop current smallest
        }
        Console.WriteLine($"top-{k} largest = {string.Join(",", topK.UnorderedItems.Select(i => i.Element).OrderByDescending(v => v))}");

        // EnqueueDequeue: push then pop the min in one op (great for the size-k pattern).
        // GOTCHAs: (1) NOT stable — equal priorities pop in arbitrary order.
        //          (2) No decrease-key. For Dijkstra, push duplicates and skip stale
        //              ones with an "if (d > dist[u]) continue;" guard.
    }

    // ---- 5. Stack / Queue / Deque ----------------------------------------
    private static void StackQueueDeque()
    {
        var stack = new Stack<int>();                 // LIFO — monotonic-stack problems
        stack.Push(1); stack.Push(2);
        Console.WriteLine($"stack peek={stack.Peek()} pop={stack.Pop()}");

        var queue = new Queue<int>();                 // FIFO — BFS
        queue.Enqueue(1); queue.Enqueue(2);
        Console.WriteLine($"queue peek={queue.Peek()} dequeue={queue.Dequeue()}");

        // Deque: C# has no built-in ArrayDeque. Use LinkedList<T> for O(1) both ends
        // (sliding-window-maximum monotonic deque). Slight node overhead, fine for interviews.
        var deque = new LinkedList<int>();
        deque.AddLast(1); deque.AddFirst(0); deque.AddLast(2);
        deque.RemoveFirst(); deque.RemoveLast();
        Console.WriteLine($"deque after ops -> {string.Join(",", deque)}  (front={deque.First!.Value})");
    }

    // ---- 6. Sorting + custom comparers -----------------------------------
    private static void Sorting()
    {
        int[] arr = [5, 2, 8, 1];
        Array.Sort(arr);                              // in place, ascending, NOT stable
        Console.WriteLine($"asc: {string.Join(",", arr)}");

        // Descending: sort then reverse, or a comparison delegate.
        Array.Sort(arr, (a, b) => b.CompareTo(a));    // use CompareTo, NOT b-a (b-a overflows!)
        Console.WriteLine($"desc: {string.Join(",", arr)}");

        // Multi-key sort: by length, then alphabetical. List.Sort with a comparison.
        var words = new List<string> { "bb", "a", "ccc", "aa" };
        words.Sort((x, y) =>
        {
            int byLen = x.Length.CompareTo(y.Length);
            return byLen != 0 ? byLen : string.Compare(x, y, StringComparison.Ordinal);
        });
        Console.WriteLine($"multi-key: {string.Join(",", words)}");

        // LINQ OrderBy is STABLE and reads clean — fine when you can afford the alloc:
        var byLenThenAlpha = words.OrderBy(w => w.Length).ThenBy(w => w, StringComparer.Ordinal);
        Console.WriteLine($"linq stable: {string.Join(",", byLenThenAlpha)}");

        // Sorting tuples sorts lexicographically (Item1 then Item2) for free:
        var pts = new List<(int x, int y)> { (1, 9), (1, 2), (0, 5) };
        pts.Sort();
        Console.WriteLine($"tuple sort: {string.Join(" ", pts)}");
    }

    // ---- 7. 2D structures : int[,] vs int[][] ----------------------------
    private static void Grids2D()
    {
        // Rectangular array int[,]: single block, fast, fixed rectangle.
        int[,] grid = new int[3, 4];                  // all zero
        grid[1, 2] = 7;
        Console.WriteLine($"rect grid dims = {grid.GetLength(0)}x{grid.GetLength(1)}, [1,2]={grid[1, 2]}");

        // Jagged array int[][]: array of arrays; rows can differ; this is what
        // LeetCode hands you (int[][]). Indexing is grid[r][c].
        int rows = 3, cols = 4;
        int[][] jagged = new int[rows][];
        for (int r = 0; r < rows; r++) jagged[r] = new int[cols];   // MUST allocate each row
        jagged[2][3] = 5;
        Console.WriteLine($"jagged[2][3]={jagged[2][3]}");

        // bool[][] visited + the 4-direction pattern (the BFS/DFS workhorse):
        var visited = new bool[rows][];
        for (int r = 0; r < rows; r++) visited[r] = new bool[cols];
        int[][] dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];          // down,up,right,left
        int sr = 0, sc = 0; int neighbors = 0;
        foreach (var d in dirs)
        {
            int nr = sr + d[0], nc = sc + d[1];
            if (nr >= 0 && nr < rows && nc >= 0 && nc < cols) neighbors++;   // in-bounds check
        }
        Console.WriteLine($"in-bounds neighbors of (0,0) = {neighbors}");
    }

    // ---- 8. Strings ------------------------------------------------------
    private static void Strings()
    {
        // GOTCHA: string is immutable. Concatenating in a loop is O(n^2). Use StringBuilder.
        var sb = new System.Text.StringBuilder();
        for (int i = 0; i < 5; i++) sb.Append((char)('a' + i));
        Console.WriteLine($"built: {sb}");                          // abcde

        // char arithmetic — the alphabet-index trick (lowercase a..z):
        char ch = 'd';
        int idx = ch - 'a';                                         // 3
        char back = (char)('a' + idx);
        Console.WriteLine($"'{ch}' -> idx {idx} -> '{back}'");

        // A 26-length int[] is the fastest "char frequency" map for a..z:
        int[] count = new int[26];
        foreach (char c in "leetcode") count[c - 'a']++;
        Console.WriteLine($"count['e']={count['e' - 'a']}");

        // Span<char> parsing — split-free, zero-alloc scanning of "12 34 56":
        string s = "12 34 56";
        int sum = 0;
        foreach (Range r in SplitOnSpace(s)) sum += int.Parse(s.AsSpan(r.Start.Value, r.End.Value - r.Start.Value));
        Console.WriteLine($"span-parsed sum = {sum}");

        // Common helpers you should reach for without thinking:
        Console.WriteLine($"reverse: {new string("abc".Reverse().ToArray())}  " +
                          $"isDigit('7')={char.IsDigit('7')}  toLower('X')={char.ToLower('X')}");
    }

    // helper: yield word ranges in a span without allocating substrings
    private static IEnumerable<Range> SplitOnSpace(string s)
    {
        int start = 0;
        for (int i = 0; i <= s.Length; i++)
            if (i == s.Length || s[i] == ' ')
            {
                if (i > start) yield return start..i;
                start = i + 1;
            }
    }

    // ---- 9. Tuples + deconstruction --------------------------------------
    private static void TuplesDeconstruction()
    {
        (int lo, int hi) Range() => (3, 9);            // return multiple values, no class needed
        var (lo, hi) = Range();
        Console.WriteLine($"lo={lo} hi={hi}");

        // Swap with no temp:
        int a = 1, b = 2;
        (a, b) = (b, a);
        Console.WriteLine($"swapped a={a} b={b}");

        // Tuple as a dictionary key (memoization on (i, j) state — common in DP):
        var memo = new Dictionary<(int, int), long>();
        memo[(1, 2)] = 42;
        Console.WriteLine($"memo[(1,2)]={memo[(1, 2)]}  contains(2,1)={memo.ContainsKey((2, 1))}");
    }

    // ---- 10. Fast input parsing (programming-exam stdin) -----------------
    private static void FastInputParsing()
    {
        // On judges (Codeforces/HackerRank) Console.ReadLine + Split is usually enough.
        // For heavy input, a buffered reader + manual int parse avoids Split allocations.
        string sampleLine = "10 20 30 40";
        int[] parsed = Array.ConvertAll(sampleLine.Split(' '), int.Parse);
        Console.WriteLine($"parsed line sum = {parsed.Sum()}");

        // Zero-alloc variant over a span (what you'd use if input is the bottleneck):
        int total = 0;
        foreach (Range r in SplitOnSpace(sampleLine)) total += int.Parse(sampleLine.AsSpan(r.Start.Value, r.End.Value - r.Start.Value));
        Console.WriteLine($"span sum = {total}");

        // For real stdin (commented so this stays runnable without piped input):
        // using var reader = new StreamReader(Console.OpenStandardInput());
        // int n = int.Parse(reader.ReadLine()!);
    }

    // ---- 11. Overflow + numeric gotchas ----------------------------------
    private static void OverflowAndNumbers()
    {
        // GOTCHA: int is 32-bit. Sums/products of constraints up to 1e9 overflow silently.
        int big = 2_000_000_000;
        long safe = (long)big + big;                  // cast BEFORE adding, not after
        Console.WriteLine($"int+int wraps={big + big}  long-safe={safe}");

        // Midpoint without overflow (binary search!): lo + (hi - lo) / 2, not (lo+hi)/2.
        int lo = 2_000_000_000, hi = 2_000_000_100;
        int mid = lo + (hi - lo) / 2;
        Console.WriteLine($"safe mid = {mid}");

        // Sentinels:
        Console.WriteLine($"int.MaxValue={int.MaxValue}  long.MinValue={long.MinValue}");

        // Floor/ceil integer division for positives:
        int ceilDiv = (7 + 3 - 1) / 3;                // ceil(7/3) = 3
        Console.WriteLine($"ceil(7/3) = {ceilDiv}");

        // Math.DivRem returns quotient + remainder together:
        (int q, int rem) = Math.DivRem(17, 5);
        Console.WriteLine($"17/5 -> q={q} r={rem}");
    }

    // ---- 12. Bit tricks (Phase 10 preview, but keep handy) ---------------
    private static void BitTricks()
    {
        int x = 0b1011010;
        Console.WriteLine($"popcount={int.PopCount(x)} " +              // set bits (.NET 7+)
                          $"trailingZeros={int.TrailingZeroCount(x)} " +
                          $"leadingZeros={int.LeadingZeroCount(x)}");

        Console.WriteLine($"isPowerOf2(64)={(64 & 63) == 0}");          // n & (n-1) == 0
        Console.WriteLine($"lowestSetBit={x & -x}");                    // isolate lowest 1-bit
        int with3set = x | (1 << 3);                                    // set bit 3
        int without3 = x & ~(1 << 3);                                   // clear bit 3
        bool bit3 = (x & (1 << 3)) != 0;                                // test bit 3
        Console.WriteLine($"set/clear/test bit3 -> {with3set},{without3},{bit3}");

        // Iterate all subsets of a bitmask (bitmask DP, Phase 8.7):
        int mask = 0b101; var subs = new List<int>();
        for (int sub = mask; sub > 0; sub = (sub - 1) & mask) subs.Add(sub);
        Console.WriteLine($"subsets of {Convert.ToString(mask, 2)}: {string.Join(",", subs.Select(s => Convert.ToString(s, 2)))}");

        // BitOperations also exists for uint paths:
        Console.WriteLine($"BitOps.PopCount(255u)={BitOperations.PopCount(255u)}");
    }

    // ---- harness ---------------------------------------------------------
    private static void Run(string name, Action demo)
    {
        Console.WriteLine($"\n=== {name} ===");
        demo();
    }
}
