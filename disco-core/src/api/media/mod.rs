/// POST /api/media
pub mod post;
pub mod data;

use mongodb::bson::oid::ObjectId;
use crate::mongo::media::{Format, Status};
use mongodb::bson::doc;
use crate::mongo::user::Alias;

const MEDIA_ROOT_FOLDER: &str = "media/";


pub async fn claim_media_filter(
    oid: &ObjectId,
    expected:&Format,
    uploaded_by: &Alias
) -> mongodb::bson::Document {
    doc! {
        "_id": oid ,
        "status": mongodb::bson::to_bson(&Status::Waiting).unwrap(),
        "format": mongodb::bson::to_bson(expected).unwrap(),
        "uploaded_by" : mongodb::bson::to_bson(uploaded_by).unwrap()
    }
}

pub async fn claim_media_update() -> mongodb::bson::Document {
    doc! { "$set": { "status": mongodb::bson::to_bson(&Status::Assigned).unwrap() } }
}


pub fn oid_to_path(oid:&mongodb::bson::oid::ObjectId) -> String {
    format!("{}/{}.blob",oid_to_folder(&oid),oid)
}

fn oid_to_folder(oid:&mongodb::bson::oid::ObjectId) -> String {
    oid
        .bytes()
        .iter()
        .fold(MEDIA_ROOT_FOLDER.to_string(), |mut acc, n| {
            acc.push('/');
            acc.push_str(n.to_string().as_str());
            acc
        })
}
