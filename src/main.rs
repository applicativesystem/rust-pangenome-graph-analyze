mod args;
mod graph;
use crate::args::GraphArgs;
use crate::graph::FinalWrite;
use crate::graph::Links;
use crate::graph::Segments;
use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam and SLB Potsdam
*Date 2025-1-13

 graph summarizer for the pangenome. Takes a graph alignment in the
 gfa format and then summarizes the graph format.

*/

fn main() {
    let args = GraphArgs::parse();
    let output = graph_args_segment(&args.graph).unwrap();
    for i in output.iter() {
        println!(
            "Results have been written:\nnumber_segment:{}\nnumber_links:{}\nnumber_arc:{}\nnumber_rank:{}\ntotal_segment_length:{}\naverage_segment_length:{}\nsum_0_segment_length:{}\n",
            i.number_segment,
            i.number_links,
            i.number_arc,
            i.max_rank,
            i.total_segment_length,
            i.average_segment_length,
            i.sum_0_segment_length
        );
    }
}

fn graph_args_segment(path: &str) -> Result<Vec<FinalWrite>, Box<dyn Error>> {
    let mut segment_counter: usize = 0usize;
    let mut links_counter: usize = 0usize;
    let mut graphreadcheck: Vec<String> = Vec::new();

    let graphopen = File::open(path).expect("file not found");
    let graphread = BufReader::new(graphopen);
    for i in graphread.lines() {
        let line = i.unwrap();
        if line.starts_with("S") {
            segment_counter += 1
        } else if line.starts_with("L") {
            links_counter += 1
        }
        graphreadcheck.push(line);
    }

    let mut segment_graph: Vec<Segments> = Vec::new();
    let mut link_graph: Vec<Links> = Vec::new();
    for i in graphreadcheck.iter() {
        if i.starts_with("S") {
            let line: Vec<_> = i.split("\t").filter(|x| !x.is_empty()).collect::<Vec<_>>();
            segment_graph.push(Segments {
                segment: line[0].to_string(),
                id: line[1].to_string(),
                seq: line[2].to_string(),
                tag: line[3].to_string(),
                aligntag: line[4].to_string(),
                alignmenttag: line[5].to_string(),
            });
        } else if i.starts_with("L") {
            let line: Vec<_> = i.split("\t").filter(|x| !x.is_empty()).collect::<Vec<_>>();
            link_graph.push(Links {
                link: line[0].to_string(),
                id1: line[1].to_string(),
                strand1: line[2].to_string(),
                id2: line[3].to_string(),
                strand2: line[4].to_string(),
                tag: line[5].to_string(),
                arc: line[6].to_string(),
            });
        }
    }

    // making the max rank

    let mut maxranklast: Vec<usize> = Vec::new();
    for i in link_graph.iter() {
        let maxiter: Vec<_> = i.arc.split(":").collect::<Vec<_>>();
        maxranklast.push(maxiter[2].parse::<usize>().unwrap());
    }
    let maxiter: HashSet<usize> = maxranklast.into_iter().collect::<HashSet<usize>>();
    let maxranknumber: usize = maxiter.len();

    // segment summary

    let mut segment_summary: Vec<String> = Vec::new();
    let mut segment_finallength: Vec<usize> = Vec::new();
    for i in segment_graph.iter() {
        segment_summary.push(i.seq.clone());
        segment_finallength.push(i.seq.len());
    }
    let finalsegmentlength: usize = segment_finallength.iter().sum();

    // sum_0_rank_segment

    let mut rank_segment: Vec<(String, usize)> = Vec::new();
    for i in segment_graph.iter() {
        let segmentholdseq: String = i.seq.clone();
        let segmentholdtag: Vec<_> = i.alignmenttag.split(":").collect::<Vec<_>>();
        if segmentholdtag[2].parse::<usize>().unwrap() == 0usize {
            rank_segment.push((segmentholdseq.clone(), segmentholdseq.len()));
        }
    }

    let mut rank_counter: usize = 0usize;
    for i in rank_segment.iter() {
        rank_counter += i.1;
    }

    // arc number calculation

    let mut arc_capture: Vec<String> = Vec::new();
    for i in link_graph.iter() {
        arc_capture.push(i.id1.clone());
        arc_capture.push(i.id2.clone());
    }

    let arc_number: usize = arc_capture.len();

    // final write for the graph.

    let mut finalwrite: Vec<FinalWrite> = vec![];
    finalwrite.push(FinalWrite {
        number_segment: segment_counter,
        number_links: links_counter,
        number_arc: arc_number,
        max_rank: maxranknumber,
        total_segment_length: finalsegmentlength,
        average_segment_length: finalsegmentlength / segment_finallength.len(),
        sum_0_segment_length: rank_counter,
    });

    let mut pangenome_summarize_write =
        File::create("summarized_pangenome.txt").expect("file not found");
    for i in finalwrite.iter() {
        writeln!( pangenome_summarize_write,
            "Results have been written:\nnumber_segment:{}\nnumber_links:{}\nnumber_arc:{}\nnumber_rank:{}\ntotal_segment_length:{}\naverage_segment_length:{}\nsum_0_segment_length:{}\n",
            i.number_segment,
            i.number_links,
            i.number_arc,
            i.max_rank,
            i.total_segment_length,
            i.average_segment_length,
            i.sum_0_segment_length
        ).expect("line not present");
    }

    let mut graphwrite = File::create("graph.fasta").expect("file not found");
    for i in segment_graph.iter(){
       writeln!(graphwrite, ">{}\n{}", i.id, i.seq).expect("line not present");
    }

    Ok(finalwrite)
}
