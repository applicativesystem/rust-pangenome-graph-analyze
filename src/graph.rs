/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 *
 * */
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Segments {
    pub segment: String,
    pub id: String,
    pub seq: String,
    pub tag: String,
    pub aligntag: String,
    pub alignmenttag: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Links {
    pub link: String,
    pub id1: String,
    pub strand1: String,
    pub id2: String,
    pub strand2: String,
    pub tag: String,
    pub arc: String,
}

#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct FinalWrite {
    pub number_segment: usize,
    pub number_links: usize,
    pub number_arc: usize,
    pub max_rank: usize,
    pub total_segment_length: usize,
    pub average_segment_length: usize,
    pub sum_0_segment_length: usize,
}
