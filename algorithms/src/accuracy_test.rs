use std::collections::HashMap;
use std::collections::HashSet;
///returns (false_positives:usize, false_negatives:usize)
pub fn accuracy_test(sample_results:&(HashMap<u32,u32>,HashMap<(u32,u32),u32>),good_results:&(HashMap<u32,u32>,HashMap<(u32,u32),u32>))-> (usize,usize){
	//take keys out of results and put them in a set
	let sample_single_keys:HashSet<u32> = sample_results.0.keys().cloned().collect();
	let sample_pair_keys:HashSet<(u32,u32)> = sample_results.1.keys().cloned().collect();
	let good_single_keys:HashSet<u32> = good_results.0.keys().cloned().collect();
	let good_pair_keys:HashSet<(u32,u32)> = good_results.1.keys().cloned().collect();

	let false_negatives = good_single_keys.difference(&sample_single_keys).count() + good_pair_keys.difference(&sample_pair_keys).count();
	let false_positives = sample_single_keys.difference(&good_single_keys).count() + sample_pair_keys.difference(&good_pair_keys).count();

	return (false_positives, false_negatives);
}