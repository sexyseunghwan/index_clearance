use crate::common::*;

use crate::repository::es_repository::*;


#[async_trait]
pub trait IndexClearService {
    
}


#[derive(Clone, Debug)]
pub struct IndexClearServicePub<R: EsRepository> {
    elastic_obj: R
}

impl<R: EsRepository> IndexClearServicePub<R> {
    
    pub fn new(elastic_obj: R) -> Self {
        Self {elastic_obj}
    } 
}

#[async_trait]
impl<R: EsRepository> IndexClearService for IndexClearServicePub<R> {   

}