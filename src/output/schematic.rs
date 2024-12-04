use std::{cmp::{max, min}, error::Error};

use mc_schem::{block::Block, region::Region, schem::Schematic, WorldEdit13SaveOption};
use vecmath::{vec3_add, vec3_mul, vec3_sub};


pub struct WriteConfig {
	pub step: [i32; 3],
	pub order: [usize; 3],
	pub size: [i32; 3],
	pub offset_callback: &'static dyn Fn(&[i32; 3]) -> [i32; 3],
}

impl WriteConfig {
	pub fn default() -> WriteConfig {
		WriteConfig {
			step: [-2, 2, 5],
			order: [0, 2, 1],
			size: [32, 16, 12],
			offset_callback: &|&[_, y, _]: &[i32; 3]| [0, y / 12 * 2, 0],
		}
	}
	fn get_at(&self, i: i32, bit: i32) -> [i32; 3] {
		let mut pos = [0; 3];
		let row = self.size[0];
		pos[self.order[0]] = i % row;
		pos[self.order[1]] = i / row;
		pos[self.order[2]] = bit;

		pos = vec3_mul(pos, self.step);
		pos = vec3_add(pos, (self.offset_callback)(&pos));

		return pos;
	}
}

fn get_support_blocks() -> [Block; 2] {
	let block_0 = Block::from_id("minecraft:black_wool").unwrap();
	let block_1 = Block::from_id("minecraft:white_wool").unwrap();

	return [block_0, block_1];
}

fn get_bit_blocks() -> [Block; 2] {
	let block_0 = Block::from_id("minecraft:glass").unwrap();
	let mut block_1 = Block::from_id("minecraft:redstone_wall_torch").unwrap();

	block_1.attributes.insert("facing".into(), "west".into());
	block_1.attributes.insert("lit".into(), "false".into());

	return [block_0, block_1];
}

pub fn write(binary: &[i32], filename: &str, cfg: &WriteConfig) -> Result<(), Box<dyn Error>> {
	let mut max_pos: [i32; 3] = [0; 3];
	let mut min_pos: [i32; 3] = [0; 3];
	let word_count = cfg.size[0] * cfg.size[1];
	let bit_blocks = get_bit_blocks();
	let support_blocks = get_support_blocks();

	for i in 0..word_count {
		for j in 0..cfg.size[2] {
			let pos = cfg.get_at(i, j);
			for k in 0..max_pos.len() {
				max_pos[k] = max(max_pos[k], pos[k]);
				min_pos[k] = min(min_pos[k], pos[k]);
			}
		}
	}

	let mut region = Region::with_shape(vec3_add(vec3_sub(max_pos, min_pos), [2, 1, 1]));
	region.fill_with(&Block::structure_void());

	for i in 0..word_count {
		let &word = binary.get(i as usize).unwrap_or(&0);
		for j in 0..cfg.size[2] {
			let pos = vec3_sub(cfg.get_at(i, j), min_pos);
			let v = match word & (1 << j) {
				0 => 0,
				_ => 1,
			};
			region.set_block(pos, &bit_blocks[v]).unwrap();
			region.set_block(vec3_add(pos, [1, 0, 0]), &support_blocks[v]).unwrap();
		}
	}
	
	let mut schem = Schematic::new();
	schem.regions.push(region);
	schem.save_world_edit_13_file(filename, &WorldEdit13SaveOption::default())?;

	return Ok(());
}

