use image::GenericImageView;
use anyhow::*;
use cfg_if::cfg_if;
use wgpu::*;


pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float; // 1.
    



pub fn from_bytes(
    device: &Device,
    queue: &Queue,
    bytes: &[u8] 
) -> Result<Texture>
{
    let img = image::load_from_memory(bytes)?;
    from_image(device, queue, &img)
}



pub fn from_image(
    device: &Device,
    queue: &Queue,
    img: &image::DynamicImage
) -> Result<Texture> 
{
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(
        &TextureDescriptor {
            label: Some("Loaded Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        }
    );
    queue.write_texture(
        ImageCopyTexture {
            aspect: TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
        },
        &rgba,
        ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        size,
    );
    Ok(texture)
}

   


pub fn create_depth_texture(
    device: &Device,
    size : Extent3d,
) -> Texture
{

    let desc = TextureDescriptor {
        label: Some("Depth Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT // 3.
            | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };
    let texture = device.create_texture(&desc);
    texture
}






pub fn default_white(device: &Device, queue: &Queue) -> Texture
{
    let texture = device.create_texture(&TextureDescriptor {
        label: Some("Default White Texture"),
        size: Extent3d {
            width: 1, 
            height: 1, 
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2, 
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let white_pixel = [255u8, 255u8, 255u8, 255u8];
    queue.write_texture(
        ImageCopyTexture {
            texture: &texture,
            mip_level: 0, 
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        &white_pixel, 
        ImageDataLayout {
            offset: 0, 
            bytes_per_row: Some(4), 
            rows_per_image: None,
        },
        Extent3d {
            width: 1, 
            height: 1, 
            depth_or_array_layers: 1,
        },
    );
    texture
}



pub fn create_frame_texture(
    device: &Device, 
    size : Extent3d,
    config: &SurfaceConfiguration,
) -> Texture
{
    let texture = device.create_texture(&TextureDescriptor {
        size, 
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: config.format,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
        label: Some("Frame Texture"),
    });
    texture
}




pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    println!("Loading {:?}", file_name);
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(file_name);
            let data = reqwest::get(url)
                .await?
                .bytes()
                .await?
                .to_vec();
        } else {
            let path = std::path::Path::new(env!("OUT_DIR"))
                .join("assets")
                .join(file_name);
            let data = std::fs::read(path)?;
        }
    }
    Ok(data)
}


pub async fn load_texture(
    file_name: &str,
    device: &Device,
    queue: &Queue,
) -> anyhow::Result<Texture> 
{
    if file_name.is_empty() {
        return Ok(default_white(device,queue));
    }
    let data = load_binary(file_name).await?;
    from_bytes(device, queue, &data)
}
