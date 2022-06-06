use std::io::{BufRead, BufReader, Read};

use anyhow::{bail, Ok};
use byteorder::{LittleEndian, ReadBytesExt};
use clap::{Parser, Subcommand};
use tracing::{debug, error, info};

use messiah_texture::EPixelFormat;

#[derive(Subcommand)]
enum Command {
    /// Convert the given Texture2D into a dds image file. Applying all required conversions
    ConvertDDS {
        #[clap(help = "Input Texture2D file")]
        texture_file: String,
        #[clap(help = "Target file name")]
        target: Option<String>,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

struct TextureHeader {
    format: EPixelFormat,
    width: u16,
    height: u16,
    mip_levels: u16,
}

impl std::fmt::Display for TextureHeader {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Texture2D")
            .field("format", &self.format)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("mip_levels", &self.mip_levels)
            .finish()
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    match args.command {
        Command::ConvertDDS {
            texture_file,
            target,
        } => {
            let file = std::fs::File::open(&texture_file)?;
            let mut reader = BufReader::new(&file);

            let magic = reader.read_u32::<LittleEndian>()?;
            if magic != 16908802 {
                bail!(
                    "Invalid Texture filed expected magic {:x} got {:x} instead",
                    16908802,
                    magic
                );
            }

            let _unk1 = reader.read_u8()?;
            let format = EPixelFormat::try_from(reader.read_u8()?)?;
            let _unk2 = reader.read_u8()?;
            let _unk3 = reader.read_u8()?;
            let _ = reader.read_u32::<LittleEndian>()?;
            let width = reader.read_u16::<LittleEndian>()?;
            let height = reader.read_u16::<LittleEndian>()?;
            reader.consume(16);
            let _size_till_end = reader.read_u32::<LittleEndian>()?;
            let _unk4 = reader.read_u16::<LittleEndian>()?;
            let num_mip_levels = reader.read_u16::<LittleEndian>()?;

            let header = TextureHeader {
                format,
                width,
                height,
                mip_levels: num_mip_levels,
            };

            info!("{}", header);

            enum Format {
                D3D(ddsfile::D3DFormat),
                DXGI(ddsfile::DxgiFormat),
            }

            impl From<ddsfile::DxgiFormat> for Format {
                fn from(other: ddsfile::DxgiFormat) -> Self {
                    Self::DXGI(other)
                }
            }

            impl From<ddsfile::D3DFormat> for Format {
                fn from(other: ddsfile::D3DFormat) -> Self {
                    Self::D3D(other)
                }
            }

            let format: Format = match header.format {
                EPixelFormat::Unknown => todo!(),
                EPixelFormat::A32R32G32B32F => ddsfile::D3DFormat::A32B32G32R32F.into(),
                EPixelFormat::A16B16G16R16F => ddsfile::D3DFormat::A16B16G16R16F.into(),
                EPixelFormat::R8G8B8A8 => ddsfile::DxgiFormat::R8G8B8A8_UNorm.into(),
                EPixelFormat::B5G6R5 => ddsfile::DxgiFormat::B5G6R5_UNorm.into(),
                EPixelFormat::A8L8 => ddsfile::D3DFormat::A8L8.into(),
                EPixelFormat::G16R16 => ddsfile::D3DFormat::G16R16.into(),
                EPixelFormat::G16R16F => ddsfile::D3DFormat::G16R16F.into(),
                EPixelFormat::G32R32F => ddsfile::D3DFormat::G32R32F.into(),
                EPixelFormat::R32F => ddsfile::D3DFormat::R32F.into(),
                EPixelFormat::R16F => ddsfile::D3DFormat::R16F.into(),
                EPixelFormat::L8 => ddsfile::D3DFormat::L8.into(),
                EPixelFormat::L16 => ddsfile::D3DFormat::L16.into(),
                EPixelFormat::A8 => ddsfile::D3DFormat::A8.into(),
                EPixelFormat::FloatRGB => todo!(),
                EPixelFormat::FloatRGBA => todo!(),
                EPixelFormat::D24 => ddsfile::DxgiFormat::D24_UNorm_S8_UInt.into(),
                EPixelFormat::D32 => ddsfile::DxgiFormat::D32_Float.into(),
                EPixelFormat::BC1 => ddsfile::DxgiFormat::BC1_UNorm.into(),
                EPixelFormat::BC2 => ddsfile::DxgiFormat::BC2_UNorm.into(),
                EPixelFormat::BC3 => ddsfile::DxgiFormat::BC3_UNorm.into(),
                EPixelFormat::BC4 => ddsfile::DxgiFormat::BC4_UNorm.into(),
                EPixelFormat::BC5 => ddsfile::DxgiFormat::BC5_UNorm.into(),
                EPixelFormat::BC6H_SF => ddsfile::DxgiFormat::BC6H_SF16.into(),
                EPixelFormat::BC6H_UF => ddsfile::DxgiFormat::BC6H_UF16.into(),
                EPixelFormat::BC7 => ddsfile::DxgiFormat::BC7_UNorm.into(),
                EPixelFormat::PVRTC2_RGB => todo!(),
                EPixelFormat::PVRTC2_RGBA => todo!(),
                EPixelFormat::PVRTC4_RGB => todo!(),
                EPixelFormat::ETC1 => todo!(),
                EPixelFormat::ETC2_RGB => todo!(),
                EPixelFormat::ETC2_RGBA => todo!(),
                EPixelFormat::ATC_RGB => todo!(),
                EPixelFormat::ATC_RGBA_E => todo!(),
                EPixelFormat::ATC_RGBA_I => todo!(),
                EPixelFormat::ASTC_4x4_LDR => todo!(),
                EPixelFormat::ASTC_5x4_LDR => todo!(),
                EPixelFormat::ASTC_5x5_LDR => todo!(),
                EPixelFormat::ASTC_6x5_LDR => todo!(),
                EPixelFormat::ASTC_6x6_LDR => todo!(),
                EPixelFormat::ASTC_8x5_LDR => todo!(),
                EPixelFormat::ASTC_8x6_LDR => todo!(),
                EPixelFormat::ASTC_8x8_LDR => todo!(),
                EPixelFormat::ASTC_10x5_LDR => todo!(),
                EPixelFormat::ASTC_10x6_LDR => todo!(),
                EPixelFormat::ASTC_10x8_LDR => todo!(),
                EPixelFormat::ASTC_10x10_LDR => todo!(),
                EPixelFormat::ASTC_12x10_LDR => todo!(),
                EPixelFormat::ASTC_12x12_LDR => todo!(),
                EPixelFormat::DepthStencil => todo!(),
                EPixelFormat::ShadowDepth => todo!(),
                EPixelFormat::ShadowDepth32 => todo!(),
                EPixelFormat::R10G10B10A2 => ddsfile::DxgiFormat::R10G10B10A2_UNorm.into(),
                EPixelFormat::R32U => ddsfile::DxgiFormat::R32_UInt.into(),
                EPixelFormat::R11G11B10F => ddsfile::DxgiFormat::R11G11B10_Float.into(),
                EPixelFormat::ASTC_4x4_HDR => todo!(),
                EPixelFormat::ASTC_5x4_HDR => todo!(),
                EPixelFormat::ASTC_5x5_HDR => todo!(),
                EPixelFormat::ASTC_6x5_HDR => todo!(),
                EPixelFormat::ASTC_6x6_HDR => todo!(),
                EPixelFormat::ASTC_8x5_HDR => todo!(),
                EPixelFormat::ASTC_8x6_HDR => todo!(),
                EPixelFormat::ASTC_8x8_HDR => todo!(),
                EPixelFormat::ASTC_10x5_HDR => todo!(),
                EPixelFormat::ASTC_10x6_HDR => todo!(),
                EPixelFormat::ASTC_10x8_HDR => todo!(),
                EPixelFormat::ASTC_10x10_HDR => todo!(),
                EPixelFormat::ASTC_12x10_HDR => todo!(),
                EPixelFormat::ASTC_12x12_HDR => todo!(),
                EPixelFormat::A32R32G32B32UI => ddsfile::DxgiFormat::R32G32B32A32_UInt.into(),
            };

            let mut dds = match format {
                Format::D3D(format) => ddsfile::Dds::new_d3d(ddsfile::NewD3dParams {
                    height: height as u32,
                    width: width as u32,
                    depth: None,
                    format,
                    mipmap_levels: Some(num_mip_levels as u32),
                    caps2: None,
                })?,
                Format::DXGI(format) => ddsfile::Dds::new_dxgi(ddsfile::NewDxgiParams {
                    height: height as u32,
                    width: width as u32,
                    depth: None,
                    format,
                    mipmap_levels: Some(num_mip_levels as u32),
                    array_layers: None,
                    caps2: None,
                    is_cubemap: false,
                    resource_dimension: ddsfile::D3D10ResourceDimension::Texture2D,
                    alpha_mode: ddsfile::AlphaMode::PreMultiplied,
                })?,
            };
            let out_data = dds.get_mut_data(0)?;

            // TODO(alexander): Sort the mip levels correctly
            // Right now we assume it's smallest to largest
            // DDS requires largest to smallest so we build a reverse buffer
            // Ideally we would store them and sort them all in the correct order
            let mut out_texture_data: Vec<u8> = vec![];
            for _mip_level in 0..num_mip_levels {
                let mip_size_in_bytes = reader.read_u32::<LittleEndian>()?;
                let _mip_width = reader.read_u16::<LittleEndian>()?;
                let _mip_height = reader.read_u16::<LittleEndian>()?;
                let _ = reader.read_u16::<LittleEndian>()?;
                let _ = reader.read_u16::<LittleEndian>()?;
                let texture_data_in_bytes = reader.read_u32::<LittleEndian>()?;
                let mut magic: [u8; 4] = [0; 4];
                reader.read_exact(&mut magic)?;
                let texture_data = if &magic == b"NNNN" {
                    if texture_data_in_bytes > 0 {
                        let mut buf = vec![0; texture_data_in_bytes as usize];
                        reader.read_exact(&mut buf)?;
                        buf
                    } else {
                        let mut buf = vec![0; width as usize * height as usize * 4];
                        reader.read_exact(&mut buf)?;
                        buf
                    }
                } else if &magic == b"ZZZ4" {
                    let uncompressed_size = reader.read_u32::<LittleEndian>()?;
                    let mut buf = vec![0; mip_size_in_bytes as usize - 24];
                    reader.read_exact(&mut buf)?;
                    let texture_data = lz4_flex::decompress(&buf, uncompressed_size as usize)?;
                    texture_data
                } else {
                    error!("Unknown texture data format: {:?}", &magic);
                    vec![]
                };
                debug!("Mip Size: {}", texture_data.len());
                out_texture_data.splice(0..0, texture_data.into_iter());
            }

            out_data.clone_from_slice(&out_texture_data);

            let target_file = if let Some(ref target) = &target {
                std::path::PathBuf::from(&target)
            } else {
                let mut n = std::path::PathBuf::from(&texture_file);
                n.set_extension("dds");
                n
            };
            let mut f = std::fs::File::create(target_file)?;

            dds.write(&mut f)?;
        }
    }

    Ok(())
}
