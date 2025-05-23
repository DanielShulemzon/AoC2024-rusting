fn main() {
    let input = include_str!("../../../input_files/day9_input");

    let mut result = first_part(input);
    println!("First part result is: {}", result);

    result = second_part(input);
    println!("Second part result is: {}", result);
}

#[derive(Debug)]
struct SlotData {
    index: usize,
    count: u8,
}

impl SlotData {
    fn from_vec_idx(arr: &[u8], index: usize) -> Self {
        SlotData {
            index,
            count: arr[index],
        }
    }
}

#[derive(Debug, Default)]
struct SumDataFirstPart {
    checksum: usize,
    block_idx: usize,
}

impl SumDataFirstPart {
    fn update_checksum(&mut self, og_box_id: usize, inc: u8) {
        for _ in 0..inc {
            self.checksum += og_box_id * self.block_idx;
            self.block_idx += 1;
        }
    }
}

fn first_part(input: &str) -> usize {
    let input = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let (mut empty, mut next) = {
        let last_full_idx = input.len() - 1 - ((input.len() - 1) % 2);
        (
            SlotData::from_vec_idx(&input, 1),
            SlotData::from_vec_idx(&input, last_full_idx),
        )
    };

    let mut i = 0;
    let mut sum_data = SumDataFirstPart::default();
    while empty.index < next.index {
        if i % 2 == 0 {
            // update checksum for found blocks.
            sum_data.update_checksum(i / 2, input[i]);
            i += 1;
        } else {
            // try fill the empty slots until all slots are full.
            if empty.count <= next.count {
                // if empty could be emptied at once
                // completely fill the empty slots, then care about result.
                sum_data.update_checksum(next.index / 2, empty.count);

                next.count -= empty.count;

                // update new states
                empty = SlotData::from_vec_idx(&input, empty.index + 2);
                if next.count == 0 {
                    // get both to their next position and break (run checks)
                    next = SlotData::from_vec_idx(&input, next.index - 2);
                }
                i += 1;
            } else {
                // 1 iteration is not enough..
                // do not increment i, update empty and next.
                sum_data.update_checksum(next.index / 2, next.count);

                // won't be zero because I said so.
                empty.count -= next.count;

                // update new states
                next = SlotData::from_vec_idx(&input, next.index - 2);
            }
        }
    }
    sum_data.checksum
}

// ------------------------------------------------------------------------------------------------------------------
// Second part stuff

#[derive(Debug, Default)]
struct SumDataSecondPart {
    checksum: usize,
}

impl SumDataSecondPart {
    fn update_checksum(
        &mut self,
        starting_block_index: usize,
        og_box_id: usize,
        inc: u8,
        offset: u8,
    ) {
        // let start_checksum = self.checksum;
        let start = starting_block_index + offset as usize;
        let end = start + inc as usize;
        for i in start..end {
            self.checksum += i * og_box_id;
        }
        // println!(
        //     "Added checksum of: {}, figure it out yourself..",
        //     self.checksum - start_checksum
        // );
    }
}
fn second_part(input: &str) -> usize {
    let input = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let mut empty_slots: Vec<Option<SlotData>> = input
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if i % 2 == 1 {
                Some(SlotData {
                    index: i,
                    count: *n,
                })
            } else {
                None
            }
        })
        .map(|s| if s.count == 0 { None } else { Some(s) })
        .collect();

    let mut sum_data = SumDataSecondPart::default();
    for i in (0..input.len()).rev().step_by(2) {
        if i % 2 == 1 {
            unreachable!();
        }
        let curr_slot = SlotData::from_vec_idx(&input, i);

        // find the first valid empty slot with small enough index.
        let target_empty_slot = get_available_slot(&mut empty_slots, &curr_slot);

        if let Some(target) = target_empty_slot {
            // update checksum, set slot as inactive if emptied.
            let tgt_ref = target.as_ref().unwrap();

            let start_block_idx = get_block_idx_start_from_arr_idx(&input, tgt_ref.index);
            let offset = calc_offset(&input, tgt_ref);
            sum_data.update_checksum(
                start_block_idx,
                curr_slot.index / 2,
                curr_slot.count,
                offset,
            );

            // println!("Hey.. so: curr_slot is: {:?}, target is: {:?}, start_block_idx is: {}, offset is: {}", &curr_slot, target.as_ref().unwrap(), start_block_idx, offset);

            target.as_mut().unwrap().count -= curr_slot.count;
            if target.as_ref().unwrap().count == 0 {
                *target = None;
            }
        } else {
            let start_block_idx = get_block_idx_start_from_arr_idx(&input, curr_slot.index);
            sum_data.update_checksum(start_block_idx, curr_slot.index / 2, curr_slot.count, 0);
            // println!(
            //     "Hey.. so: curr_slot is: {:?}, it actually stays., start_block_idx is: {}",
            //     &curr_slot, start_block_idx,
            // );
        }
    }
    sum_data.checksum
}

fn get_available_slot<'a>(
    empty_slots: &'a mut [Option<SlotData>],
    curr_slot: &SlotData,
) -> Option<&'a mut Option<SlotData>> {
    empty_slots.iter_mut().find(|os| {
        if let Some(s) = os {
            s.count >= curr_slot.count && s.index < curr_slot.index
        } else {
            false
        }
    })
}

fn get_block_idx_start_from_arr_idx(input: &[u8], idx: usize) -> usize {
    input[0..idx].iter().map(|n| *n as usize).sum()
}

fn calc_offset(input: &[u8], empty_slot: &SlotData) -> u8 {
    // original - current
    input[empty_slot.index] - empty_slot.count
}
