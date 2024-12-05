// this file is not used anymore.
// keep it for reference only.

use salvo::{fs::NamedFile, prelude::*, Router};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::path::Path;
use std::{fs, io::Read};

use crate::error::{ServiceError, ServiceResult};

use super::utils::get_req_path;

pub fn router() -> Router {
    Router::new()
        .push(Router::with_path("version").get(last_version_handler))
        .push(Router::with_path("download/<version>").get(download_handler))
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenApiLastVersionResp {
    version: String,
    checksum: String,
}

impl Scribe for OpenApiLastVersionResp {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

#[derive(Debug, Deserialize)]
enum Platform {
    APK,
    Windows,
}

impl Platform {
    fn platform_file_name(&self) -> &'static str {
        match *self {
            Platform::APK => "xbb.apk",
            Platform::Windows => "xbb_desktop_windows_setup.exe",
        }
    }
}

#[handler]
async fn last_version_handler(
    req: &Request,
    _res: &mut Response,
) -> ServiceResult<OpenApiLastVersionResp> {
    let asset_dir = "assets";
    let Ok(dirs) = fs::read_dir(asset_dir) else {
        return Err(ServiceError::InternalServerError(
            "read dir failed".to_owned(),
        ));
    };

    let Some(platform): Option<Platform> = req.query("platform") else {
        return Err(ServiceError::BadRequest(
            "need query param `platform` = APK|Windows ".to_owned(),
        ));
    };
    let file_name = platform.platform_file_name();

    let mut versions: Vec<String> = dirs
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    return Some(path.file_name()?.to_str()?.to_string());
                }
            }
            None
        })
        .collect();

    versions.sort_by(|a, b| compare_versions(b, a));

    if let Some(latest_version) = versions.get(0) {
        let file_path = format!("assets/{}/{}", latest_version, file_name);
        let sha256checksum = calculate_sha256_checksum(&file_path)?;
        Ok(OpenApiLastVersionResp {
            version: latest_version.clone(),
            checksum: sha256checksum,
        })
    } else {
        Err(ServiceError::NotFound("no vesrion".to_owned()))
    }
}

fn calculate_sha256_checksum(file_path: &str) -> ServiceResult<String> {
    let mut file =
        fs::File::open(file_path).map_err(|_err| ServiceError::NotFound(file_path.to_string()))?;
    let mut hasher = Sha256::new();

    // 读取文件并计算 SHA-256 校验和
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;
    hasher.update(&buffer);

    // 获取最终的 SHA-256 校验和
    let result = hasher.finalize();

    // 将结果转为十六进制字符串并返回
    Ok(format!("{:x}", result))
}

// 根据版本号进行排序
fn compare_versions(a: &str, b: &str) -> Ordering {
    let parse_version = |s: &str| {
        s.split('.')
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .collect::<Vec<u32>>()
    };
    parse_version(a).cmp(&parse_version(b))
}

#[handler]
async fn download_handler(req: &mut Request, res: &mut Response) -> ServiceResult<()> {
    let version = get_req_path(req, "version")?;
    let Some(platform): Option<Platform> = req.query("platform") else {
        return Err(ServiceError::BadRequest(
            "need query param `platform` = APK|Windows ".to_owned(),
        ));
    };
    let file_name = platform.platform_file_name();
    let file_path = format!("assets/{}/{}", version, file_name);

    if Path::new(&file_path).exists() {
        NamedFile::builder(&file_path)
            .attached_name(file_name)
            .send(req.headers(), res)
            .await;
    } else {
        return Err(ServiceError::NotFound("no target version file".to_owned()));
    }

    Ok(())
}
