use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    size: i32,
}
pub fn get_repo_size(proj_owner: &str, proj_name: &str) -> Result<usize, ureq::Error> {
    let resp: Response = ureq::get(&format!(
        "https://api.github.com/repos/{}/{}",
        &proj_owner, &proj_name
    ))
    .call()?
    .into_json()?;
    Ok(resp.size as usize)
}
