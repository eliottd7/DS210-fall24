DS210 Final Project by Eliott Dinfotan

# Neighboring Nodes with Minimum Other Neighbors #

## Concept ##

Say there are nodes on a weighted, undirected graph.
The task is to figure out which node on the graph has the fewest nodes within a certain distance.

However, the caveat is that the nodes U and V must be within a maximum threshold by the sum of the weights along a path from U to V.

For example, let's take the very simple graph with edges (U, V, weight):
- A, B, 3
- B, C, 1
- B, D, 4
- C, D, 1

If we set the maximum "distance" to 4, we can observe that:
- A has 2 neighbors, B and C
- B has 3 neighbors
- C also has 3 neighbors
- D has 2 neighbors, B and C

Therefore, we can say either A or D has 2 neighboring cities.

The idea came from [here](https://www.geeksforgeeks.org/problems/city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/1?page=1&category%5B%5D=Disjoint+Set&sortBy=submissions) but my version is modified to be applied to a much larger datset... also I'm not really following their algorithm verbatim.

## Implementation ##

In my implementation, I use Depth First Search to traverse the graph. The function `dfs_maxdepth_minneighbors()` returns the node
that it encountered within the maximum allowed distance, which has the fewest neighbors.

To prevent cycles, I use a HashSet to track previously visited nodes.

The GeoPoint structure is a custom struct with positional data (x, y), and a list of weighted connections represented as two aligned
vectors indicating the ID of the connected node, and the weight of the connection. The graph is capable of being fully represented by
the GeoPoint alone.

The WBDGraph (or Weighted BiDirectional Graph) is a utility wrapper struct for GeoPoint graphs. This is where I put the implementation
for loading the CSV data and constructing the graph, as well as the DFS algorithm.

I added basic threading for a supposed performance boost. I found it provides only minimal benefit.

## Data set ##

I make use of the Twitch Gamers dataset. This data has 168k nodes and 1.05 million edges.

(My choice of this dataset was originally intended for another idea around user networks).

The weight of a connection U, V is just the Euclidean distance. I had to implement a slightly odd version of the Pythagorean theorem
to prevent multiplication overflow, but have since then changed to `usize` instead of `u32` and since the code worked, I did not change it.

## Runtime Instructions ##

This program relies on the external crates Rand and CSV. Please be connected to the internet on the first-time compilation to download these
automatically by running `cargo build`.

You MUST have the Twitch Large dataset downloaded. For your convenience, the appropriate files are included within the Git repository.

*This code is NOT generic and will not work with another dataset.*
You have the option to specify your own path to these datasets.
Run `cargo run path/to/twitch_large_features.csv path/to/twitch_large_edges.csv K` where the third argument K allows you to specify your own
distance cap for the DFS algorithm.
If you do NOT specify any arguments, the program will default to `./twitch_gamers/twitch_large_features.csv ./twitch_gamers/twitch_large_edges.csv 3000`.

Reducing the value of K does not necessarily speed up the algorithm.


## Expected Output ##

If you don't add arguments, the program will notify you of this and show a brief help.

The program will print out when it begins loading in a specific file.

Then, I run a single DFS iteration on Node 49, and the relevant statistics are displayed.

Finally, the program runs DFS iterations on 10 thousand random nodes, dumping the statistics
to a file in the working directory called `mnm.csv`. This is slow, about 10 minutes. I did my best to make it faster.

You can check on progress by running (on a Linux machine) `wc mnm.csv`; the first output number from wc is the number of lines in a file, which is out of 10k.

You may find the output a little odd if you run with a depth threshhold of 6000 or less; it seems that many of the origin nodes for DFS are also less connected. This is because the nodes with higher numbers of connections are also more likely to have higher distances and therefore exceed the threshold (see last paragraph). It is worth running this with a much higher threshold at the expense of waiting longer to see that, at depth values over 12000, the same nodes start appearing as the min-neighbor winners with less than 5 connections. On my laptop this took over an hour so I don't recommend unless you're that bored.

The Twitch Gamers dataset edges are follows between users. However, the nature of the platform itself lends that more followers implies more views, which is one of the fields used in calculating the weight of a node. So increased connectivity is highly correlated to increased weights along all edges.
