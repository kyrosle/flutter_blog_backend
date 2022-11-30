use std::collections::HashMap;

use once_cell::sync::Lazy;
use rbdc_pg::driver::PgDriver;

pub static SQL: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("fetchArticleList", "select a.id, a.title, c.id as category, a.content  from article a left join category c on a.category = c.id where c.id = ?;");
    m.insert("fetchArticleCates", "select c.id, c.name from category c;");
    m.insert("fetchArticleDetail", "select a.id, a.title, c.id as category, a.content  from article a left join category c on a.category = c.id where a.id = ?;");
    m.insert(
        "fetchArticleCteSingleById",
        "select * from category c where c.id = ?;",
    );
    m
});

#[allow(dead_code)]
pub static RB: Lazy<rbatis::Rbatis> = Lazy::new(|| {
    // local docker
    // match rb.init(PgDriver {}, "postgres://postgres:1234@localhost:55435/blog") {
    let rb = rbatis::Rbatis::new();
    // local raspberry docker postgres 
    match rb.init(PgDriver {}, "postgres://postgres:1234@192.168.0.188:55433/blog") {
        Ok(_) => {}
        Err(e) => panic!("!rbatis connect mysql server error!\n{}", e),
    }
    rb
});
