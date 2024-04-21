use image::GenericImageView;
use anyhow::*;
use cfg_if::cfg_if;


pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.
    


pub fn from_bytes(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: &[u8], 
    label: &str
) -> Result<wgpu::Texture>
{
    let img = image::load_from_memory(bytes)?;
    from_image(device, queue, &img, Some(label))
}



pub fn from_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img: &image::DynamicImage,
    label: Option<&str>
) -> Result<wgpu::Texture> 
{
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(
        &wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        }
    );
    queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        size,
    );
    Ok(texture)
}

   


pub fn create_depth_texture(
    device: &wgpu::Device,
    size : wgpu::Extent3d,
    label: &str,
    filter: wgpu::FilterMode,
) -> wgpu::Texture
{

    let desc = wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 3.
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };
    let texture = device.create_texture(&desc);
    texture
}






pub fn default_white(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture
{
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Default White Texture"),
        size: wgpu::Extent3d {
            width: 1, 
            height: 1, 
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2, 
        // format: wgpu::TextureFormat::Bgra8UnormSrgb,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let white_pixel = [255u8, 255u8, 255u8, 255u8];
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0, 
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &white_pixel, 
        wgpu::ImageDataLayout {
            offset: 0, 
            bytes_per_row: Some(4), 
            rows_per_image: None,
        },
        wgpu::Extent3d {
            width: 1, 
            height: 1, 
            depth_or_array_layers: 1,
        },
    );
    texture
}



pub fn create_blank_texture(
    device: &wgpu::Device, 
    size : wgpu::Extent3d,
    label: &str,
    filter: wgpu::FilterMode,
) -> wgpu::Texture
{
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size, 
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        // format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
        label: Some(label),
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
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<wgpu::Texture> 
{
    if file_name.is_empty() {
        return Ok(default_white(device,queue));
    }
    let data = load_binary(file_name).await?;
    from_bytes(device, queue, &data, file_name)
}
