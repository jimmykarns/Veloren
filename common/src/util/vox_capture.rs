use crate::{
    terrain::Block,
    vol::{ReadVol, Vox},
};
use color_quant::NeuQuant;
use std::path::Path;
use vek::*;

// Given a `ReadVol`, a center position, and a filename
// Saves a 256x256x256 cube of volume data in .vox format
// Uses `color_quant` to keep the color count to the limits imposed by magica
pub fn vox_capture(
    vol: &impl ReadVol<Vox = Block>,
    center: Vec3<i32>,
    save_path: &Path,
) -> Result<String, String> {
    // First read block into color and pos vecs
    let (positions, colors) = (-128..128)
        .flat_map(move |x| {
            (-128..128).flat_map(move |y| (-128..128).map(move |z| Vec3::new(x, y, z) + center))
        })
        .map(|pos| (pos, vol.get(pos).ok().copied().unwrap_or(Block::empty())))
        .filter_map(|(pos, block)| {
            block.get_color().map(|color| {
                (
                    (pos - center + Vec3::from(128)).map(|e| e as u8),
                    Rgba::from(color),
                )
            })
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut positions, mut colors), (pos, color)| {
                positions.push(pos);
                colors.extend_from_slice(&color);
                (positions, colors)
            },
        );

    // Quantize colors
    // dot_vox docs seem to imply there are only 255 (and not 256) indices in
    // palette
    let quant = NeuQuant::new(10, 255, &colors);
    // Extract palette
    // Note: palette includes alpha, we could abuse this as alternative to indices
    // to store extra info
    let palette = quant
        .color_map_rgba()
        .chunks_exact(4)
        .map(|c| {
            // Magica stores them backwards?
            ((c[3] as u32) << 24)
                | ((c[2] as u32) << 16)
                | ((c[1] as u32) << 8)
                | ((c[0] as u32) << 0)
        })
        .collect();
    // Build voxel list with palette indices
    let voxels = colors
        .chunks_exact(4)
        .map(|p| quant.index_of(p) as u8)
        .zip(positions)
        .map(|(index, pos)| dot_vox::Voxel {
            x: pos.x,
            y: pos.y,
            z: pos.z,
            i: index,
        })
        .collect();

    let model = dot_vox::Model {
        size: dot_vox::Size {
            x: 256,
            y: 256,
            z: 256,
        },
        voxels,
    };

    let dot_vox_data = dot_vox::DotVoxData {
        version: 150, // TODO: is this correct at all??
        models: vec![model],
        palette,
        materials: Vec::new(),
    };

    let save_path = save_path.with_extension("vox");
    // Check if folder exists and create it if it does not
    if !save_path.parent().map_or(false, |p| p.exists()) {
        std::fs::create_dir_all(&save_path.parent().unwrap())
            .map_err(|err| format!("Couldn't create folder for vox capture: {:?}", err))?;
    }
    // Attempt to create a file (hopefully all this effort wasn't for nothing...)
    let mut writer = std::fs::File::create(save_path.with_extension("vox"))
        .map(|file| std::io::BufWriter::new(file))
        .map_err(|err| format!("Failed to create file to save vox: {:?}", err))?;

    // Save
    dot_vox_data
        .write_vox(&mut writer)
        .map(|_| format!("Succesfully saved vox to: {}", save_path.to_string_lossy()))
        .map_err(|err| format!("Failed to write vox: {:?}", err))
}
