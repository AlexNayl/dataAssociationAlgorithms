use std::collections::HashMap;

fn hash1(val1:u32, val2:u32,n:usize)-> usize{
	(val1 ^ val2) as usize % n
}

fn hash2(val1:u32, val2:u32,n:usize)-> usize{
	(val1 ^ val2 ^ val1) as usize % n
}

pub fn pcy(data:&[Vec<u32>],max_i:usize,support_threshold:u32, confidence_threshold:f32)-> (HashMap<u32,u32>,HashMap<(u32,u32),u32>){
	
	let mut frequent_items_mask:Vec<bool> = vec![false;max_i+1];
	let mut map1_mask:Vec<bool> = vec![false;max_i+1];
	let mut single_counts: HashMap<u32,u32> = HashMap::new();
	{
		//code block forces memory dealloc after phase ends
		let mut frequent_items:Vec<u32> = vec![0;max_i+1];
		let mut map1:Vec<u32> =  vec![0;max_i+1];
		//pass 1
		for basket in data{
			for item in basket{
				// add 1 to key cound, or if it does not exists yet set it to 1
				frequent_items[*item as usize] += 1;
			}
			for iter_a in 0..basket.len(){
				for iter_b in iter_a + 1 .. basket.len(){
					let (item1, item2) = (basket[iter_a], basket[iter_b]);
					if item1 != item2{
						let key:(u32,u32);
						if item1 < item2{
								key = (item1,item2);
							}else{
								key = (item2,item1);
							}
						map1[hash1(key.0,key.1,max_i)] += 1;
					}
				}
			}
		}
		//prune
		for i in 0..max_i{
			if frequent_items[i] >= support_threshold{
				frequent_items_mask[i] = true;
				single_counts.insert(i as u32, frequent_items[i]);
			}
			if map1[i] >= support_threshold{
				map1_mask[i] = true;
			}
		}
	}//end code block, pass 1 specific vecs should dealloc here
	
	//pass2
	let mut map2_mask:Vec<bool> = vec![false; max_i + 1];
	{//codeblock forces local memory inside to be dealloced at end of block
		let mut map2:Vec<u32> = vec![0;max_i+1];

		for basket in data{
			for iter_a in 0..basket.len(){
				for iter_b in iter_a + 1 .. basket.len(){
					let (item1, item2) = (basket[iter_a], basket[iter_b]);
					if item1 != item2 && frequent_items_mask[item1 as usize] && frequent_items_mask[item2 as usize] && map1_mask[hash1(item1,item2,max_i)]{
						let key:(u32,u32);
						if item1 < item2{
							key = (item1,item2);
						}else{
							key = (item2,item1);
						}
						map2[hash2(key.0, key.1, max_i)] += 1;
					}
				}
			}
		}

		for i in 0..max_i{
			if map2[i] >= support_threshold{
				map2_mask[i] = true;
			}
		}
	}
	//pass3
	let mut pair_counts : HashMap<(u32,u32),u32> = HashMap::new();
	for basket in data{
		for iter_a in 0..basket.len(){
				for iter_b in iter_a + 1 .. basket.len(){
					let (item1, item2) = (basket[iter_a], basket[iter_b]);
				if item1 != item2 && frequent_items_mask[item1 as usize] && frequent_items_mask[item2 as usize] && map1_mask[hash1(item1,item2,max_i)] && map2_mask[hash2(item1,item2,max_i)]{
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