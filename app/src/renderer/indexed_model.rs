use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use ogc_rs::{print, println};
use wavefront::{Index, Obj, Vertex};

/**
 * Our representation of a model.
 */
pub struct IndexedModel {
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
    pub tex_coords: Vec<f32>,
}

/**
 * Implementation of the indexed model.
 */
impl IndexedModel {
    /**
     * Turn a model into its indexed equivalent.
     *
     * This is done by filling a memotable whose keys are known vertices that we have seen before,
     * and whose values are indexes into an array containing the position of those vertices.
     */
    pub fn new(obj_data: &Obj) -> IndexedModel {
        let mut memo: BTreeMap<Index, u16> = BTreeMap::new();
        let mut positions = Vec::new();
        let mut tex_coords: Vec<f32> = Vec::new();
        let indices = obj_data
            .vertices()
            .map(|vertex| {
                let vertexId = vertex.position_index();
                *memo.entry(vertexId).or_insert_with(|| {
                    let index = (positions.len() / 3) as u16;
                    positions.extend(vertex.position());
                    let uv: [f32; 2] = vertex.uv().unwrap_or([0.0, 0.0, 0.0])[0..2]
                    .try_into()
                    .expect("UV slice with incorrect length");
                    tex_coords.extend(uv);
                    index
                })
            })
            .collect();

        IndexedModel {
            vertices: positions,
            indices,
            tex_coords
        }
    }
}
