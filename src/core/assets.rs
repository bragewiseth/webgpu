use std::io::{BufReader, Cursor};
use cfg_if::cfg_if;





#[cfg(target_arch = "wasm32")]
fn format_url(file_name: &str) -> reqwest::Url {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let mut origin = location.origin().unwrap();
    if !origin.ends_with("f✦stop") {
        origin = format!("{}/assets", origin);
    }
    let base = reqwest::Url::parse(&format!("{}/", origin,)).unwrap();
    base.join(file_name).unwrap()
}


pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    println!("Loading {:?}", file_name);
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(file_name);
            let txt = reqwest::get(url)
                .await?
                .text()
                .await?;
        } else {
            let path = std::path::Path::new(env!("OUT_DIR"))
                .join("assets")
                .join(file_name);
            let txt = std::fs::read_to_string(path)?;
        }
    }
    Ok(txt)
}





struct ObjMesh {
    name: String,
    mesh: tobj::Model,
}


struct ObjMaterial {
    name: String,
    material: tobj::Material,
}



pub async fn load_model( file_name: &str,) -> anyhow::Result<(Vec<ObjMesh>, Vec<ObjMaterial>)> 
{
    let obj_text = load_string(file_name).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions 
        {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move 
        {
            let mat_text = load_string(&p).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        }
    ) .await?;

    let mut materials = Vec::new();
    
    if obj_materials.is_ok() {
        for m in obj_materials? {
        materials.push(
        ObjMaterial
        {
            name: m.name.clone(),
            material: m,
        });
    }}

    if models.is_empty() {
        return Err(anyhow::anyhow!("No models found"));
    }
    else {
        let mut meshes = Vec::new();
        for m in models {
            let mesh = ObjMesh {
                name: m.name.clone(),
                mesh: m,
            };
            meshes.push(mesh);
        }
        Ok((meshes, materials))
    }
}






