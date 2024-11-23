use crate::posts::posts_response::PostInfo;
use crate::posts::posts_service_server::PostsService;
use crate::posts::{
    ChannelRequest, FileChunk, FileData, FileDownloadRequest, FileId, FileName, PostsResponse,
    UploadStatusResponse,
};
use crate::sql_operations;
use async_stream::try_stream;
use futures_util::{Stream, StreamExt};
use log::{error, info};
use std::pin::Pin;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
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
        info!("Received request to upload post");

        let mut stream = request.into_inner();
        let uuid: String = Uuid::new_v4().to_string();
        let mut channel_id = None;
        let mut file_name = None;
        let mut title = None;
        let mut description = None;
        let mut file = None;

        while let Some(file_chunk) = stream.next().await {
            let file_chunk = file_chunk?;

            if channel_id.is_none() {
                channel_id = Some(file_chunk.channel_id);
                file_name = Some(file_chunk.filename);
                title = Some(file_chunk.title);
                description = Some(file_chunk.description);

                let channel_path = format!("/data/files/{}", channel_id.as_ref().unwrap());
                tokio::fs::create_dir_all(&channel_path)
                    .await
                    .map_err(|e| {
                        error!("Failed to create directory: {:?}", e);
                        Status::internal("Failed to create directory")
                    })?;

                let extension = file_name.as_ref().unwrap().split('.').last().unwrap();
                let file_path = format!("{}/{}.{}", channel_path, uuid, extension);

                file = Some(tokio::fs::File::create(file_path).await.map_err(|e| {
                    error!("Failed to create file: {:?}", e);
                    Status::internal("Failed to create file")
                })?);
            }

            if let Some(ref mut f) = file {
                f.write_all(&file_chunk.content).await.map_err(|e| {
                    error!("Failed to write file data: {:?}", e);
                    Status::internal("Failed to write file data")
                })?;
            }
        }

        let sql_result = sql_operations::create_post(
            uuid.clone(),
            channel_id.clone().unwrap(),
            file_name.clone().unwrap(),
            title.unwrap(),
            description.unwrap(),
        )
        .await;

        match sql_result {
            Ok(result) => {
                info!("File uploaded to server successfully");
                Ok(Response::new(UploadStatusResponse {
                    success: result,
                    message: "File uploaded successfully".to_string(),
                }))
            }
            Err(e) => {
                error!("Failed to insert file data: {:?}", e);
                Ok(Response::new(UploadStatusResponse {
                    success: false,
                    message: "Failed to upload file".to_string(),
                }))
            }
        }
    }

    async fn get_posts_by_channel_id(
        &self,
        request: Request<ChannelRequest>,
    ) -> Result<Response<PostsResponse>, Status> {
        let channel_id = request.into_inner().channel_id;

        let result = sql_operations::get_posts_by_channel_id(channel_id).await;

        if result.is_err() {
            return Err(Status::internal("Failed to get posts"));
        }

        let post_infos: Vec<PostInfo> = result
            .unwrap()
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

    async fn get_file_name_by_file_id(
        &self,
        request: Request<FileId>,
    ) -> Result<Response<FileName>, Status> {
        let file_id = request.into_inner().file_id;

        let result = sql_operations::get_file_name(file_id).await;

        if result.is_err() {
            return Err(Status::internal("Failed to get file name"));
        }

        let name = result.unwrap();

        let response = FileName { filename: name };

        Ok(Response::new(response))
    }

    type DownloadFileStream = Pin<Box<dyn Stream<Item = Result<FileData, Status>> + Send>>;

    async fn download_file(
        &self,
        request: Request<FileDownloadRequest>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        let request = request.into_inner();
        let file_id = request.file_id.clone();
        let channel_id = request.channel_id;

        let file_name = sql_operations::get_file_name(file_id.clone())
            .await
            .map_err(|e| {
                error!("Failed to get file name: {:?}", e);
                Status::internal("Failed to get file name")
            })?;

        let extension = file_name.split('.').last().unwrap_or("");

        let file_path = format!("/data/files/{}/{}.{}", channel_id, file_id, extension);

        let mut file = File::open(&file_path).await.map_err(|e| {
            error!("Failed to open file: {:?}", e);
            Status::not_found("File not found")
        })?;

        let file_stream = try_stream! {
            let mut buffer = vec![0; 1024];

            loop {
                let bytes_read = file.read(&mut buffer).await.map_err(|e| {
                    error!("Failed to read file: {:?}", e);
                    Status::internal("Failed to read file")
                })?;

                if bytes_read == 0 {
                    break;
                }

                let chunk = FileData {
                    content: buffer[..bytes_read].to_vec(),
                    filename: file_id.clone(),
                };

                yield chunk;
            }
        };

        Ok(Response::new(
            Box::pin(file_stream) as Self::DownloadFileStream
        ))
    }
}
