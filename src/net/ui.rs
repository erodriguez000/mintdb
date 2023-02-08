use warp::Filter;

pub fn config() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("src/ui/index.html"))
        .or(warp::fs::dir("src/ui/"))
}