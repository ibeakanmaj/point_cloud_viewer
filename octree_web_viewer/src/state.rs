use crate::backend_error::PointsViewerError;
use point_viewer::octree;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct OctreeKeyParams {
    /// Location prefix, including
    prefix: String,
    /// Tree ID
    suffix: String,
}

impl OctreeKeyParams {
    pub fn get_octree_address(&self, octree_key: &String) -> Result<String, PointsViewerError> {
        Ok(format!("{}/{}/{}", self.prefix, octree_key, self.suffix))
    }
}

#[derive(Clone)]
pub struct AppState {
    /// LRU Cache for Octrees
    pub octree_map: Arc<RwLock<HashMap<String, Arc<octree::Octree>>>>,
    //pub octree_factory: octree::OctreeFactory,
    pub key_params: OctreeKeyParams,
}

impl AppState {
    pub fn new(map_size: usize, prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        AppState {
            octree_map: Arc::new(RwLock::new(HashMap::with_capacity(map_size))),
            //octree_factory: octree::OctreeFactory::new(),
            key_params: OctreeKeyParams {
                prefix: prefix.into(),
                suffix: suffix.into(),
            },
        }
    }

    pub fn load_octree(
        &self,
        uuid: impl AsRef<str>,
    ) -> Result<Arc<octree::Octree>, PointsViewerError> {
        //exists
        let octree_id = uuid.as_ref();
        {
            let map = self.octree_map.read().unwrap();
            let octree = map.get(octree_id);

            if let Some(tree) = octree {
                return Ok(Arc::clone(&tree));
            }
        }

        let octree_key: String = octree_id.to_string();
        return self.insert_octree(octree_key); //todo ownwership
    }

    pub fn insert_octree(
        &self,
        uuid: impl Into<String>,
    ) -> Result<Arc<octree::Octree>, PointsViewerError> {
        let octree_key = uuid.into();
        let addr = &self.key_params.get_octree_address(&octree_key)?;
        let octree: Arc<octree::Octree> = Arc::from(octree::octree_from_directory(&addr)?);
        {
            let mut wmap = self.octree_map.write().unwrap(); //todo try?
            wmap.insert(octree_key, Arc::clone(&octree));
        }
        Ok(octree)
    }
}