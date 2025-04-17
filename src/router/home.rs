use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::Path,
    sync::LazyLock,
};

use bytes::Bytes;
use glacier::prelude::*;

pub async fn home(req: HttpRequest) -> Result<HttpResponse, ()> {
    tracing::debug!("{}", req.uri().path());

    let res = HttpResponse::Ok();

    FILES_BUF
        .get(req.uri().path())
        .map(|(key, data)| match key {
            Some(key) => res.header(CONTENT_TYPE, key).easy_body(data.clone()),
            None => res.easy_body(data.clone()),
        })
        .ok_or_else(|| tracing::debug!("target file is none"))
}

static FILES_BUF: LazyLock<HashMap<String, (Option<HeaderValue>, Bytes)>> = LazyLock::new(|| {
    let file_map = files_to_hashmap("public").unwrap();
    file_map
        .into_iter()
        .map(|(mut key, value)| {
            key.insert(0, '/');
            if let Some("js") = key.rsplit(".").next() {
                (key, (Some(APPLICATION_JS), value))
            } else {
                (key, (None, value))
            }
        })
        .collect()
});

fn files_to_hashmap<P: AsRef<Path>>(dir_path: P) -> io::Result<HashMap<String, Bytes>> {
    let mut file_map: HashMap<String, Bytes> = HashMap::new();

    // 读取目录
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        // 获取文件名
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => continue,
        };

        if path.is_file() {
            // 如果是文件，直接读取内容
            let mut file = fs::File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let bytes = Bytes::from(buffer);

            file_map.insert(file_name, bytes);
        } else if path.is_dir() {
            // 如果是目录，递归处理
            let sub_map = files_to_hashmap(&path)?;
            // 将子目录的文件名加上路径前缀，避免命名冲突
            for (sub_name, bytes) in sub_map {
                let full_name = format!("{}/{}", file_name, sub_name);
                file_map.insert(full_name, bytes);
            }
        }
    }

    Ok(file_map)
}

#[test]
fn test() {
    let map = files_to_hashmap("public").unwrap();
    println!("{:#?}", map.keys());
}
