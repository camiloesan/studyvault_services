use crate::posts::posts::PostInfo;
use crate::posts::posts_service_server::PostsService;
use crate::posts::Channel;
use crate::posts::Posts;
use crate::posts::{FileChunk, UploadStatus};
use crate::sql_operations;
use tokio::io::AsyncWriteExt;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Default)]
pub struct PostsServicesStruct;

#[tonic::async_trait]
impl PostsService for PostsServicesStruct {
    async fn upload_post(
        &self,
        request: Request<tonic::Streaming<FileChunk>>,
    ) -> Result<Response<UploadStatus>, Status> {
        println!("upload requested");
        let mut stream = request.into_inner();
        let mut file = None;
        let mut file_name: Option<String> = None;
        let mut channel_id: Option<u32> = None;
        let mut title: Option<String> = None;
        let mut description: Option<String> = None;
        let uuid: String = Uuid::new_v4().to_string();

        while let Some(file_chunk) = stream.message().await? {
            if file_name.is_none() {
                channel_id = Some(file_chunk.channel_id);
                file_name = Some(file_chunk.filename);
                tokio::fs::create_dir_all(format!(
                    "/data/files/{}",
                    &channel_id.unwrap().to_string()
                ))
                .await
                .map_err(|e| {
                    eprintln!("Failed to create directory: {:?}", e);
                    Status::internal("Failed to create directory")
                })?;

                let extension = file_name.as_ref().unwrap().split('.').last().unwrap();
                let file_path = format!(
                    "/data/files/{}/{}",
                    channel_id.unwrap().to_string(),
                    format!("{}.{}", &uuid, extension)
                );
                file = Some(tokio::fs::File::create(file_path).await.map_err(|e| {
                    eprintln!("Failed to create file: {:?}", e);
                    Status::internal("Failed to create file")
                })?);
            }

            title = Some(file_chunk.title);
            description = Some(file_chunk.description);

            if let Some(ref mut f) = file {
                f.write_all(&file_chunk.content).await.map_err(|e| {
                    eprintln!("Failed to write file data: {:?}", e);
                    Status::internal("Failed to write file data")
                })?;
            }
        }

        let result = sql_operations::create_post(
            uuid,
            channel_id.unwrap(),
            file_name.unwrap(),
            title.unwrap(),
            description.unwrap(),
        )
        .await;

        Ok(Response::new(UploadStatus {
            success: result,
            message: format!("File uploaded successfully"),
        }))
    }

    async fn get_posts_by_channel_id(
        &self,
        request: Request<Channel>,
    ) -> Result<Response<Posts>, Status> {
        let channel_id = request.into_inner().channel_id;

        let db_posts = sql_operations::get_posts_by_channel_id(channel_id).await;

        let post_infos: Vec<PostInfo> = db_posts
            .into_iter()
            .map(|post| PostInfo {
                post_id: post.post_id,
                channel_id: post.channel_id,
                file_id: post.file_id,
                title: post.title,
                description: post.description,
                publish_date: post.publish_date,
            })
            .collect();

        let response = Posts { posts: post_infos };

        Ok(Response::new(response))
    }
}
