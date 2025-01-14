# rust-pangenome-graph-summarize
 
 - rust pangenome graph summarize.
 - it writes the pangenome graph fasta also.
 - Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.


 ```
 cargo build 

 ```

 ```
 ╭─gauravsablok@fedora ~/Desktop/rust-pangenome-graph-summarize  ‹main*› 
 ╰─➤  ./target/debug/rust-pangenome-graph-summarize -h
 Usage: rust-pangenome-graph-summarize <GRAPH>

 Arguments:
  <GRAPH>  please provide the path to the graph file

 Options:
  -h, --help     Print help
  -V, --version  Print version
 
 ```

 ```
 ╭─gauravsablok@fedora ~/Desktop/rust-pangenome-graph-summarize  ‹main*› 
 ╰─➤  ./rust-pangenome-graph-summarize ./sample-file/sample-pangenome.gfa 
 Results have been written:
 number_segment:8
 number_links:11
 number_arc:22
 number_rank:2
 total_segment_length:17572
 average_segment_length:2196
 sum_0_segment_length:16569

 ```

 Gaurav Sablok
