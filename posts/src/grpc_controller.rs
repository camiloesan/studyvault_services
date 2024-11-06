use crate::posts::posts_response::PostInfo;
use crate::posts::posts_service_server::PostsService;
use crate::posts::ChannelRequest;
use crate::posts::PostsResponse;
use crate::posts::{FileChunk, UploadStatusResponse};
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
    ) -> Result<Response<UploadStatusResponse>, Status> {
        println!("upload requested");
        let mut stream = request.into_inner();
        let uuid: String = Uuid::new_v4().to_string();

        let mut channel_id = None;
        let mut file_name = None;
        let mut title = None;
        let mut description = None;

        while let Some(file_chunk) = stream.message().await? {
            channel_id = Some(file_chunk.channel_id);
            file_name = Some(file_chunk.filename);
            title = Some(file_chunk.title);
            description = Some(file_chunk.description);

            tokio::fs::create_dir_all(format!("/data/files/{}", &channel_id.unwrap().to_string()))
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
            let mut file = Some(tokio::fs::File::create(file_path).await.map_err(|e| {
                eprintln!("Failed to create file: {:?}", e);
                Status::internal("Failed to create file")
            })?);

            if let Some(ref mut f) = file {
                f.write_all(&file_chunk.content).await.map_err(|e| {
                    eprintln!("Failed to write file data: {:?}", e);
                    Status::internal("Failed to write file data")
                })?;
            }
        }

        let sql_result = sql_operations::create_post(
            uuid.clone(),
            channel_id.unwrap(),
            file_name.clone().unwrap(),
            title.unwrap(),
            description.unwrap(),
        )
        .await;

        if !sql_result {
            return Ok(Response::new(UploadStatusResponse {
                success: false,
                message: format!("Failed to upload file"),
            }));
        }

        Ok(Response::new(UploadStatusResponse {
            success: sql_result,
            message: format!("File uploaded successfully"),
        }))
    }

    async fn get_posts_by_channel_id(
        &self,
        request: Request<ChannelRequest>,
    ) -> Result<Response<PostsResponse>, Status> {
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

        let response = PostsResponse { posts: post_infos };

        Ok(Response::new(response))
    }
}
