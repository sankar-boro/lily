use actix_web::{HttpResponse,web};
use crate::Connections;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use scylla::{macros::FromRow, query::Query};
use crate::utils::{
    ParseUuid,
	GetQueryResult
};
use crate::query::{GET_SIZE, PAGE_SIZE};
use crate::Error;

#[derive(FromRow, Serialize)]
pub struct BlogMetadata {
    docid: Uuid,
    authorId: i32,
    title: String,
    body: String,
    url: Option<String>,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct BlogsMetadataResponse {
    blogs: Vec<BlogMetadata>,
    page: Option<Vec<u8>>,
}

// cannot use * when getting all documents;
static BLOGS_QUERY: &'static str = "SELECT docid, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.blogs";
pub async fn getBlogsWithPageSize(
    app: web::Data<Connections>
) 
-> Result<HttpResponse, Error> {
    let query = Query::new(BLOGS_QUERY).with_page_size(GET_SIZE);
    let documents = 
    app.query(query, &[])
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BlogMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogsMetadataResponse{
                blogs: docs,
                page
            }))
        },
        None => {
            let x: Vec<BlogMetadata> = Vec::new();
            let y = BlogsMetadataResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct NextPageRequest {
    page: Vec<u8>
}

// cannot use * when getting all documents;
pub async fn getNextBlogsWithPageSize(
    app: web::Data<Connections>, 
    request: web::Json<NextPageRequest>
) 
-> Result<HttpResponse, Error> {
    let query = Query::new(BLOGS_QUERY).with_page_size(GET_SIZE);
    let documents = 
    app.query_paged(query, &[], request.page.clone())
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BlogMetadata>> = documents.get_query_result()?;
    match documents {
        Some(blogs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogsMetadataResponse{
                blogs,
                page
            }))
        },
        None => {
            let x: Vec<BlogMetadata> = Vec::new();
            let y = BlogsMetadataResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

#[derive(FromRow, Serialize)]
pub struct BlogNode {
    docid: Uuid,
    uniqueId: Uuid,
    parentId: Option<Uuid>,
    authorId: Option<i32>,
    title: String,
    body: String,
    identity: i16,
    metadata: Option<String>,
    url: Option<String>,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct BlogNodesResponse {
    nodes: Vec<BlogNode>,
    page: Option<Vec<u8>>,
}

static GET_BLOG_NODES_WITH_PAGE_SIZE: &'static str = "SELECT docid, uniqueId, parentId, authorId, title, body, identity, metadata, url, createdAt, updatedAt from sankar.blog WHERE docid=";
pub async fn getBlogNodesWithPageSizeFromId(
    app: web::Data<Connections>, 
    blog_id: web::Path<String>
) -> Result<HttpResponse, Error> 
{
    let query = format!("{}{}", GET_BLOG_NODES_WITH_PAGE_SIZE, &blog_id.to_uuid()?);
    let query = Query::new(query).with_page_size(PAGE_SIZE);
    let documents = 
		app.query(query, &[]).await?;
    let page = documents.paging_state.clone();
	let documents: Option<Vec<BlogNode>> = documents.get_query_result()?;

    match documents {
        Some(nodes) => {
            let page = match page {
                Some(page) => {
                    Some(page.to_vec())
                },
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogNodesResponse{nodes, page}))
        },
        None => {
            let x: Vec<BlogNode> = Vec::new();
            let y = BlogNodesResponse{nodes: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}


pub async fn getNextBlogNodesWithPageSizeFromId(
    app: web::Data<Connections>, 
    blog_id: web::Path<String>,
    request: web::Json<NextPageRequest>,
) -> Result<HttpResponse, Error> {
    let query = format!("{}{}", GET_BLOG_NODES_WITH_PAGE_SIZE, &blog_id.to_uuid()?);
    let query = Query::new(query).with_page_size(PAGE_SIZE);
    let documents = 
		app.query_paged(query, &[], request.page.clone())
		.await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BlogNode>> = documents.get_query_result()?;

    match documents {
        Some(nodes) => {
            let page = match page {
                Some(page) => {
                    Some(page.to_vec())
                },
                None => None,
            };
            return Ok(HttpResponse::Ok().json(BlogNodesResponse{nodes, page }));
        },
        None => {
            let x: Vec<BlogNode> = Vec::new();
            let y = BlogNodesResponse{nodes: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}


#[derive(FromRow, Serialize)]
pub struct CategoryBlogMetadata {
    category: String,
    docid: Uuid,
    authorId: i32,
    title: String,
    body: String,
    url: Option<String>,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct CategoryBlogsMetadataResponse {
    blogs: Vec<CategoryBlogMetadata>,
    page: Option<Vec<u8>>,
}

// cannot use * when getting all documents;
static BOOKS_QUERY_CATEGORY: &'static str = "SELECT category, docid, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.categoryblogs WHERE category";
pub async fn getBlogsWithPageSizeCategories(
    app: web::Data<Connections>,
    category: web::Path<String>,
) 
-> Result<HttpResponse, Error> 
{
    let mut categories = String::from("");
    let split_categories: Vec<&str> = category.split('-').collect();
    for (c_index, category) in split_categories.iter().enumerate() {
        if c_index != split_categories.len() - 1 {
            categories.push_str(&format!("'{}',", category));
        } else {
            categories.push_str(&format!("'{}'", category));
        }
    }

    let query = format!("{} IN ({})", BOOKS_QUERY_CATEGORY, categories);
    let query = Query::new(query).with_page_size(GET_SIZE);
    let documents = app.query(query, &[])
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<CategoryBlogMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(CategoryBlogsMetadataResponse{blogs: docs, page }))
        },
        None => {
            let x: Vec<CategoryBlogMetadata> = Vec::new();
            let y = CategoryBlogsMetadataResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

pub async fn getBlogsWithPageSizeCategoriesNext(
    app: web::Data<Connections>,
    category: web::Path<String>,
    request: web::Json<NextPageRequest>,
) 
-> Result<HttpResponse, Error> 
{
    let mut categories = String::from("");
    let split_categories: Vec<&str> = category.split('-').collect();
    for (c_index, category) in split_categories.iter().enumerate() {
        if c_index != split_categories.len() - 1 {
            categories.push_str(&format!("'{}',", category));
        } else {
            categories.push_str(&format!("'{}'", category));
        }
    }

    let query = format!("{} IN ({})", BOOKS_QUERY_CATEGORY, categories);
    let query = Query::new(query).with_page_size(GET_SIZE);
    let documents = app.query_paged(query, &[], request.page.clone())
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<CategoryBlogMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(CategoryBlogsMetadataResponse{blogs: docs, page }))
        },
        None => {
            let x: Vec<CategoryBlogMetadata> = Vec::new();
            let y = CategoryBlogsMetadataResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}