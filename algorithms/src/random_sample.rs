use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashMap;

pub fn random_sample(data:&[Vec<u32>],sample_proportion:f32,support_threshold:u32, confidence_threshold:f32)-> (HashMap<u32,u32>,HashMap<(u32,u32),u32>){
	let sample_size = (data.len() as f32 * sample_proportion).floor() as usize;
	let mut rng = thread_rng();
	let sample = data.iter().choose_multiple(&mut rng, sample_size);
	return apriori(sample, support_threshold, confidence_threshold*sample_proportion)
}

pub fn apriori(data:Vec<&Vec<u32>>,support_threshold:u32, confidence_threshold:f32)-> (HashMap<u32,u32>,HashMap<(u32,u32),u32>){
	// counts is a map where the key is a set of values, and the value is the count of the previously mentored values
	let mut single_counts:HashMap<u32,u32> = HashMap::new();

	//pass 1
	for basket in &data{
		for item in *basket{
			// add 1 to key cound, or if it does not exists yet set it to 1
			single_counts.entry(*item).and_modify(|e|{*e += 1}).or_insert(1);
		}
	}

	//prune
	{
		let mut condemed_keys = Vec::new();
		for (key,val) in &single_counts{
			if *val < support_threshold{
				condemed_keys.push(key.clone());
			}
		}
		for key in &condemed_keys{
			single_counts.remove(&key);
		}
	}


	//pass 2
	let mut pair_counts:HashMap<(u32,u32),u32> = HashMap::new();
	for basket in &data{
		for i in 0..basket.len(){
			for j in i+1 .. basket.len(){
				let (item1, item2) = (basket[i], basket[j]);
				if item1 != item2{
					if single_counts.contains_key(&item1) && single_counts.contains_key(&item2){
						let key:(u32,u32);
						if item1 < item2{
							key = (item1,item2);
						}else{
							key = (item2,item1);
						}
						pair_counts.entry(key).and_modify(|e|{*e += 1}).or_insert(1);
					}
				}
			}
		}
	}
	//prune
	let mut condemed_keys = Vec::new();
	for (key,val) in &pair_counts{
		if *val < support_threshold{
			condemed_keys.push(key.clone());
		}else{
				let (key1,key2) = key;
				if(*pair_counts.get(key).unwrap() as f32/ *single_counts.get(key1).unwrap() as f32) < confidence_threshold{
					condemed_keys.push(key.clone());
				}else if(*pair_counts.get(key).unwrap() as f32/ *single_counts.get(key2).unwrap() as f32) < confidence_threshold{
					condemed_keys.push(key.clone());
			}	
		}
	}
	for key in &condemed_keys{
		pair_counts.remove(&key);
	}


	return (single_counts, pair_counts);
}