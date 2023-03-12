use std::collections::HashMap;
use std::collections::HashSet;
use crate::fast_apriori::apriori;

pub fn son(data:&[Vec<u32>],num_segments:usize,support_threshold:u32, confidence_threshold:f32)-> (HashMap<u32,u32>,HashMap<(u32,u32),u32>){
	let mut candidates_single : HashSet<u32> = HashSet::new();
	let mut candidates_pair : HashSet<(u32,u32)> = HashSet::new();

	let segment_size = data.len() / num_segments;

	//Pass 1
	for segment_index in 0..num_segments{
		let start_row = segment_index * segment_size;
		let last_row;
		if segment_index < num_segments - 1{
			// if were not on the last segment
			last_row = (segment_index + 1) * segment_size;
		}else{
			// if were on the last segment
			last_row = data.len();
		}
		let (results_single, results_pair) = apriori(&data[start_row..last_row], support_threshold/ num_segments as u32, confidence_threshold);
		candidates_single.extend(results_single.keys());
		candidates_pair.extend(results_pair.keys());
	}

	let mut frequent_singles:HashMap<u32,u32> = HashMap::new();
	let mut frequent_pairs:HashMap<(u32,u32),u32> = HashMap::new();

	//Pass 2
	for basket in data{
		for pair_itter_a in 0..basket.len(){
			let item_a = basket[pair_itter_a];
			if candidates_single.contains(&item_a){
				// add 1 to key count, or if it does not exists yet set it to 1
				frequent_singles.entry(item_a).and_modify(|e|{*e += 1}).or_insert(1);
				for pair_itter_b in pair_itter_a + 1 ..basket.len(){
					let item_b = basket[pair_itter_b];
					if item_a != item_b && candidates_single.contains(&item_b){
						let key:(u32,u32);
						if item_a < item_b{
							key = (item_a,item_b);
						}else{
							key = (item_a,item_b);
						}
						if candidates_pair.contains(&key){
							// add 1 to key count, or if it does not exists yet set it to 1
							frequent_pairs.entry(key).and_modify(|e|{*e += 1}).or_insert(1);
						}
					}
				}
			}
		}
	}
	//prune pairs
	let mut condemed_keys = Vec::new();
	for (key,val) in &frequent_pairs{
		if *val < support_threshold{
			condemed_keys.push(key.clone());
		}else{
				let (key1,key2) = key;
				if(*frequent_pairs.get(key).unwrap() as f32/ *frequent_singles.get(key1).unwrap() as f32) < confidence_threshold{
					condemed_keys.push(key.clone());
				}else if(*frequent_pairs.get(key).unwrap() as f32/ *frequent_singles.get(key2).unwrap() as f32) < confidence_threshold{
					condemed_keys.push(key.clone());
			}	
		}
	}
	for key in &condemed_keys{
		frequent_pairs.remove(&key);
	}
	//prune singles
	{
		let mut condemed_keys = Vec::new();
		for (key,val) in &frequent_singles{
			if *val < support_threshold{
				condemed_keys.push(key.clone());
			}
		}
		for key in &condemed_keys{
			frequent_singles.remove(&key);
		}
	}

	return (frequent_singles, frequent_pairs);

}