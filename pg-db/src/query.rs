#![allow(unused_imports, dead_code)]

#[macro_export]
macro_rules! create_query {
    ( $a:expr, $( $x:expr ),* ) => {
        {
            let xx = format!("INSERT INTO {}", $a);
            let mut aa = String::from("(");
            let mut bb = String::from("VALUES (");
            $(
                aa.push_str($x);
                aa.push_str(", ");
                bb.push_str("?, ");
            )*
            let mut aa = format!("{}", &aa[0..aa.len()-2]);
            let mut bb = format!("{}", &bb[0..bb.len()-2]);
            aa.push_str(")");
            bb.push_str(")");
            format!("{} {} {}", xx, aa, bb)
        }
    };
}

/* Book */

// GET
pub static BOOK_DATA: &str = "SELECT uid, authorid, docid, parentid, title, body, identity, metadata, createdat FROM booknode WHERE docid=$1";
pub static GET_BOOK_TITLES_FOR_ID: &str = "SELECT uid, parentid, title, identity, createdat FROM title WHERE docid=$1";

// INSERT
pub static CREATE_BOOK: &str = "INSERT INTO book (
    authorid, title, body, imageurl, metadata
) VALUES(
    $1, $2, $3, $4, $5
) RETURNING uid";
pub static CREATE_BOOK_TITLE: &str = "INSERT INTO title (
    docid, parentid, title, identity
) VALUES(
    $1, $2, $3, $4
)";
pub static CREATE_BOOK_NODE: &str = "INSERT INTO booknode (
    authorid, docid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8
)";

// UPDATE
pub static UPDATE_BOOKS: &str = "UPDATE books SET title=$1, body=$2, metadata=$3 WHERE uid=$4";

// DELETE
pub static DELETE_BOOKS: &str = "DELETE FROM books where uid=$1";

/* Book */


/* Blog */
pub static ALL_BLOGS: &str = "SELECT uid, authorid, title, body, metadata, createdat FROM blog";
pub static BLOG_DATA: &str = "SELECT uid, authorid, docid, parentid, title, body, identity, metadata FROM blognode WHERE docid=$1";

pub static CREATE_BLOG: &str = "INSERT INTO blog (
    authorid, title, body, imageurl, metadata
) VALUES(
    $1, $2, $3, $4, $5
) RETURNING uid";

pub static CREATE_BLOG_NODE: &str = "INSERT INTO blognode (
    authorid, docid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8
)";

pub static UPDATE_BLOGS: &str = "UPDATE blogs SET title=$1, body=$2, metadata=$3 WHERE uid=$4";

pub static DELETE_BLOGS: &str = "DELETE FROM blogs where uid=$1";

/* Blog */


/**
 * We dont include parentId, because the first node is the parent node.
 */


pub static CREATE_USER_BOOKS: &str = "INSERT INTO sankar.userbooks (
    docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_CATEGORY_BOOKS: &str = "INSERT INTO sankar.categorybooks (
    category, docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static ADD_USER_CATEGORY: &str = "INSERT INTO sankar.usercategories (
    authorId, category, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";
pub static DELETE_CATEGORY: &str = "DELETE FROM sankar.usercategories WHERE authorId=? AND category=?";
pub static CREATE_BLOGS: &str = "INSERT INTO sankar.blogs (
    docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_USER_BLOGS: &str = "INSERT INTO sankar.userblogs (
    docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_CATEGORY_BLOGS: &str = "INSERT INTO sankar.categoryblogs (
    category, docid, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_ALLCATEGORY: &str = "INSERT INTO sankar.allcategories (
    category, authorId, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";

pub static CREATE_USER_BOOK_SETTINGS: &str = "INSERT INTO sankar.userbooksettings (
    authorId, docid, settings
) VALUES(
    ?, ?, ?
)";

pub static UPDATE_USER_BOOK_SETTINGS: &str = "UPDATE sankar.userbooksettings SET settings=? where authorId=? AND docid=?";

pub static FOLLOW_USER: &str = "INSERT INTO sankar.followers (
    userId, followerId, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";
pub static UNFOLLOW_USER: &str = "DELETE FROM sankar.followers WHERE userId=? AND followerId=?";

pub static DELETE_USERBOOKS: &str = "DELETE FROM sankar.userbooks where authorId=? AND docid IN (?)";

pub static SIGNUP: &str = "INSERT INTO users (fname, lname, email, password) VALUES ($1, $2, $3, $4) RETURNING uid";
pub static LOGIN: &str = "SELECT user_id, fname, lname, password FROM users WHERE phone=$1";