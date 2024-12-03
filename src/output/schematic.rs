use std::{cmp::{max, min}, error::Error};

use mc_schem::{block::Block, region::Region, schem::Schematic, WorldEdit13SaveOption};
use vecmath::{vec3_add, vec3_mul, vec3_sub};


pub struct WriteConfig {
	pub step: [i32; 3],
	pub order: [usize; 3],
	pub size: [i32; 3],
	pub offset_callback: &'static dyn Fn(&[i32; 3]) -> [i32; 3],
	pub world_offset: [i32; 3],
}

impl WriteConfig {
	pub fn default() -> WriteConfig {
		WriteConfig {
			step: [-2, 2, 5],
			order: [0, 2, 1],
			size: [32, 16, 12],
			world_offset: [0, 0, 0],
			offset_callback: &|&[x, y, z]: &[i32; 3]| [x, y / 6 * 2, z],
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

fn get_blocks() -> [Block; 2] {
	let block_air = Block::from_id("minecraft:air").unwrap();
	let mut block_torch = Block::from_id("minecraft:torch").unwrap();

	block_torch.attributes.insert("facing".into(), "east".into());
	block_torch.attributes.insert("lit".into(), "false".into());

	return [block_air, block_torch];
}

pub fn write(binary: &[i32], filename: &str, cfg: &WriteConfig) -> Result<(), Box<dyn Error>> {
	let mut max_pos: [i32; 3] = [0; 3];
	let mut min_pos: [i32; 3] = [0; 3];
	let word_count = cfg.size[0] * cfg.size[1];
	let blocks = get_blocks();

	for i in 0..word_count {
		for j in 0..cfg.size[2] {
			let pos = cfg.get_at(i, j);
			for k in 0..max_pos.len() {
				max_pos[k] = max(max_pos[k], pos[k]);
				min_pos[k] = min(min_pos[k], pos[k]);
			}
		}
	}

	let mut region = Region::with_shape(vec3_add(vec3_sub(max_pos, min_pos), [1, 1, 1]));

	for i in 0..word_count {
		let &word = binary.get(i as usize).unwrap_or(&0);
		for j in 0..cfg.size[2] {
			let pos = vec3_sub(cfg.get_at(i, j), min_pos);
			region.set_block(pos, match word & j {
				0 => &blocks[0],
				_ => &blocks[1],
			}).unwrap();
		}
	}

	let mut schem = Schematic::new();
	schem.regions.push(region);
	schem.metadata.schem_offset = vec3_sub(min_pos, cfg.world_offset);
	schem.save_world_edit_13_file(filename, &WorldEdit13SaveOption::default())?;

	return Ok(());
}

